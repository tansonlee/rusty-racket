(include stdlib::list)

(define (list-sum lst)
    (cond
        [(empty? lst) 0]
        [true (+ (car lst) (list-sum (cdr lst)))]))

(define (main) (list-sum (list::create 0 2000)))
