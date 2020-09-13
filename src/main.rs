mod ast;
mod parser;
mod tokenizer;

fn main() -> anyhow::Result<()> {
  let code = String::from(
    r#"
let Digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";
let OneDigitPlus = {
    "00": [false, "0"];
    "01": [false, "1"];
    "02": [false, "2"];
    "03": [false, "3"];
    "04": [false, "4"];
    "05": [false, "5"];
    "06": [false, "6"];
    "07": [false, "7"];
    "08": [false, "8"];
    "09": [false, "9"];
    "10": [false, "1"];
    "11": [false, "2"];
    "12": [false, "3"];
    "13": [false, "4"];
    "14": [false, "5"];
    "15": [false, "6"];
    "16": [false, "7"];
    "17": [false, "8"];
    "18": [false, "9"];
    "19": [true, "0"];
    "20": [false, "2"];
    "21": [false, "3"];
    "22": [false, "4"];
    "23": [false, "5"];
    "24": [false, "6"];
    "25": [false, "7"];
    "26": [false, "8"];
    "27": [false, "9"];
    "28": [true, "0"];
    "29": [true, "1"];
    "30": [false, "3"];
    "31": [false, "4"];
    "32": [false, "5"];
    "33": [false, "6"];
    "34": [false, "7"];
    "35": [false, "8"];
    "36": [false, "9"];
    "37": [true, "0"];
    "38": [true, "1"];
    "39": [true, "2"];
    "40": [false, "4"];
    "41": [false, "5"];
    "42": [false, "6"];
    "43": [false, "7"];
    "44": [false, "8"];
    "45": [false, "9"];
    "46": [true, "0"];
    "47": [true, "1"];
    "48": [true, "2"];
    "49": [true, "3"];
    "50": [false, "5"];
    "51": [false, "6"];
    "52": [false, "7"];
    "53": [false, "8"];
    "54": [false, "9"];
    "55": [true, "0"];
    "56": [true, "1"];
    "57": [true, "2"];
    "58": [true, "3"];
    "59": [true, "4"];
    "60": [false, "6"];
    "61": [false, "7"];
    "62": [false, "8"];
    "63": [false, "9"];
    "64": [true, "0"];
    "65": [true, "1"];
    "66": [true, "2"];
    "67": [true, "3"];
    "68": [true, "4"];
    "69": [true, "5"];
    "70": [false, "7"];
    "71": [false, "8"];
    "72": [false, "9"];
    "73": [true, "0"];
    "74": [true, "1"];
    "75": [true, "2"];
    "76": [true, "3"];
    "77": [true, "4"];
    "78": [true, "5"];
    "79": [true, "6"];
    "80": [false, "8"];
    "81": [false, "9"];
    "82": [true, "0"];
    "83": [true, "1"];
    "84": [true, "2"];
    "85": [true, "3"];
    "86": [true, "4"];
    "87": [true, "5"];
    "88": [true, "6"];
    "89": [true, "7"];
    "90": [false, "9"];
    "91": [true, "0"];
    "92": [true, "1"];
    "93": [true, "2"];
    "94": [true, "3"];
    "95": [true, "4"];
    "96": [true, "5"];
    "97": [true, "6"];
    "98": [true, "7"];
    "99": [true, "8"];
};
fn LastDigit Str =
  distribute Digit in
    if-match `[Rest]{Digit}` then
      [Rest, Digit]
    else
      never
;
fn ConcatStr Left Right =
  `{Left}{Right}`
;
"#,
  );
  let (_, tokens) = tokenizer::tokenize(&code).map_err(|err| err.to_owned())?;
  // println!("{:?}", tokens);
  let (_, ast) =
    parser::parse(&tokens).map_err(|err| err.map_input(|tokens| format!("{:?}", tokens)))?;
  println!("{:?}", ast);
  Ok(())
}
