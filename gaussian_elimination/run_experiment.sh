#!/bin/bash
for t in {1,2,4,8,16,32,36,64,72}; 
    do for n in {128,256,512,1024,2048};
        do for e in {1,2,3,4,5};
            do echo $t $n;
            cargo run $n $t >> results.csv;
        done;
    done;
done
