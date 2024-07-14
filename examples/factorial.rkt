(define (factorial n)
    (cond
        [(= n 1) 1]
        [true (* n (factorial (- n 1)))]))

(define (main) (+ (factorial 1) (+  (factorial 2) (factorial 10))))