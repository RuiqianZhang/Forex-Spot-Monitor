import { ref } from 'vue';

// 支持的语言列表
export type Locale = 'zh' | 'en';

// 当前语言（响应式）
export const currentLocale = ref<Locale>('zh');

// 语言包定义
const messages: Record<Locale, Record<string, string>> = {
    zh: {
        // 顶部
        appTitle: '数据源配置',
        tabProviders: '数据源',
        tabParsers: '解析规则',
        tabGlobal: '通用设置',
        saving: '保存中...',
        saveConfig: '保存配置',
        loading: '正在加载...',

        // 左侧面板标题
        panelGlobal: '通用设置',
        panelProviders: '数据源配置',
        panelParsers: '解析规则配置',

        // 通用设置
        showInstrumentName: '菜单栏显示品种名称',
        showInstrumentNameDesc: '开启后，系统菜单栏会显示品种的中文名称；关闭则只显示价格',
        refreshInterval: '刷新频率',
        refreshIntervalDesc: '系统托盘行情的数据更新间隔（秒）',
        language: '界面语言',
        languageDesc: '切换应用界面的显示语言',

        // 数据源
        selectProvider: '选择数据源:',
        addProvider: '添加',
        deleteProvider: '删除',
        fieldKey: 'Key',
        fieldLabel: '显示名称',
        fieldMethod: '请求方式',
        fieldTimeout: '超时(秒)',
        fieldUrlTemplate: 'URL 模板',
        urlHint: '提示：URL 中的 {symbol} 会被自动替换为品种代号',
        fieldImpersonate: '浏览器模拟',
        fieldSslVerify: 'SSL 验证',
        sectionHeaders: 'HTTP Headers',
        sectionParams: 'URL 查询参数 (?key=value)',
        paramsHint: '一般建议直接写在 URL 模板中',
        sectionBody: '请求体 Body (POST)',
        jsonSyntaxError: '[语法错误]',

        // 解析规则
        parserProvider: '数据源:',
        parserInstrument: '品种:',
        addInstrument: '添加品种',
        deleteInstrument: '删除',
        parserSettings: '解析规则设置',
        instrLabel: '显示名称',
        instrSymbol: '品种代号',
        instrGroup: '所属分组',
        parserType: '解析方式',
        optJsonFields: 'JSON_FIELDS (通用 JSON 字段映射)',
        optSinaForex: 'SINA_FOREX (新浪外汇)',
        optSinaFutures: 'SINA_FUTURES (新浪期货)',
        optSwissquote: 'SWISSQUOTE_BBO (瑞讯银行)',
        optGoldApi: 'GOLD_API_XAU (黄金 API)',
        pricePath: '价格路径',
        bidAskPath: '买/卖价',
        timePath: '时间路径',
        builtinParser: '使用内置解析器',
        builtinParserDesc: '该数据源使用专用的解析器，无需手动配置字段路径。',

        // 测试面板
        testSymbolPlaceholder: '输入测试用的品种代号',
        stopRequest: '停止请求',
        requesting: '请求中...',
        sendRequest: '发送请求',
        testHint: '点击上方「发送请求」测试数据源连通性',
        requestInfo: '请求信息',
        requestBody: '请求体',
        responseHeaders: '响应头',

        // Toast 消息
        saveFailJsonError: '保存失败：JSON 格式有误',
        configSaved: '配置已保存！',
        saveFailed: '保存失败: ',
        requestCancelled: '已取消请求',
        jsonCheckError: 'JSON 格式有误，请检查后重试',
        requestFailed: '请求失败: ',
        connectionError: '连接错误: ',

        // 新建默认值
        newProviderLabel: '新数据源',
        newInstrumentLabel: '新品种',
        defaultGroup: '默认分组',
    },

    en: {
        // 顶部
        appTitle: 'Data Source Config',
        tabProviders: 'Providers',
        tabParsers: 'Parsers',
        tabGlobal: 'Settings',
        saving: 'Saving...',
        saveConfig: 'Save',
        loading: 'Loading...',

        // 左侧面板标题
        panelGlobal: 'General Settings',
        panelProviders: 'Provider Configuration',
        panelParsers: 'Parser Configuration',

        // 通用设置
        showInstrumentName: 'Show instrument name in menu bar',
        showInstrumentNameDesc: 'When enabled, the menu bar displays the instrument name alongside the price',
        refreshInterval: 'Refresh Interval',
        refreshIntervalDesc: 'Data update interval for the menu bar (in seconds)',
        language: 'Language',
        languageDesc: 'Switch the display language of the application',

        // 数据源
        selectProvider: 'Provider:',
        addProvider: 'Add',
        deleteProvider: 'Delete',
        fieldKey: 'Key',
        fieldLabel: 'Label',
        fieldMethod: 'Method',
        fieldTimeout: 'Timeout(s)',
        fieldUrlTemplate: 'URL Template',
        urlHint: 'Tip: {symbol} in the URL will be replaced with the instrument symbol',
        fieldImpersonate: 'Impersonate',
        fieldSslVerify: 'SSL Verify',
        sectionHeaders: 'HTTP Headers',
        sectionParams: 'URL Query Params (?key=value)',
        paramsHint: 'Usually better to include params directly in the URL template',
        sectionBody: 'Request Body (POST)',
        jsonSyntaxError: '[Syntax Error]',

        // 解析规则
        parserProvider: 'Provider:',
        parserInstrument: 'Instrument:',
        addInstrument: 'Add Instrument',
        deleteInstrument: 'Delete',
        parserSettings: 'Parser Settings',
        instrLabel: 'Label',
        instrSymbol: 'Symbol',
        instrGroup: 'Group',
        parserType: 'Parser Type',
        optJsonFields: 'JSON_FIELDS (Generic JSON mapping)',
        optSinaForex: 'SINA_FOREX (Sina Forex)',
        optSinaFutures: 'SINA_FUTURES (Sina Futures)',
        optSwissquote: 'SWISSQUOTE_BBO (Swissquote)',
        optGoldApi: 'GOLD_API_XAU (Gold API)',
        pricePath: 'Price Path',
        bidAskPath: 'Bid / Ask',
        timePath: 'Time Path',
        builtinParser: 'Built-in Parser Active',
        builtinParserDesc: 'This data source uses a dedicated parser. No manual field configuration needed.',

        // 测试面板
        testSymbolPlaceholder: 'Enter test symbol',
        stopRequest: 'Cancel',
        requesting: 'Requesting...',
        sendRequest: 'Send',
        testHint: 'Click "Send" above to test data source connectivity',
        requestInfo: 'Request Info',
        requestBody: 'Request Body',
        responseHeaders: 'Response Headers',

        // Toast 消息
        saveFailJsonError: 'Save failed: Invalid JSON format',
        configSaved: 'Configuration saved!',
        saveFailed: 'Save failed: ',
        requestCancelled: 'Request cancelled',
        jsonCheckError: 'Invalid JSON format, please check and retry',
        requestFailed: 'Request failed: ',
        connectionError: 'Connection error: ',

        // 新建默认值
        newProviderLabel: 'New Provider',
        newInstrumentLabel: 'New Instrument',
        defaultGroup: 'Default',
    }
};

// 翻译函数
export function t(key: string): string {
    return messages[currentLocale.value]?.[key] ?? key;
}

// 切换语言
export function setLocale(locale: Locale) {
    currentLocale.value = locale;
    // 持久化到 localStorage
    try { localStorage.setItem('app_locale', locale); } catch { }
}

// 初始化时从 localStorage 读取
export function initLocale() {
    try {
        const saved = localStorage.getItem('app_locale') as Locale | null;
        if (saved && (saved === 'zh' || saved === 'en')) {
            currentLocale.value = saved;
        }
    } catch { }
}
