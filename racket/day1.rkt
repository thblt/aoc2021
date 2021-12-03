#lang racket

(define (fun)
  (define (fun previous count)
    (let ((n (read-line (current-input-port))))
      (if (eof-object? n)
          count
          (let ((n (string->number (string-trim n))))
            (fun n (if (and previous
                            (< previous n))
                       (add1 count) count))))))
  (fun #f 0))

(fun)
