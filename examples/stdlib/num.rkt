(define (num::min a b)
    (cond
        [(< a b) a]
        [true b]))

(define (num::max a b)
    (cond
        [(> a b) a]
        [true b]))
