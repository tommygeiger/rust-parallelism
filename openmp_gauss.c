/* 
 * Original author:  UNKNOWN
 *
 * Modified:         Kai Shen (January 2010)
 *                   Ben Reber (Feb 2021)
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/time.h>
#include <math.h>
#include <assert.h>
#include <omp.h>


#define SWAP(a,b)       {double tmp; tmp = a; a = b; b = tmp;}

/* Solve the equation:
 *   matrix * X = R
 */

double **matrix, *X, *R;

/* Pre-set solution. */

double *X__;
int NSIZE = 0;

void printMatrix()
{
    printf("Hello from printMatrix %d\n",NSIZE);
    for(int i = 0;i<NSIZE;i++)
    {
        for(int j = 0;j<NSIZE;j++)
        {
            printf("%f ",matrix[i][j]);
        }
    printf("\n");
    }

}

void printVector(double *vec){
    for(int i = 0; i<NSIZE; i++){
        printf("%f ",vec[i]);
    }
    printf("\n");
}

/* Initialize the matirx. */
void initMatrix(){
    double *tmp;
    int i,j;
    /* Initialize the space and set all elements to zero. */
    matrix = (double**)malloc(NSIZE*sizeof(double*));
    assert(matrix != NULL);
    tmp = (double*)malloc(NSIZE*NSIZE*sizeof(double));
    assert(tmp != NULL);    
    for (i = 0; i < NSIZE; i++) {
        matrix[i] = tmp;
        tmp = tmp + NSIZE;
    }
    for (i = 0; i < NSIZE; i++) {
        for (j = 0; j < NSIZE; j++) {
			//initialize to random float between 1 and 10
            matrix[i][j] =rand()/(float)(RAND_MAX/10);
            //
            //matrix[i][j] = 10.0 *  (float) (i+1) +  (float) (j+1);
        }
    }
}

void swapRows(int pivotrow, int currow){
    double *temp;
    temp = matrix[pivotrow];
    matrix[pivotrow]=matrix[currow];
    matrix[currow]=temp;

}
/* Initialize the right-hand-side following the pre-set solution. */

void initRHS()
{
    int i, j;

    X__ = (double*)malloc(NSIZE * sizeof(double));
    assert(X__ != NULL);
    for (i = 0; i < NSIZE; i++) {
	    X__[i] = i+1;
    }

    R = (double*)malloc(NSIZE * sizeof(double));
    assert(R != NULL);
    for (i = 0; i < NSIZE; i++) {
	    R[i] = 0.0;
        for (j = 0; j < NSIZE; j++) {
            R[i] += matrix[i][j] * X__[j];
        }
    }
#ifdef DEBUG
    printf("Initial R: ");
    printVector(R);
#endif
}

/* Initialize the results. */

void initResult()
{
    int i;

    X = (double*)malloc(NSIZE * sizeof(double));
    assert(X != NULL);
    for (i = 0; i < NSIZE; i++) {
	    X[i] = 0.0;
    }
}

/* Get the pivot - the element on column with largest absolute value. */

void getPivot(int currow)
{
    int i, pivotrow;

    pivotrow = currow;
    for (i = currow+1; i < NSIZE; i++) {
	    if (fabs(matrix[i][currow]) > fabs(matrix[pivotrow][currow])) {
	        pivotrow = i;
	    }
    }

    if (fabs(matrix[pivotrow][currow]) == 0.0) {
        fprintf(stderr, "The matrix is singular\n");
        exit(-1);
    }
    
    if (pivotrow != currow) {
#ifdef DEBUG
	fprintf(stdout, "\npivot row at step %5d is %5d", currow, pivotrow);
    printVector(matrix[pivotrow]);


#endif
        swapRows(pivotrow,currow);
        SWAP(R[pivotrow],R[currow]);
    }
}


void computeGauss()
{

    int i, j, k;
    double pivotval;

    for (i = 0; i < NSIZE; i++) {
#ifdef DEBUG
        printMatrix();
        printf("R: ");
        printVector(R);
        printf("--------------\n");
        fflush(stdout);
#endif
        getPivot(i);
             
        /* Scale the maain row. */
        pivotval = matrix[i][i];
        if (pivotval != 1.0) {
            matrix[i][i] = 1.0;
            for (j = i + 1; j < NSIZE; j++) {
                matrix[i][j] /= pivotval;
            }
            R[i] /= pivotval;
        }
         
#pragma omp parallel private(k,pivotval)
#pragma omp for schedule(static)  
        for (j = i + 1; j < NSIZE; j++) {
            //printf("Thread %d. row %d. iter %d pivotrow:  %f %f %f %f\n",omp_get_thread_num(),j,i,matrix[i][0],matrix[i][1],matrix[i][2],matrix[i][3]);
            pivotval = matrix[j][i];
            matrix[j][i] = 0.0;
            for (k = i + 1; k < NSIZE; k++) {
                matrix[j][k] -= pivotval * matrix[i][k];
            }
            R[j] -= pivotval * R[i];
        }
    }
#ifdef DEBUG
    printMatrix();
#endif
}

/* Solve the equation. */

void solveGauss()
{
    int i, j;

    X[NSIZE-1] = R[NSIZE-1];
    for (i = NSIZE - 2; i >= 0; i --) {
        X[i] = R[i];
        for (j = NSIZE - 1; j > i; j--) {
            X[i] -= matrix[i][j] * X[j];
        }
    }

#ifdef DEBUG
    fprintf(stdout, "X = [");
    for (i = 0; i < NSIZE; i++) {
        fprintf(stdout, "%.6f ", X[i]);
    }
    fprintf(stdout, "];\n");
#endif
}

int main(int argc, char *argv[])
{
    int i, c;
    double inittime, reftime, verifytime, error;
    struct timeval start, initdone, refdone, finish;

    while ((c = getopt (argc, argv, "t:n:")) != -1)
        {
        switch (c) {
        case 'n':
            NSIZE = atoi(optarg);
            break;
        case 't':
            printf("Warning: havent added logic for thread num\n");
            //num_threads = atoi(optarg);
            break;
        }
    } 
                
    gettimeofday(&start, 0);

    initMatrix();
    initRHS();
    initResult();

    gettimeofday(&initdone, 0);
    computeGauss();
        
    gettimeofday(&refdone, 0);
    solveGauss();
    error = 0.0;
    for (i = 0; i < NSIZE; i++) {
	    double error__ = (X__[i]==0.0) ? 1.0 : fabs((X[i]-X__[i])/X__[i]);
	    if (error < error__) {
	        error = error__;
	    }
    }

    gettimeofday(&finish, 0);
    inittime = (initdone.tv_sec - start.tv_sec) + (initdone.tv_usec - start.tv_usec)*0.000001;
    reftime = (refdone.tv_sec - initdone.tv_sec) + (refdone.tv_usec - initdone.tv_usec)*0.000001;
    verifytime = (finish.tv_sec - refdone.tv_sec) + (finish.tv_usec - refdone.tv_usec)*0.000001;
    printf("%d,%d,%f,%f,%f,%f\n",NSIZE,omp_get_num_threads(),inittime,reftime,verifytime,error);

    //delay loop to ensure prints happen
    float a;
    a=1.0;
    for(int i=0;i<100000;i++){
        a/= (float) i; 
    }
    printf("%f\n",a);
#ifdef DEBUG
    printf("X__: ");
    printVector(X__);
    printf("X: ");
    printVector(X);
#endif

    return 0;
}
