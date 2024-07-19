(include stdlib::list)
(include stdlib::bst)


(define (main) (bst::depth (bst::from-list (list::create 0 1000))))
