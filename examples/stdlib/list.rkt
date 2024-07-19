(include stdlib::num)

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

(define (list::nth lst n)
    (cond
        [(= n 0) (car lst)]
        [true (list::nth (cdr lst) (- n 1))]))

(define (list::take lst n)
    (cond
        [(= n 0) empty]
        [true (cons (car lst) (list::take (cdr lst) (- n 1)))]))

(define (list::drop lst n)
    (cond
        [(= n 0) lst]
        [true (list::drop (cdr lst) (- n 1))]))

(define (__list::min-helper lst min)
    (cond
        [(empty? lst) min]
        [true (__list::min-helper (cdr lst) (num::min min (car lst)))]))

(define (list::min lst) (__list::min-helper lst (car lst)))

(define (__list::max-helper lst max)
    (cond
        [(empty? lst) max]
        [true (__list::max-helper (cdr lst) (num::max max (car lst)))]))

(define (list::max lst) (__list::max-helper lst (car lst)))

(define (__list::sorted-merge-helper lst1 lst2 acc)
    (cond
        [(empty? lst1) (list::append (list::reverse acc) lst2)]
        [(empty? lst2) (list::append (list::reverse acc) lst1)]
        [(< (car lst1) (car lst2)) (__list::sorted-merge-helper (cdr lst1) lst2 (cons (car lst1) acc))]
        [true (__list::sorted-merge-helper lst1 (cdr lst2) (cons (car lst2) acc))]))

(define (__list::sorted-merge lst1 lst2) (__list::sorted-merge-helper lst1 lst2 empty))

(define (list::sort lst)
    (cond
        [(| (empty? lst) (empty? (cdr lst))) lst]
        [(empty? (cdr (cdr lst))) 
         (__list::sorted-merge (list (car lst)) (cdr lst))]
        [true 
         (__list::sorted-merge (list::sort (list::take lst (/ (list::length lst) 2)))
                               (list::sort (list::drop lst (/ (list::length lst) 2))))]))

(define (list::create start end)
    (cond
        [(> start end) empty]
        [true (cons start (list::create (+ start 1) end))]))
        