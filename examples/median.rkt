(define (median a b c)
    (cond
        [(| (& (< a b) (< b c)) (& (< c b) (< b a))) b]
        [(| (& (< b a) (< a c)) (& (< c a) (< a b))) a]
        [(| (& (< a c) (< c b)) (& (< b c) (< c a))) c]))
(define (main) (median 3 1 2))