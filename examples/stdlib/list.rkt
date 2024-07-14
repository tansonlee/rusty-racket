(define (list::length lst)
    (cond
        [(empty? lst) 0]
        [true (+ 1 (list::length (cdr lst)))]))

(define (__list::list-reverse-helper lst acc)
    (cond
        [(empty? lst) acc]
        [true (__list::list-reverse-helper (cdr lst) (cons (car lst) acc))]))

(define (list::reverse lst) (__list::list-reverse-helper lst empty))


(define (list::contains lst val)
    (cond
        [(empty? lst) false]
        [true (cond
                [(= val (car lst)) true]
                [true (list::contains (cdr lst))])])) 

(define (list::append lst1 lst2)
    (cond
        [(empty? lst1) lst2]
        [true (cons (car lst1) (list::append (cdr lst1) lst2))])) 