(local { : split : map : reduce : filter } (require :lume))

(fn not-empty? [str] (not (= str "")))
(fn sum [coll] (reduce coll (lambda [acc val] (+ acc val) ) 0))

(fn read-file [path]
  (let [f (assert (io.open path))
        contents (f:read "*a")]
    (f:close)
    contents))

(fn parse-their-hand [hand] 
  (if (= hand "A") :rock
      (= hand "B") :paper
      (= hand "C") :scissors))

(fn parse-our-hand [hand] 
  (if (= hand "X") :rock
      (= hand "Y") :paper
      (= hand "Z") :scissors))

(fn parse-result [result]
  (if (= result "X") :lose
      (= result "Y") :tie
      (= result "Z") :win))


(fn get-result [round] 
  (let [{: us : them } round]
    (if (= us :rock)
          (if (= them :scissors) :win 
              (= them :rock)     :tie 
              (= them :paper)    :lose)
        (= us :paper)
          (if (= them :rock)     :win
              (= them :paper)    :tie
              (= them :scissors) :lose)
        (= us :scissors)
          (if (= them :paper)    :win 
              (= them :scissors) :tie 
              (= them :rock)     :lose))))

(fn score-result [result]
  (if (= result :win)  6
      (= result :tie)  3
      (= result :lose) 0))

(fn score-hand [hand]
  (if (= hand :rock)     1
      (= hand :paper)    2
      (= hand :scissors) 3))

(fn force-result [them result]
  (if (= them :rock)
        (if (= result :win)  :paper
            (= result :tie)  :rock
            (= result :lose) :scissors)
      (= them :paper)
        (if (= result :win)  :scissors
            (= result :tie)  :paper
            (= result :lose) :rock)
      (= them :scissors)
        (if (= result :win)  :rock
            (= result :tie)  :scissors
            (= result :lose) :paper)))

(fn parse-round [round] 
  (let [[them us] (split round " ")]
    { :them (parse-their-hand them) :us (parse-our-hand us) }))

(fn parse-partial-round [round]
  (let [[them-str result-str] (split round " ")
        them (parse-their-hand them-str)
        result (parse-result result-str)]
    { : them  :us (force-result them result) }))

(fn parse-input [input]
  "Parse the input string into rounds of the form {:us <hand> :them <hand>},
  where <hand> is either :rock, :paper, or :scissors"
  (let [rounds (filter (split input "\n") not-empty?)]
    (map rounds parse-round)))

(fn parse-input-2 [input]
  (let [rounds (filter (split input "\n") not-empty?)]
    (map rounds parse-partial-round)))

(fn score-round [round]
  (let [{ : us } round]
    (+ (score-hand us) (score-result (get-result round)))))

(fn part1 [input] 
  (sum (map (parse-input input) score-round)))

(fn part2 [input] 
  (sum (map (parse-input-2 input) score-round)))



(comment
  (part1 (read-file "./test-input.txt")) ; 15
  (part1 (read-file "./input.txt")) ; 8392
  (part2 (read-file "./test-input.txt")) ; 12
  (part2 (read-file "./input.txt")) ; 10116
  (score-round (parse-round "A X"))
  (parse-result "X")
  (force-result :rock :win)
  (force-result :rock :lose)
  (parse-partial-round "A X")
  (parse-input-2 (read-file "./test-input.txt"))
  (parse-input (read-file "./test-input.txt")))

