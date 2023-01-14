#![allow(dead_code)]

use semver::Version;

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct GoTool<'a> {
    pub name:                     &'a str,
    pub import_path:              &'a str,
    pub module_path:              &'a str,
    pub important:                bool,
    pub replaced_by_gopls:        bool,
    pub description:              &'a str,
    pub default_version:          Option<&'a str>,
    pub latest_version:           Option<&'a str>,
    pub latest_version_timestamp: Option<std::time::SystemTime>,
    pub minimum_go_version:       Option<semver::Version>,
}

impl<'a> std::fmt::Display for GoTool<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl GoTool<'_> {
    pub fn install_path(&self) -> String {
        let mut s = String::new();
        s.push_str(self.import_path);
        s.push('@');
        s.push_str(
            self.latest_version
                .unwrap_or_else(|| self.default_version.unwrap_or("latest")),
        );
        s
    }
}

const GOCODE: GoTool<'_> = GoTool {
    name:                     "gocode",
    import_path:              "github.com/mdempsky/gocode",
    module_path:              "github.com/mdempsky/gocode",
    important:                true,
    replaced_by_gopls:        true,
    description:              "Auto-completion, does not work with modules",
    default_version:          Some("v0.0.0-20200405233807-4acdcbdea79d"),
    minimum_go_version:       None,
    latest_version:           None,
    latest_version_timestamp: None,
};

const GOCODE_GOMOD: GoTool<'_> = GoTool {
    name:                     "gocode-gomod",
    import_path:              "github.com/stamblerre/gocode",
    module_path:              "github.com/stamblerre/gocode",
    important:                true,
    replaced_by_gopls:        true,
    description:              "Auto-completion, does not work with modules",
    default_version:          Some("v1.0.0"),
    minimum_go_version:       Some(Version::new(1, 11, 0)),
    latest_version:           None,
    latest_version_timestamp: None,
};

const GO_OUTLINE: GoTool<'_> = GoTool {
    name:                     "go-outline",
    import_path:              "github.com/ramya-rao-a/go-outline",
    module_path:              "github.com/ramya-rao-a/go-outline",
    important:                true,
    replaced_by_gopls:        true,
    description:              "Go to symbol in file",
    default_version:          Some("v0.0.0-20210608161538-9736a4bde949"),
    minimum_go_version:       None,
    latest_version:           None,
    latest_version_timestamp: None,
};

const GO_SYMBOLS: GoTool<'_> = GoTool {
    name:                     "go-symbols",
    import_path:              "github.com/acroca/go-symbols",
    module_path:              "github.com/acroca/go-symbols",
    important:                false,
    replaced_by_gopls:        true,
    description:              "Go to symbol in workspace",
    default_version:          Some("v0.1.1"),
    minimum_go_version:       None,
    latest_version:           None,
    latest_version_timestamp: None,
};

const GURU: GoTool<'_> = GoTool {
    name:                     "guru",
    import_path:              "golang.org/x/tools/cmd/guru",
    module_path:              "golang.org/x/tools",
    important:                false,
    replaced_by_gopls:        true,
    description:              "Find all references and Go to implementation of symbols",
    default_version:          None,
    minimum_go_version:       None,
    latest_version:           None,
    latest_version_timestamp: None,
};

const GORENAME: GoTool<'_> = GoTool {
    name:                     "gorename",
    import_path:              "golang.org/x/tools/cmd/gorename",
    module_path:              "golang.org/x/tools",
    replaced_by_gopls:        true,
    important:                false,
    description:              "Rename symbols",
    default_version:          None,
    minimum_go_version:       None,
    latest_version:           None,
    latest_version_timestamp: None,
};

const STATICCHECK: GoTool<'_> = GoTool {
    name:                     "staticcheck",
    import_path:              "honnef.co/go/tools/cmd/staticcheck",
    module_path:              "honnef.co/go/tools",
    important:                true,
    replaced_by_gopls:        false,
    description:              "Linter",
    default_version:          None,
    latest_version:           None,
    latest_version_timestamp: None,
    minimum_go_version:       None,
};

const GOLANGCI_LINT: GoTool<'_> = GoTool {
    name:                     "golangci-lint",
    import_path:              "github.com/golangci/golangci-lint/cmd/golangci-lint",
    module_path:              "github.com/golangci/golangci-lint",
    important:                true,
    replaced_by_gopls:        false,
    description:              "Linter",
    default_version:          Some("v1.50.1"),
    latest_version:           None,
    latest_version_timestamp: None,
    minimum_go_version:       None,
};

const REVIVE: GoTool<'_> = GoTool {
    name:                     "revive",
    import_path:              "github.com/mgechev/revive",
    module_path:              "github.com/mgechev/revive",
    important:                true,
    replaced_by_gopls:        false,
    description:              "Linter",
    default_version:          Some("v1.2.3"),
    latest_version:           None,
    latest_version_timestamp: None,
    minimum_go_version:       None,
};

const GOPLS: GoTool<'_> = GoTool {
    name:                     "gopls",
    import_path:              "golang.org/x/tools/gopls",
    module_path:              "golang.org/x/tools/gopls",
    important:                true,
    replaced_by_gopls:        false,
    description:              "Language Server from Google",
    default_version:          None,
    latest_version:           Some("v0.10.1"),
    latest_version_timestamp: None,
    minimum_go_version:       Some(Version::new(1, 13, 0)),
};

const DLV: GoTool<'_> = GoTool {
    name:                     "dlv",
    import_path:              "github.com/go-delve/delve/cmd/dlv",
    module_path:              "github.com/go-delve/delve",
    important:                true,
    replaced_by_gopls:        false,
    description:              "Go debugger (Delve)",
    default_version:          None,
    minimum_go_version:       Some(Version::new(1, 12, 0)),
    latest_version:           Some("v1.6.1"),
    latest_version_timestamp: None,
};

pub(super) const ALL_TOOLS_INFORMATION: &[GoTool] =
    &[STATICCHECK, GOLANGCI_LINT, REVIVE, GOPLS, DLV];
