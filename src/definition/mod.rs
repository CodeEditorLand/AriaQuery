pub enum ARIAAbstractRole {
    Command,
    Composite,
    Input,
    Landmark,
    Range,
    RoleType,
    Section,
    SectionHead,
    Select,
    Structure,
    Widget,
    Window,
}

pub enum ARIAWidgetRole {
    Button,
    CheckBox,
    GridCell,
    Link,
    MenuItem,
    MenuItemCheckBox,
    MenuItemRadio,
    Option,
    ProgressBar,
    Radio,
    ScrollBar,
    SearchBox,
    Slider,
    SpinButton,
    Switch,
    Tab,
    TabPanel,
    TextBox,
    TreeItem,
}

pub enum ARIACompositeWidgetRole {
    ComboBox,
    Grid,
    ListBox,
    Menu,
    MenuBar,
    RadioGroup,
    TabList,
    Tree,
    TreeGrid,
}

pub enum ARIADocumentStructureRole {
    Application,
    Article,
    BlockQuote,
    Caption,
    Cell,
    ColumnHeader,
    Definition,
    Deletion,
    Directory,
    Document,
    Emphasis,
    Feed,
    Figure,
    Generic,
    Group,
    Heading,
    Img,
    Insertion,
    List,
    ListItem,
    Mark,
    Math,
    Meter,
    None,
    Note,
    Paragraph,
    Presentation,
    Row,
    RowGroup,
    RowHeader,
    Separator,
    Strong,
    Subscript,
    Superscript,
    Table,
    Term,
    Time,
    ToolBar,
    ToolTip,
}

pub enum ARIALandmarkRole {
    Banner,
    Complementary,
    ContentInfo,
    Form,
    Main,
    Navigation,
    Region,
    Search,
}

pub enum ARIALiveRegionRole {
    Alert,
    Log,
    Marquee,
    Status,
    Timer,
}

pub enum ARIAWindowRole {
    AlertDialog,
    Dialog,
}

pub enum ARIAUncategorizedRole {
    Code,
}

// please define enum equal to the above flow type
pub enum ARIADPubRole {
    DocAbstract,
    DocAcknowledgments,
    DocAfterword,
    DocAppendix,
    DocBacklink,
    DocBiblioentry,
    DocBibliography,
    DocBiblioref,
    DocChapter,
    DocColophon,
    DocConclusion,
    DocCover,
    DocCredit,
    DocCredits,
    DocDedication,
    DocEndnote,
    DocEndnotes,
    DocEpigraph,
    DocEpilogue,
    DocErrata,
    DocExample,
    DocFootnote,
    DocForeword,
    DocGlossary,
    DocGlossref,
    DocIndex,
    DocIntroduction,
    DocNoteref,
    DocNotice,
    DocPagebreak,
    DocPagelist,
    DocPart,
    DocPreface,
    DocPrologue,
    DocPullquote,
    DocQna,
    DocSubtitle,
    DocTip,
    DocToc,
}

pub enum ARIAGraphicsRole {
    GraphicsDocument,
    GraphicsObject,
    GraphicsSymbol,
}

pub enum ARIARole {
    ARIAWidgetRole(ARIAWidgetRole),
    ARIACompositeWidgetRole(ARIACompositeWidgetRole),
    ARIADocumentStructureRole(ARIADocumentStructureRole),
    ARIALandmarkRole(ARIALandmarkRole),
    ARIALiveRegionRole(ARIALiveRegionRole),
    ARIAWindowRole(ARIAWindowRole),
    ARIAUncategorizedRole(ARIAUncategorizedRole),
    ARIADPubRole(ARIADPubRole),
    ARIAGraphicsRole(ARIAGraphicsRole),
}

pub enum ARIARoleDefinitionKey {
    ARIAAbstractRole(ARIAAbstractRole),
    ARIARole(ARIARole),
    ARIADPubRole(ARIADPubRole),
    ARIAGraphicsRole(ARIAGraphicsRole),
}

pub enum ARIANameFromSources {
    Author,
    Contents,
    Prohibited,
}
