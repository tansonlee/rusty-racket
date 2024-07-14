(define (list-sum lst)
    (cond
        [(empty? lst) 0]
        [true (+ (car lst) (list-sum (cdr lst)))]))

(define (main) (list-sum (list 1 2 3 4 5)))