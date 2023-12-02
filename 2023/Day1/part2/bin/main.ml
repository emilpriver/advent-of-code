open String

let get_numbers_from_string (str: string) : int array =
  let nums = ref [] in
  let matches = ["zero"; "one"; "two"; "three"; "four"; "five"; "six"; "seven"; "eight"; "nine"] in

  for i = 0 to String.length str - 1 do
    match str.[i] with
    | '0'..'9' ->
      nums := int_of_char str.[i] - int_of_char '0' :: !nums
    | _ ->
      let len = String.length str in
      let word = String.sub str i (len - i) in
      for i = 0 to List.length matches - 1 do
        let match_word = List.nth matches i in
        if starts_with ~prefix:match_word word then
          nums := i - 0 :: !nums
      done
  done;

  Array.of_list (List.rev !nums)

let () =
  try 
    let chan = open_in "input.txt" in
    let results = ref [] in
    try
      while true do
        let line = input_line chan in
        let numbers = get_numbers_from_string line in
        let res = match Array.length numbers with
          | len when len >= 2 ->
              let first = Array.get numbers 0 in
              let last = Array.get numbers (len - 1) in
              Printf.sprintf "%d%d" first last
          | len when len = 1 ->
              let first = Array.get numbers 0 in
              Printf.sprintf "%d%d" first first
          | _ -> "0" in
        let res_as_number = int_of_string res in
        results := res_as_number :: !results
      done
    with
    | End_of_file -> 
      let result_array = Array.of_list (List.rev !results) in
      let sum = ref 0 in
      for i = 0 to Array.length result_array - 1 do
        sum := !sum + Array.get result_array i
      done;
      print_int !sum;
      close_in chan
  with
  | Sys_error msg ->
    Printf.printf "Error opening file: %s\n" msg
