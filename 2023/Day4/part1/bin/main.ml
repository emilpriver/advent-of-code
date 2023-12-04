let rec get_common_ints list1 list2 =
  List.filter (fun a -> List.mem list1 a) list2

let line_to_list  str =
  let items = String.split_on_char ' ' str in
  let non_empty = List.filter (fun s -> String.length (String.trim s) > 0) items in
  List.map (fun s -> print_string s; print_string " "; int_of_string (String.trim s)) non_empty

let line_into_lists str =
  let remove_first_part = String.split_on_char ':' str in
  let last_item = List.nth remove_first_part 1 in
  let lines = String.split_on_char '|' (String.trim last_item) in
  let lines_map = List.map line_to_list lines in
  lines_map

let process_line (index, acc) str =
  let lines = line_into_lists str in
  let first_line = List.hd lines in
  let second_line = List.nth lines 1 in
  let winning_numbers = get_common_ints first_line second_line in
  List.iter (fun i -> print_int i; print_string " ") winning_numbers;
  print_newline();
  (index + 1, acc)

let () =
  let chan = open_in "input.txt" in
  let (_, res) = In_channel.fold_lines process_line (1, 0) chan in
  Printf.printf "%d\n" res
