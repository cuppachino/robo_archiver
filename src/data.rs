use serde::{ Deserialize, Serialize };
use derive_more::{ From, Display };

#[derive(Serialize, Deserialize, Debug)]
pub struct Periodical {
    /// An overall summary of the periodical and its issues. 1-3 sentences.
    ///
    /// This is provided by an operator.
    pub description: String,
    /// The collection name that the periodical is a part of.
    pub collection: PeriodicalCollection,
    /// The institution that holds the item.
    pub contributing_institution: ContributingInstitution,
    /// A collection of issues that make up the periodical.
    pub issues: Vec<Issue>,
    /// Topics manually added by the operator, up to 3.
    ///
    /// These should be listed in alphabetical order and separated by a pipe `|`.
    pub topics: Vec<String>,
}

pub const FAMILY_SEARCH_DIGITIZING_INSTITUION: &str = "FamilySearch International";
pub const AZ_CONTRIBUTING_INSTITUION: &str =
    "State of Arizona Research Library- Arizona State Library, Archives and Public Records";
pub const AZ_PERIDICAL_COLLECTION: &str = "Arizona Collection|Arizona Periodicals and Magazines";
pub const AZ_RIGHTS_STATEMENT: &str =
    "NO COPYRIGHT - UNITED STATES. The organization that has made the Item available believes that the Item is in the Public Domain under the laws of the United States, but a determination was not made as to its copyright status under the copyright laws of other countries. The Item may not be in the Public Domain under the laws of other countries. Please refer to the organization that has made the Item available for more information. http://rightsstatements.org/vocab/NoC-US/1.0/";

/// Defaults to [`FAMILY_SEARCH_DIGITIZING_INSTITUION`].
#[derive(Clone, Serialize, Deserialize, Debug, From, Display)]
pub struct DigitizingInstitution(pub String);

impl Default for DigitizingInstitution {
    fn default() -> Self {
        Self(FAMILY_SEARCH_DIGITIZING_INSTITUION.to_string())
    }
}

/// Defaults to [`AZ_CONTRIBUTING_INSTITUION`].
#[derive(Clone, Serialize, Deserialize, Debug, From, Display)]
pub struct ContributingInstitution(pub String);

impl Default for ContributingInstitution {
    fn default() -> Self {
        Self(AZ_CONTRIBUTING_INSTITUION.to_string())
    }
}

/// Defaults to [`AZ_PERIDICAL_COLLECTION`].
#[derive(Clone, Serialize, Deserialize, Debug, From, Display)]
pub struct PeriodicalCollection(pub String);

impl Default for PeriodicalCollection {
    fn default() -> Self {
        Self(AZ_PERIDICAL_COLLECTION.to_string())
    }
}

/// Defaults to [`AZ_RIGHTS_STATEMENT`].
#[derive(Clone, Serialize, Deserialize, Debug, From, Display)]
pub struct RightsStatement(pub String);

impl Default for RightsStatement {
    fn default() -> Self {
        Self(AZ_RIGHTS_STATEMENT.to_string())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum IssueNo {
    Number(String),
    Season(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IssueType {
    Text,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IssueFormatType {
    Periodical,
}

/// The call number of the item. Obtained from the catalog.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CallNumber {
    /// E.g. `PERIODICAL`.
    Periodical,
    ///
    /// E.g. `AZ CATTLELOG 1945-1976`.
    Shelf(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DigitalFormat {
    PDF,
    Other(String),
}

impl From<&str> for DigitalFormat {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pdf" => DigitalFormat::PDF,
            _ => DigitalFormat::Other(s.to_string()),
        }
    }
}

impl From<String> for DigitalFormat {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "pdf" => DigitalFormat::PDF,
            _ => DigitalFormat::Other(s),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarcData {
    /// The creator of the item. Obtained from the catalog or MARC record.
    ///
    /// Multiple creators are separated with a pipe `|`.
    ///
    /// MARC field: 100 or 110, sometimes 700 and 710.
    ///
    /// Format: `First name Last name, (dates)`.
    #[serde(rename = "Creator")]
    pub creators: Vec<String>,

    /// Publisher name from the periodical, obtained from the catalog.
    ///
    /// Sometimes the publisher's name will be shortened in the catalog, but the full name should be used when possible.
    ///
    /// ### Warn
    ///
    /// Publishers and presses/printers are not the same. This field is for the publisher.
    ///
    /// MARC field: 260 or 264 subfield b. (a is the place of publication, we just need b, c is the date of publication.)
    #[serde(rename = "Publisher")]
    pub publisher: String,

    /// Subject headings from the periodical, obtained from the catalog.
    ///
    /// Subjects are separated with a pipe `|`.
    ///
    /// MARC field: 610, 650 (possibly any 600???).
    ///
    /// Format examples:
    /// - `Mexico$xEconomic conditions -> Mexico--Economic conditions `.
    /// - `Water rights$zArizona. -> Water rights--Arizona`.
    ///
    /// If there is only one heading, it should be noted for further review by the operator, and an
    /// additional heading should be grabbed from https://authorities.loc.gov/cgi-bin/Pwebrecon.cgi?DB=local&PAGE=First.
    #[serde(rename = "Subject")]
    pub subject_headings: Vec<String>,

    /// Essentially the serial number of the issue. Obtained from the catalog.
    ///
    /// If the call number is Periodical, this can be left blank.
    #[serde(rename = "Call Number")]
    pub call_number: CallNumber,

    /// Obtained from the catalog. If the number is less than 9 digits, left pad with zeros.
    ///
    /// MARC field: 001 or 003.
    ///
    /// E.g. `004 -> 000000004`.
    #[serde(rename = "OCLC Number")]
    pub oclc_number: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    /// The title of the item
    ///
    /// `Title, yyyy-mm-dd` OR `Title, yyyy-mm` OR `Title, yyyy`.
    ///
    /// If a periodical is for a two or three month period, the title should reflect that. E.g. `Title, 1930-12 & 1930-01`.
    #[serde(rename = "NODE TITLE")]
    pub node_title: String,

    /// The title of the previous issue, if there is one.
    ///
    /// Format: `Title, yyyy-mm-dd`.
    #[serde(rename = "Previous Issue")]
    pub previous_issue: Option<String>,

    /// The title of the next issue, if there is one.
    ///
    /// Format: `Title, yyyy-mm-dd`.
    #[serde(rename = "PascalCase")]
    pub next_issue: Option<String>,

    /// Contributor names from the periodical, including editors, staff members, and authors.
    ///
    /// Multiple contributors are separated with a pipe `|`.
    ///
    /// Lists of contributors longer than 7 should be noted for further review with a Collective Librarian.
    ///
    /// Format: `(title) First name Last name`.
    #[serde(rename = "Contributor")]
    pub contributors: Vec<String>,

    /// The volume number of the periodical, if applicable. Obtained from the periodical.
    ///
    /// This is sometimes a roman numeral, but should be entered as an arabic numeral.
    #[serde(rename = "Volume")]
    pub volume_no: Option<String>,

    /// The issue number of the periodical, if applicable. Obtained from the periodical.
    ///
    /// This is sometimes a roman numeral, but should be entered as an arabic numeral.
    ///
    /// If the issue is seasonal, rather than numerical, this field should be entered along with the issue name in the `description` field of the spreadsheet.
    #[serde(rename = "Issue")]
    pub issue_no: Option<IssueNo>,

    /// The year of specific issue.
    ///
    /// Format: `yyyy` or `yyyy-mm` or `yyyy-mm-dd`.
    ///
    /// If there are multiple dates, list them all and separate them with an em-dash `—-`.
    ///
    /// If the date is seasonal, type in the date range. E.g. `Summer 1918 -> 1918-06--1918-08`.
    #[serde(rename = "Date Original")]
    pub data_original: Vec<String>,

    /// The date range of the issue.
    ///
    /// E.g. `1845 -> 1840s (1840-1849)`.
    ///
    /// If there are multiple dates, list them all and separate them with a pipe `|`.
    ///
    /// E.g. `Item published in 1856 and reprinted in 1973 -> 1850s (1850-1859)|1970s (1970-1979)`.
    #[serde(rename = "Date Range")]
    pub date_range: Vec<String>,

    /// The type of the item.
    #[serde(rename = "Type")]
    pub item_type: IssueType,

    /// The original format the item was published in.
    #[serde(rename = "Original Format")]
    pub format_type: IssueFormatType,

    /// The language of the item.
    ///
    /// Most likely English, but there will occasionally be other languages.
    ///
    /// Separate multiple languages with a pipe `|`.
    ///
    /// E.g. `English|Spanish`.
    #[serde(rename = "Language")]
    pub languages: Vec<String>,

    /// Name of the periodical the issue is a part of (just the title, not the full citation).
    #[serde(rename = "Subcollection")]
    pub parent_collection: String,

    /// The MARC data for the parent periodical and issue.
    pub marc: MarcData,

    /// The copyright statement.
    #[serde(rename = "Rights Statement")]
    pub rights_statement: RightsStatement,

    /// The digital format of the document.
    #[serde(rename = "Digital Format")]
    pub digital_format: DigitalFormat,

    /// The digitizing institution.
    #[serde(rename = "Digitizing Institution")]
    pub digitizing_institution: DigitizingInstitution,
}
