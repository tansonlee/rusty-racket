(define (fibonacci n)
    (cond
        [(= n 0) 0]
        [(= n 1) 1]
        [true (+ (fibonacci (- n 1)) (fibonacci (- n 2)))]))

(define (main) (+ (fibonacci 0) (+ (fibonacci 1) (+ (fibonacci 5) (fibonacci 10)))))