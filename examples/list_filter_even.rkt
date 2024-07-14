(define (list-filter-even lst)
    (cond
        [(empty? lst) empty]
        [true (cond
                [(= (% (car lst) 2) 0) (list-filter-even (cdr lst))]
                [true (cons (car lst) (list-filter-even (cdr lst)))])])) 

(define (main) (list-filter-even (list 1 2 3 4 5 6 7 8)))