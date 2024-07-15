(define (find-divisor n test-divisor)
    (cond
        [(> (* test-divisor test-divisor) n) false]
        [(= 0 (% n test-divisor)) true]
        [true (find-divisor n (+ test-divisor 1))]))

(define (prime? n)
    (cond
        [(< n 2) false]
        [(= n 2) true]
        [true (! (find-divisor n 2))]))

(define (primes-until-helper n curr)
    (cond
        [(= curr n) empty]
        [(prime? curr) (cons curr (primes-until-helper n (+ curr 1)))]
        [true (primes-until-helper n (+ curr 1))]))

(define (primes-until n) (primes-until-helper n 2))

(define (main) (primes-until 100))