(include stdlib::binary-tree)
(include stdlib::num)
(include stdlib::list)

(define (bst::empty) (binary-tree::empty))

(define (bst::empty? bst) (binary-tree::empty? bst))

(define (bst::create) empty)

(define (__bst::from-list-helper lst)
    (cond
        [(bst::empty? lst) (bst::empty)]
        [true (bst::insert (__bst::from-list-helper (cdr lst)) (car lst))]))

(define (__bst::balanced-insertion-order lst)
    (cond
        [(empty? lst) empty]
        [true (cons (list::nth lst (/ (list::length lst) 2))
                    (list::append 
                        (__bst::balanced-insertion-order (list::take lst (/ (list::length lst) 2)))
                        (__bst::balanced-insertion-order (list::drop lst (+ 1 (/ (list::length lst) 2))))))]))

(define (bst::from-list lst)
    (__bst::from-list-helper
        (list::reverse
            (__bst::balanced-insertion-order
                (list::sort lst)))))

(define (bst::insert bst value)
    (cond
        [(bst::empty? bst) (binary-tree::create value (bst::empty) (bst::empty))]
        [(< value (binary-tree::get-value bst))
            (binary-tree::create (binary-tree::get-value bst)
                          (bst::insert (binary-tree::get-left bst) value)
                          (binary-tree::get-right bst))]
        [(> value (binary-tree::get-value bst))
            (binary-tree::create (binary-tree::get-value bst)
                          (binary-tree::get-left bst)
                          (bst::insert (binary-tree::get-right bst) value))]
        [true bst]))

(define (bst::delete bst value)
    (cond
        [(bst::empty? bst) bst]
        [(< value (binary-tree::get-value bst))
            (binary-tree::create (binary-tree::get-value bst)
                          (bst::delete (binary-tree::get-left bst) value)
                          (binary-tree::get-right bst))]
        [(> value (binary-tree::get-value bst))
            (binary-tree::create (binary-tree::get-value bst)
                          (binary-tree::get-left bst)
                          (bst::delete (binary-tree::get-right bst) value))]
        [true
            (cond
                [(& (bst::empty? (binary-tree::get-left bst)) (bst::empty? (binary-tree::get-right bst))) (bst::empty)]
                [(bst::empty? (binary-tree::get-left bst)) (binary-tree::get-right bst)]
                [(bst::empty? (binary-tree::get-right bst)) (binary-tree::get-left bst)]
                [true
                    (binary-tree::create
                        (bst::min (binary-tree::get-right bst))
                        (binary-tree::get-left bst)
                        (bst::delete (binary-tree::get-right bst) (bst::min (binary-tree::get-right bst))))])]))

(define (bst::contains? bst value)
    (cond
        [(bst::emtpy? bst) false]
        [(= value (binary-tree::get-value bst)) true]
        [(< value (binary-tree::get-value bst)) (bst::contains? (binary-tree::get-left bst) value)]
        [(> value (binary-tree::get-value bst)) (bst::contains? (binary-tree::get-right bst) value)]))

(define (bst::depth bst)
    (cond
        [(bst::empty? bst) 0]
        [true (+ 1 (num::max 
                    (bst::depth (binary-tree::get-left bst))
                    (bst::depth (binary-tree::get-right bst))))]))

(define (bst::size bst)
    (cond
        [(bst::empty? bst) 0]
        [true (+ 1 (+ 
                    (bst::size (binary-tree::get-left bst))
                    (bst::size (binary-tree::get-right bst))))]))

(define (bst::min bst)
    (cond
        [(bst::empty? bst) 0]
        [(bst::empty? (binary-tree::get-left bst)) (binary-tree::get-value bst)]
        [true (bst::min (binary-tree::get-left bst))]))

(define (bst::max bst)
    (cond
        [(bst::empty? bst) 0]
        [(bst::empty? (binary-tree::get-right bst)) (binary-tree::get-value bst)]
        [true (bst::min (binary-tree::get-right bst))]))