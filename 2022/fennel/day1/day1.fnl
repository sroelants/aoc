(local lume (require :lume))

;; Come on, lua...
(fn sum [coll] (lume.reduce coll (lambda [acc val] (+ acc val) ) 0))

(fn read-file [path]
  (let [f (assert (io.open path))
        contents (f:read "*a")]
    (f:close)
    contents))

(fn parse-input [input] 
  "Take the input string and return an array of arrays of calories"
  (let [groups (lume.split input "\n\n")]
    (lume.map groups (lambda [group] (lume.map (lume.split group "\n") tonumber)))))

(fn total-calories [elf]
  "Calculate the total amount of calories for a given elf"
  (sum elf))

(fn part1 [input] 
  (let [elves (parse-input input)
        calories (lume.map elves total-calories)]
    (math.max (unpack calories)))) ; #<function: 0x7efcd20a78f8>

(fn part2 [input]
  (let [elves (parse-input input)
        calories (lume.map elves total-calories)
        sorted-calories (lume.sort calories (lambda [a b] (> a b)))]
    (sum (lume.first sorted-calories 3))))


(part1 (read-file "./test-input.txt")) ; 24000
(part2 (read-file "./test-input.txt")) ; 45000

(part1 (read-file "./input.txt")) ; 64454
(part2 (read-file "./input.txt")) ; 195292
