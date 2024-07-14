(include stdlib::list)

(define (list-flatten lst)
    (cond
        [(empty? lst) empty]
        [(list? (car lst)) (list::append (list-flatten (car lst)) (list-flatten (cdr lst)))]
        [true (cons (car lst) (list-flatten (cdr lst)))])) 

(define (main) (list-flatten (list (list 1 2 (list 3 (list 4 5) 6 7)) 8 9)))