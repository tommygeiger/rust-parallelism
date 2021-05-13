package main
 
import (
		"fmt"
		"log"
		"math"
		"math/rand"
		"sync"
		"flag"
		"time"
)

type Matrix struct {
		a [][]float64
		b []float64
		x []float64
}

var m int // matrix dimensions
var matrix Matrix // matrix
var aug [][]float64 // augmented matrix
var x []float64 // solution matrix
var g int // number of worker threads
const ε = 1e-6 // tolerance

// Synchronization globals
var channel chan int
var wg sync.WaitGroup
var mutex = sync.Mutex{}
var c = sync.NewCond(&mutex)


func main() {

		// Get flags
		gPtr := flag.Int("g", 1, "number of workers")
		mPtr := flag.Int("m", 128, "matrix dimensions")
		flag.Parse()
		g = *gPtr
		m = *mPtr

		initializeMatrix()
		
		// Init solution matrix and augmented matrix
		x = make([]float64, m)
		aug = make([][]float64, m)
		for i, ai := range matrix.a {
			row := make([]float64, m+1)
			copy(row, ai)
			row[m] = matrix.b[i]
			aug[i] = row
		}

		// Init buffered channel
		channel = make(chan int, g-1)


		/*----*/ start := time.Now() /*----*/

		// Spawn goroutines for elimination step
		for id := 0; id < g; id++ {
			wg.Add(1)
			go doEliminate(id)
		}
		wg.Wait()

		/*----*/ end := time.Now() /*----*/
		elapsed := end.Sub(start)


		// Back substitution
		for i := m - 1; i >= 0; i-- {
				x[i] = aug[i][m]
				for j := i + 1; j < m; j++ {
						x[i] -= aug[i][j] * x[j]
				}
				x[i] /= aug[i][i]
		}

		// Verify solution
		fmt.Printf("%v,%v,%v\n",m,g,elapsed)
		for i, xi := range x {
				if math.Abs(matrix.x[i] - xi) > ε {
						fmt.Println("Calculated:", x)
						fmt.Println("Expected:", matrix.x)
						log.Fatal("Out of tolerance")
				}
		}
}


// Perform elimination step
func doEliminate(id int) {
	  defer wg.Done()

		for k := range aug {

			// Partial pivoting once
			if id == 0 {
				doPivot(k)
			}

			Barrier(id)

			// Elimination step
			for i := k + 1 + id; i < m; i += g {
				for j := k + 1; j <= m; j++ {
					aug[i][j] -= aug[k][j] * (aug[i][k] / aug[k][k])
				}
				aug[i][k] = 0
			}

			Barrier(id)
		}
}


// Perform partial pivoting
func doPivot(k int) {
	iMax := 0
	max := -1.

	for i := k; i < m; i++ {
			row := aug[i]

			// Compute scale factor s = max abs in row
			s := -1.
			for j := k; j < m; j++ {
					rowAbs := math.Abs(row[j])
					if rowAbs > s {
							s = rowAbs
					}
			}

			// Scale the abs used to pick the pivot
			if abs := math.Abs(row[k]) / s; abs > max {
					iMax = i
					max = abs
			}
	}
	
	// Check for singular matrix
	if aug[iMax][k] == 0 {
			log.Fatal("Singular")
	}
	
	// Swap rows
	aug[k], aug[iMax] = aug[iMax], aug[k]
}


// Barrier implementation, syncs all goroutines
func Barrier(id int) {

	// Master thread waits for all channels, then broadcasts
	if id == 0 {

		// Read all values from channel, then broadcast
		for i := 0; i < g-1; i++ {
			<-channel
		}
		c.Broadcast()

	} else {

		// Write to channel and wait
		c.L.Lock()
		channel <- id
		c.Wait()
		c.L.Unlock()

	}
}


// Initialize m x m matrix with easily verifiable values
func initializeMatrix() {
	
	matrix.a = make([][]float64, m)
	matrix.b = make([]float64, m)
	matrix.x = make([]float64, m)

	for i := range matrix.a {
		matrix.a[i] = make([]float64, m) // augmented matrix
		for j := 0; j < m; j++ {
			matrix.a[i][j] = rand.Float64() * 10.
			matrix.b[i] += matrix.a[i][j] * float64(j+1)
		}
		matrix.x[i] = float64(i+1) // initialize x to [1, 2, 3...]
	}
}
