(define (binary-tree::empty) empty)

(define (binary-tree::empty? node) (empty? node))

(define (binary-tree::create val left right) (list val left right))

(define (binary-tree::get-value node) (car node))

(define (binary-tree::get-left node) (car (cdr node)))

(define (binary-tree::get-right node) (car (cdr (cdr node))))