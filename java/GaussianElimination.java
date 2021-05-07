public class GaussianElimination extends Thread{

    int size, id;
    double[][] system;
    int num_threads;

    public GaussianElimination(int size){
        if(size > 0){
                this.size = size;
                this.system =  new double[size][size+1];
        }
    }

    void print_matrix(){
        for (int i=0;i<this.size;i++){
            for (int j=0;j<=this.size;j++){
                System.out.print(this.system[i][j]);
            }

            System.out.print("\n");
        }

    }
    void init(){

        for (int i=0;i<this.size;i++){
            for (int j =0;j<this.size;j++){
                this.system[i][j]= Math.random()*10.0;
            }
        }

        for (int i=0;i<this.size;i++){
            for (int j =0;j<this.size;j++){
                this.system[i][this.size]+= new Double(j+1) * this.system[i][j] ;
            }
        }
    }

    void swap_rows(int a, int b){
        double[] tmp = new double[size+1];
        tmp = this.system[a];
        this.system[a]=this.system[b];
        this.system[b]=tmp;
    }
    double[] eliminate(){
        int pivotrow;
        double pivotval;
        double[] x = new double[size];

        System.out.println(this.size);
        for(int i=0;i<this.size;i++){

            pivotrow=i;
            for(int j=0;j<this.size-1;j++){
                if(Math.abs(this.system[j][i]) > Math.abs(this.system[pivotrow][i])) {
                    pivotrow=j;
                }
            }
            if(pivotrow != i){
                this.swap_rows(i,pivotrow);
            }

            //scale main row
            pivotval=this.system[i][i];
            if(pivotval!=0.0){
                this.system[i][i]= 1.0;
                for(int j=i+1;j<this.size+1;j++){
                    this.system[i][j]/=pivotval;
                }
            }

            //scale lower matrix
            for (int j=i+1;j<this.size;j++){
                double factor = this.system[j][i];
                this.system[j][i]= 0.0;
                for (int k=i+1;k<this.size+1;k++){
                    this.system[j][k] -= factor * this.system[i][k];
                }
            }
        }

        x[this.size-1] = this.system[this.size-1][this.size];
        for (int i = this.size - 2; i >= 0; i --) {
            x[i] = this.system[i][this.size];
            for (int j = this.size - 1; j > i; j--) {
                x[i] -= this.system[i][j] * x[j];
            }
        }
        return x;
    }

    public double verify(double[] solution) {
        
    double err   = 0.0;
    double err__;
    for (int i=0; i<this.size;i++){
        double actual =new  Double(i) + 1.0;
        err__ = Math.abs(solution[i] - actual)/ actual;
        if(err < err__){
            err = err__;
        }
    }
    return err;

    }
    public static void main(String[] args){
        GaussianElimination myge = new GaussianElimination(2048);
        myge.init();
        double[] x = myge.eliminate();
        System.out.println(myge.verify(x));
    }
}
