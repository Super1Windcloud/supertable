#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Locale {
    ZhCn,
    EnUs,
}

impl Locale {
    pub fn toggle(self) -> Self {
        match self {
            Self::ZhCn => Self::EnUs,
            Self::EnUs => Self::ZhCn,
        }
    }

    pub fn switch_label(self) -> &'static str {
        match self {
            Self::ZhCn => "EN",
            Self::EnUs => "中",
        }
    }

    pub fn app_tagline(self) -> &'static str {
        match self {
            Self::ZhCn => "优雅的数据工作台，用于连接、查询与探索",
            Self::EnUs => "Elegant data workspace for query, inspect and connect",
        }
    }

    pub fn grid_search_placeholder(self) -> &'static str {
        match self {
            Self::ZhCn => "筛选结果中的 234 行",
            Self::EnUs => "Filter 234 rows in result set",
        }
    }

    pub fn connection_name_placeholder(self) -> &'static str {
        match self {
            Self::ZhCn => "连接名称",
            Self::EnUs => "Connection name",
        }
    }

    pub fn connection_database_placeholder(self) -> &'static str {
        match self {
            Self::ZhCn => "数据库 / schema / db index",
            Self::EnUs => "Database / schema / db index",
        }
    }

    pub fn connection_username_placeholder(self) -> &'static str {
        match self {
            Self::ZhCn => "用户名",
            Self::EnUs => "Username",
        }
    }

    pub fn connection_password_placeholder(self) -> &'static str {
        match self {
            Self::ZhCn => "密码",
            Self::EnUs => "Password",
        }
    }

    pub fn open_source_badge(self) -> &'static str {
        match self {
            Self::ZhCn => "MIT 开源免费",
            Self::EnUs => "MIT Open Source",
        }
    }

    pub fn new_query(self) -> &'static str {
        match self {
            Self::ZhCn => "新建查询",
            Self::EnUs => "New Query",
        }
    }

    pub fn add_connection(self) -> &'static str {
        match self {
            Self::ZhCn => "添加连接",
            Self::EnUs => "Add Connection",
        }
    }

    pub fn export(self) -> &'static str {
        match self {
            Self::ZhCn => "社区版",
            Self::EnUs => "Community",
        }
    }

    pub fn connections(self) -> &'static str {
        match self {
            Self::ZhCn => "连接",
            Self::EnUs => "Connections",
        }
    }

    pub fn configured_endpoints(self, count: usize) -> String {
        match self {
            Self::ZhCn => format!("已配置 {count} 个端点，随时可用"),
            Self::EnUs => format!("{count} configured endpoints ready to use"),
        }
    }

    pub fn today(self) -> &'static str {
        match self {
            Self::ZhCn => "今日",
            Self::EnUs => "Today",
        }
    }

    pub fn sync_healthy(self) -> &'static str {
        match self {
            Self::ZhCn => "已连接后从真实数据库读取",
            Self::EnUs => "Reads directly from live database sources",
        }
    }

    pub fn latency_hint(self) -> &'static str {
        match self {
            Self::ZhCn => "支持 MySQL、PostgreSQL、Redis、MongoDB、SQLite 驱动",
            Self::EnUs => "Includes MySQL, PostgreSQL, Redis, MongoDB and SQLite drivers",
        }
    }

    pub fn create_connection(self) -> &'static str {
        match self {
            Self::ZhCn => "创建连接",
            Self::EnUs => "Create Connection",
        }
    }

    pub fn database_explorer(self) -> &'static str {
        match self {
            Self::ZhCn => "数据库浏览器",
            Self::EnUs => "Database Explorer",
        }
    }

    pub fn schema_item(self, label: &str) -> String {
        match (self, label) {
            (Self::ZhCn, "Tables") => "表".to_string(),
            (Self::ZhCn, "Views") => "视图".to_string(),
            (Self::ZhCn, "Functions") => "函数".to_string(),
            (Self::ZhCn, "Triggers") => "触发器".to_string(),
            (Self::ZhCn, "Users") => "用户".to_string(),
            (Self::ZhCn, "Migrations") => "迁移".to_string(),
            (Self::ZhCn, "Favorites") => "收藏".to_string(),
            (Self::ZhCn, "Pins") => "置顶".to_string(),
            _ => label.to_string(),
        }
    }

    pub fn get_started(self) -> &'static str {
        match self {
            Self::ZhCn => "开始使用",
            Self::EnUs => "Get Started",
        }
    }

    pub fn first_workspace(self) -> &'static str {
        match self {
            Self::ZhCn => "建立你的第一个数据工作区",
            Self::EnUs => "Build your first data workspace",
        }
    }

    pub fn onboarding_intro(self) -> &'static str {
        match self {
            Self::ZhCn => "添加连接后，你可以立即浏览结构、编写 SQL，并在统一结果面板中查看输出。",
            Self::EnUs => "Add a connection to browse schema, write SQL, and inspect output in one shared result view.",
        }
    }

    pub fn import_database(self) -> &'static str {
        match self {
            Self::ZhCn => "导入数据库",
            Self::EnUs => "Import Database",
        }
    }

    pub fn version(self) -> &'static str {
        match self {
            Self::ZhCn => "版本",
            Self::EnUs => "Version",
        }
    }

    pub fn welcome_copy(self) -> &'static str {
        match self {
            Self::ZhCn => "完全开源免费，为多种数据库连接、探索与查询打造的现代工作台",
            Self::EnUs => "A fully open-source and free workspace for connecting, exploring and querying across databases",
        }
    }

    pub fn info_fast_setup_title(self) -> &'static str {
        match self {
            Self::ZhCn => "快速接入",
            Self::EnUs => "Fast setup",
        }
    }

    pub fn info_fast_setup_body(self) -> &'static str {
        match self {
            Self::ZhCn => "支持导入 MySQL / PostgreSQL / Redis / MongoDB / SQLite",
            Self::EnUs => "Supports importing MySQL / PostgreSQL / Redis / MongoDB / SQLite",
        }
    }

    pub fn info_focused_workflow_title(self) -> &'static str {
        match self {
            Self::ZhCn => "无付费限制",
            Self::EnUs => "No paywall",
        }
    }

    pub fn info_focused_workflow_body(self) -> &'static str {
        match self {
            Self::ZhCn => "默认提供完整界面能力，不区分免费版与专业版",
            Self::EnUs => "The full interface ships without a separate free tier or pro tier",
        }
    }

    pub fn create_connection_title(self) -> &'static str {
        match self {
            Self::ZhCn => "创建连接",
            Self::EnUs => "Create Connection",
        }
    }

    pub fn import_database_title(self) -> &'static str {
        match self {
            Self::ZhCn => "导入数据库",
            Self::EnUs => "Import Database",
        }
    }

    pub fn configure_connection(self, kind: &str) -> String {
        match self {
            Self::ZhCn => format!("配置 {kind} 连接并保存到你的工作区"),
            Self::EnUs => format!("Configure a {kind} connection and save it to your workspace"),
        }
    }

    pub fn close(self) -> &'static str {
        match self {
            Self::ZhCn => "关闭",
            Self::EnUs => "Close",
        }
    }

    pub fn connection_preset(self) -> &'static str {
        match self {
            Self::ZhCn => "连接预设",
            Self::EnUs => "Connection preset",
        }
    }

    pub fn preset_hint(self) -> &'static str {
        match self {
            Self::ZhCn => "切换数据库类型会自动填充默认端口",
            Self::EnUs => "Switching database type fills the default port automatically",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::ZhCn => "名称",
            Self::EnUs => "Name",
        }
    }

    pub fn host(self) -> &'static str {
        match self {
            Self::ZhCn => "主机",
            Self::EnUs => "Host",
        }
    }

    pub fn port(self) -> &'static str {
        match self {
            Self::ZhCn => "端口",
            Self::EnUs => "Port",
        }
    }

    pub fn database(self) -> &'static str {
        match self {
            Self::ZhCn => "数据库",
            Self::EnUs => "Database",
        }
    }

    pub fn database_file(self) -> &'static str {
        match self {
            Self::ZhCn => "数据库文件",
            Self::EnUs => "Database file",
        }
    }

    pub fn tag_or_note(self) -> &'static str {
        match self {
            Self::ZhCn => "标签 / 备注",
            Self::EnUs => "Tag / note",
        }
    }

    pub fn username(self) -> &'static str {
        match self {
            Self::ZhCn => "用户名",
            Self::EnUs => "Username",
        }
    }

    pub fn password(self) -> &'static str {
        match self {
            Self::ZhCn => "密码",
            Self::EnUs => "Password",
        }
    }

    pub fn supported_connection_types(self) -> &'static str {
        match self {
            Self::ZhCn => "支持的连接类型",
            Self::EnUs => "Supported connection types",
        }
    }

    pub fn cancel(self) -> &'static str {
        match self {
            Self::ZhCn => "取消",
            Self::EnUs => "Cancel",
        }
    }

    pub fn save_connection(self) -> &'static str {
        match self {
            Self::ZhCn => "保存连接",
            Self::EnUs => "Save Connection",
        }
    }

    pub fn run(self) -> &'static str {
        match self {
            Self::ZhCn => "运行",
            Self::EnUs => "Run",
        }
    }

    pub fn format(self) -> &'static str {
        match self {
            Self::ZhCn => "格式化",
            Self::EnUs => "Format",
        }
    }

    pub fn explain(self) -> &'static str {
        match self {
            Self::ZhCn => "解析",
            Self::EnUs => "Explain",
        }
    }

    pub fn connected(self) -> &'static str {
        match self {
            Self::ZhCn => "已连接",
            Self::EnUs => "Connected",
        }
    }

    pub fn execute_hint(self) -> &'static str {
        match self {
            Self::ZhCn => "Ctrl+Enter 执行当前选择",
            Self::EnUs => "Ctrl+Enter to execute selection",
        }
    }

    pub fn live_draft(self) -> &'static str {
        match self {
            Self::ZhCn => "实时草稿",
            Self::EnUs => "Live draft",
        }
    }

    pub fn result_stats(self) -> &'static str {
        match self {
            Self::ZhCn => "数据库源返回",
            Self::EnUs => "Returned from source",
        }
    }

    pub fn live_source(self) -> &'static str {
        match self {
            Self::ZhCn => "实时数据源",
            Self::EnUs => "Live source",
        }
    }

    pub fn no_data(self) -> &'static str {
        match self {
            Self::ZhCn => "当前数据源没有可展示的数据",
            Self::EnUs => "No data available for the current source",
        }
    }

    pub fn load_failed(self) -> &'static str {
        match self {
            Self::ZhCn => "读取失败",
            Self::EnUs => "Load failed",
        }
    }

    pub fn data_tab(self) -> &'static str {
        match self {
            Self::ZhCn => "数据",
            Self::EnUs => "Data",
        }
    }

    pub fn structure_tab(self) -> &'static str {
        match self {
            Self::ZhCn => "结构",
            Self::EnUs => "Structure",
        }
    }

    pub fn console_tab(self) -> &'static str {
        match self {
            Self::ZhCn => "控制台",
            Self::EnUs => "Console",
        }
    }

    pub fn sqlite_file(self) -> &'static str {
        match self {
            Self::ZhCn => "本地数据库文件",
            Self::EnUs => "Local database file",
        }
    }

    pub fn memory_cache(self) -> &'static str {
        match self {
            Self::ZhCn => "内存缓存",
            Self::EnUs => "In-memory cache",
        }
    }

    pub fn endpoint_label(self, kind: &str) -> String {
        match self {
            Self::ZhCn => format!("{kind} 端点"),
            Self::EnUs => format!("{kind} endpoint"),
        }
    }
}
