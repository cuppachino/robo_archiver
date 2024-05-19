# Robo Archiver

A quick and scrappy CLI tool for automating archival of historical documents.

## What does this do?

`robo_archiver` bulk processes periodical issues using their file name and MARC record into the CSV structure used by the [Arizona Memory Project](https://azmemory.azlibrary.gov/).
Apparently, a lot of archival is done by hand, and many of the fields are somewhat redundant in-context. The aim of this program is to streamline the process for archivists.

The program does **not** *currently* attempt to extract information (such as volume no., issue no., publisher, languages, or contributors) from the contents of the file.

> This could maybe be accomplished with help from a pdf-to-text utility and an AI API call or two, but for now I reckon the time to review outweighs the cost of entering that information manaully.

## Commands

Run the application with `-h` or `--help` for a full list of commands.

## Notes

### File name-date scheme

All files for the same periodical should begin with the same name and end with their date in partial `yyyy-mm-dd` format.

<details>
  <summary>Example files</summary>

  - `An_Arizona_Desert-ation_1967.pdf`
  - `An_Arizona_Desert-ation_1967-04.pdf`
  - `An_Arizona_Desert-ation_1967-06-20.pdf`

</details>

### MARC and Call Number

Marc data and call number are obtained from the [asla catalogue](https://asla.ent.sirsi.net/client/en_US/default) and pasted when prompted.

<details>
  <summary>Example MARC</summary>
  
  ```
  Tag	Ind.	Subfields
  001	 	ocn893691141
  003	 	OCoLC
  005	 	20141024031649.0
  008	 	141024u19uuuuuuazumr 0 0eng d
  035		$a(Sirsi) o893691141
  035		$a(OCoLC)893691141
  040		$aAZP$cAZP
  049		$aAZPF
  245	03	$aAn Arizona desert-ation.
  246	13	$aArizona desertation.
  246	13	$aArizona desert ation.
  246	13	$aArizona dissertation.
  260		$aPhoenix, Ariz. :$bDesert Sunshine Exposure Tests.
  300		$billustrations ;$c28 cm.
  336		$atext$btxt$2rdacontent
  337		$aunmediated$bn$2rdamedia
  338		$avolume$bnc$2rdacarrier
  500		$a"C.R. Caryl, director."
  588		$aDescription based on: April 1967 ; title from caption
  610	20	$aDesert Sunshine Exposure Tests (Phoenix, Ariz.)
  650	0	$aSolar radiation$xEnvironmental effects$xTesting.
  650	0	$aMaterials$xTesting.
  700	1	$aCaryl, C. R.
  710	2	$aDesert Sunshine Exposure Tests (Phoenix, Ariz.)
  ```

</details>

## Build steps

On MacOS you may need to run `xcode-select --install` to be able to compile macros.

1. clone the repository
2. run `cargo build --release`

### Post-build

#### Windows

Add `<path_to_clone_directory>\target\release` to your PATH, or alias it in your powershell `$PROFILE`.

#### MacOS

Create an alias in `.zshrc`: 
```zsh
alias robo_archiver=<path_to_clone_directory>\target\release\robo_archiver
```
