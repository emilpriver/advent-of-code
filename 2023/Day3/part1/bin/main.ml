let check_diagonal str =
  print_string str

let line_into_list str =
  let l = String.split_on_char '.' str in
  let list_without_space = List.filter (fun s -> String.length (String.trim s) > 0) l in
  List.map (fun s -> String.trim s) list_without_space

let remove_last l =
  let rec remove_last_helper acc = function
    | [] -> acc 
    | [_] -> acc (* only one element left, don't include it *)
    | h::t -> remove_last_helper (h::acc) t 
  in
  remove_last_helper [] l

let is_number s =
  try 
    let _ = int_of_string s in true 
  with Failure _ ->
    false

let filter_only_numbers l =
  let rec filter_numbers_helper acc = function
    | [] -> acc
    | h::t -> 
      if is_number h
      then filter_numbers_helper (h::acc) t
      else filter_numbers_helper acc t
  in
  filter_numbers_helper [] l

let strings_to_int_list strings =
  List.map int_of_string strings

let remove_dots str =
  List.filter (fun s -> (s <> ".")) str

let remove_empty_items str =
  List.filter (fun s -> print_string s; print_string " "; (String.length s > 0)) str

let process_line (index, acc) str =
  let list = line_into_list str in
  let list_no_space = remove_empty_items list in
  let list_no_dots = remove_dots list_no_space in
  let list = remove_last list_no_dots in
  let list_only_numbers = filter_only_numbers list in
  let list_numbers = strings_to_int_list list_only_numbers in
  let sum = List.fold_left (fun current i -> current + i) 0 list_numbers in
  print_int sum;
  print_newline();
  (index +1, acc + sum)

let () =
  let chan = open_in "input.txt" in
  let (_, res) = In_channel.fold_lines process_line (0,0) chan in
  Printf.printf "%d\n" res
