let process_json_object json_obj =
  Printf.printf "Processing JSON object: %s\n" (Yojson.Basic.to_string json_obj)

let process_json_line json_line =
  try
    let json_obj = Yojson.Basic.from_string json_line in
    process_json_object json_obj
  with
  | Yojson.Json_error msg ->
    Printf.printf "Error parsing JSON: %s\n" msg

let read_json_file filename =
  try
    let chan = open_in filename in
    try
      while true do
        let line = input_line chan in
        process_json_line line
      done
    with
    | End_of_file -> close_in chan
  with
  | Sys_error msg ->
    Printf.printf "Error opening file: %s\n" msg

let () =
  let filename = "input.txt" in
  read_json_file filename

