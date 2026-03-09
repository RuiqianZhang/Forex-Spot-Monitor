<template>
  <div class="h-screen w-full flex flex-col font-sans select-none text-[13px] text-zinc-800 dark:text-zinc-200 overflow-hidden relative" style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;">
    
    <!-- 背景装饰 -->
    <div class="absolute inset-0 z-[-2] bg-[#f2f4f8] dark:bg-[#121212] transition-colors duration-500"></div>
    <div class="absolute top-[-15%] left-[-10%] w-[50%] h-[50%] bg-[#007aff]/30 dark:bg-[#007aff]/20 rounded-full blur-[120px] z-[-1] pointer-events-none"></div>
    <div class="absolute bottom-[-15%] right-[-10%] w-[60%] h-[60%] bg-[#34c759]/20 dark:bg-[#5e5ce6]/20 rounded-full blur-[150px] z-[-1] pointer-events-none"></div>
    <div class="absolute top-[20%] right-[10%] w-[30%] h-[30%] bg-[#5856d6]/20 dark:bg-[#ff375f]/15 rounded-full blur-[100px] z-[-1] pointer-events-none"></div>

    <!-- 顶部控制栏 -->
    <header class="flex-none pt-4 pb-2 px-6 flex flex-col items-center justify-center relative z-10 drop-shadow-sm">
      <div class="flex items-center p-0.5 bg-black/[0.04] dark:bg-white/[0.08] backdrop-blur-2xl rounded-[8px] w-full max-w-[360px] shadow-inner ring-1 ring-black/[0.06] dark:ring-white/[0.1]">
        <button
          v-for="tab in ['providers', 'parsers', 'global']"
          :key="tab"
          @click="activeTab = tab"
          :class="[
            'flex-1 py-[3px] text-center font-medium rounded-[6px] transition-all duration-300 ease-[cubic-bezier(0.25,0.8,0.25,1)] text-[12px]',
            activeTab === tab 
              ? 'bg-white/90 dark:bg-[#3a3a3c]/90 text-black dark:text-white shadow-[0_2px_8px_rgba(0,0,0,0.08)] ring-1 ring-black/5 dark:ring-white/10' 
              : 'text-zinc-500 dark:text-zinc-400 hover:text-black dark:hover:text-white hover:bg-white/30 dark:hover:bg-white/5'
          ]"
        >
          {{ tab === 'providers' ? t('tabProviders') : tab === 'parsers' ? t('tabParsers') : t('tabGlobal') }}
        </button>
      </div>
      <button 
        @click="saveConfig" 
        :disabled="saving"
        class="absolute right-6 top-1/2 -translate-y-1/2 px-3 py-1.5 bg-gradient-to-b from-[#2a8bf2] to-[#007aff] hover:from-[#3a9bfb] hover:to-[#0062cc] disabled:from-[#a0cfff] disabled:to-[#007aff]/50 text-white rounded-[6px] shadow-md shadow-[#007aff]/30 font-semibold tracking-wide transition-all ring-1 ring-[#007aff]/20 text-[11px]"
      >
        {{ saving ? t('saving') : t('saveConfig') }}
      </button>
    </header>

    <main class="flex-1 w-full mx-auto px-6 pb-6 pt-1 h-full min-h-0 flex flex-col relative z-10 max-w-[1600px]">
      <div v-if="loading" class="flex-1 flex items-center justify-center text-zinc-500 font-medium tracking-widest animate-pulse">{{ t('loading') }}</div>
      
      <div v-else class="flex-1 flex flex-col md:flex-row gap-4 h-full min-h-0 min-w-0 w-full items-stretch">
        
        <!-- 左半区 -->
        <div class="flex-auto md:w-[55%] flex flex-col min-w-0 min-h-0 bg-white/40 dark:bg-black/30 backdrop-blur-3xl border border-white/60 dark:border-white/10 rounded-xl shadow-[0_8px_32px_rgba(0,0,0,0.04)] overflow-hidden">
          <div class="px-4 py-2 flex-none border-b border-black/5 dark:border-white/5 bg-white/20 dark:bg-black/20 backdrop-blur-md">
            <h2 class="font-bold text-[13px] text-zinc-800 dark:text-zinc-200">
               {{ activeTab === 'global' ? t('panelGlobal') : (activeTab === 'providers' ? t('panelProviders') : t('panelParsers')) }}
            </h2>
          </div>

          <div class="flex-1 overflow-y-auto p-4 space-y-4 custom-scrollbar relative min-w-0">
          
            <!-- GLOBAL -->
            <div v-show="activeTab === 'global'" class="space-y-3 animate-fade-in">
              <!-- 语言切换 -->
              <div class="mac-card">
                <div class="mac-row flex justify-between items-center bg-white/30 dark:bg-black/20">
                  <div>
                    <div class="font-semibold text-zinc-800 dark:text-zinc-200">{{ t('language') }}</div>
                    <div class="text-[11px] text-zinc-500 dark:text-zinc-400 mt-1">{{ t('languageDesc') }}</div>
                  </div>
                  <select v-model="locale" @change="onLocaleChange" class="mac-select w-32 font-bold text-[12px] h-7">
                    <option value="zh">简体中文</option>
                    <option value="en">English</option>
                  </select>
                </div>
              </div>
              <!-- 菜单栏品种名称 -->
              <div class="mac-card">
                <div class="mac-row flex justify-between items-center bg-white/30 dark:bg-black/20">
                  <div>
                    <div class="font-semibold text-zinc-800 dark:text-zinc-200">{{ t('showInstrumentName') }}</div>
                    <div class="text-[11px] text-zinc-500 dark:text-zinc-400 mt-1">{{ t('showInstrumentNameDesc') }}</div>
                  </div>
                  <label class="relative inline-flex items-center cursor-pointer" v-if="config.defaults">
                    <input type="checkbox" v-model="config.defaults.show_instrument_name" class="sr-only peer">
                    <div class="w-10 h-6 bg-black/10 dark:bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-[#34c759] shadow-inner transition-colors"></div>
                  </label>
                </div>
              </div>
            </div>

            <!-- PROVIDERS -->
            <div v-show="activeTab === 'providers'" class="space-y-4 animate-fade-in pb-8 min-w-0 flex flex-col min-h-full">
              <div class="flex items-center justify-between pb-1 flex-wrap gap-2 flex-none">
                <div class="flex items-center space-x-2">
                  <span class="font-semibold text-zinc-600 dark:text-zinc-400 text-[12px] uppercase">{{ t('selectProvider') }}</span>
                  <select v-model="selectedProviderIndex" class="mac-select w-44 font-bold text-[12px] h-7">
                    <option v-for="(p, i) in config.providers" :key="p.key" :value="i">{{ p.label }}</option>
                  </select>
                </div>
                <div class="flex space-x-2 ml-auto">
                  <button @click="addProvider" class="mac-icon-btn text-[#007aff] bg-[#007aff]/10 border-[#007aff]/20 hover:bg-[#007aff]/20 active:bg-[#007aff]/30 shadow-none px-3 py-1 text-[11px]">{{ t('addProvider') }}</button>
                  <button @click="removeProvider" :disabled="config.providers.length <= 1" class="mac-icon-btn text-[#ff3b30] bg-[#ff3b30]/10 border-[#ff3b30]/20 hover:bg-[#ff3b30]/20 active:bg-[#ff3b30]/30 disabled:opacity-40 shadow-none px-3 py-1 text-[11px]">{{ t('deleteProvider') }}</button>
                </div>
              </div>

              <div v-if="currentProvider" class="space-y-3 min-w-0 flex-1 flex flex-col" style="min-height: 500px">
                <div class="mac-card flex-none">
                  <div class="mac-row !py-1 pb-[2px] border-none flex"><div class="flex items-center flex-1" style="min-width:0;"><span class="w-14 flex-none font-medium text-zinc-500 text-[11px]">{{ t('fieldKey') }}</span><input v-model="currentProvider.key" class="mac-input flex-1 min-w-0 font-mono text-[11px]" /></div><div class="flex items-center flex-1 ml-2" style="min-width:0;"><span class="w-14 flex-none font-medium text-zinc-500 text-[11px] text-right pr-2">{{ t('fieldLabel') }}</span><input v-model="currentProvider.label" class="mac-input flex-1 min-w-0 font-bold text-[11px]" /></div></div>
                  <div class="mac-row !py-1 pt-0 mt-0 flex"><div class="flex items-center flex-1" style="min-width:0;"><span class="w-14 flex-none font-medium text-zinc-500 text-[11px]">{{ t('fieldMethod') }}</span><select v-model="currentProvider.request.method" class="mac-select w-full text-[#5856d6] font-bold text-[11px] h-[22px] py-0"><option value="GET">GET</option><option value="POST">POST</option></select></div><div class="flex items-center flex-1 ml-2" style="min-width:0;"><span class="w-14 flex-none font-medium text-zinc-500 text-[11px] text-right pr-2">{{ t('fieldTimeout') }}</span><input type="number" v-model="currentProvider.request.timeout" class="mac-input flex-1 min-w-0 text-center text-[11px]" /></div></div>
                  <div class="mac-row !py-1"><div class="flex items-center" style="min-width:0;"><span class="w-14 flex-none font-medium text-zinc-500 text-[11px]">{{ t('fieldUrlTemplate') }}</span><input v-model="currentProvider.request.url_template" class="mac-input flex-1 min-w-0 font-mono text-[11px] text-[#007aff]" /></div></div>
                  
                  <div class="mac-row !py-1.5 bg-black/[0.02] dark:bg-white/[0.02] flex items-center justify-between">
                     <span class="text-[9px] text-zinc-400" v-html="t('urlHint').replace('{symbol}', '<b class=&quot;text-[#007aff] bg-[#007aff]/10 px-1 rounded&quot;>{symbol}</b>')"></span>
                     <div class="flex items-center gap-2">
                       <label class="flex items-center space-x-1"><span class="font-medium text-zinc-500 text-[10px]">{{ t('fieldImpersonate') }}</span><input v-model="currentProvider.request.impersonate" class="mac-input w-24 font-mono text-[10px]" /></label>
                       <label class="flex items-center space-x-1"><input type="checkbox" v-model="currentProvider.request.verify" class="mac-checkbox w-3 h-3" /><span class="font-medium text-zinc-500 text-[10px]">{{ t('fieldSslVerify') }}</span></label>
                     </div>
                  </div>
                </div>

                <div class="mac-card flex flex-col flex-1 border-[1px] border-[#007aff]/30 dark:border-white/10 overflow-hidden min-h-[300px]">
                  
                  <div @click="toggleSection('headers')" class="px-3 py-1.5 bg-white/40 dark:bg-black/40 border-b border-black/5 dark:border-white/10 flex justify-between items-center cursor-pointer hover:bg-white/60 dark:hover:bg-black/60 transition-colors flex-none">
                    <span class="text-[10px] font-bold text-zinc-700 dark:text-zinc-300 uppercase tracking-widest shrink-0">{{ t('sectionHeaders') }}</span>
                    <svg :class="{'rotate-180':!sections.headers}" class="w-3.5 h-3.5 text-zinc-400 transition-transform duration-200" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
                  </div>
                  <div v-show="sections.headers" class="p-1.5 bg-white/20 dark:bg-black/20 border-b border-black/5 dark:border-white/5 flex-1 min-h-[100px] flex flex-col relative group">
                    <textarea v-model="headersJsonStr" @blur="syncParams" spellcheck="false" class="mac-textarea flex-1 text-[11px] p-2 leading-relaxed custom-scrollbar bg-transparent border-dashed border-black/10 dark:border-white/10 group-focus-within:border-[#007aff]/40 m-0 pattern-bg w-full h-full resize-none"></textarea>
                  </div>

                  <div @click="toggleSection('params')" class="px-3 py-1.5 bg-white/40 dark:bg-black/40 border-b border-black/5 dark:border-white/10 flex justify-between items-center cursor-pointer hover:bg-white/60 dark:hover:bg-black/60 transition-colors flex-none">
                    <span class="text-[10px] font-bold text-zinc-700 dark:text-zinc-300 uppercase tracking-widest shrink-0">{{ t('sectionParams') }}</span>
                    <svg :class="{'rotate-180':!sections.params}" class="w-3.5 h-3.5 text-zinc-400 transition-transform duration-200" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
                  </div>
                  <div v-show="sections.params" class="p-1.5 bg-white/20 dark:bg-black/20 border-b border-black/5 dark:border-white/5 flex-1 min-h-[100px] flex flex-col relative group">
                    <div class="absolute right-3 top-3 z-10 text-[9px] text-zinc-400 bg-black/5 dark:bg-white/5 px-1.5 py-0.5 rounded pointer-events-none">{{ t('paramsHint') }}</div>
                    <textarea v-model="paramsJsonStr" @blur="syncParams" spellcheck="false" class="mac-textarea flex-1 text-[11px] p-2 leading-relaxed custom-scrollbar bg-transparent border-dashed border-black/10 dark:border-white/10 group-focus-within:border-[#007aff]/40 m-0 w-full h-full resize-none"></textarea>
                  </div>

                  <div @click="toggleSection('body')" class="px-3 py-1.5 bg-white/40 dark:bg-black/40 flex justify-between items-center cursor-pointer hover:bg-white/60 dark:hover:bg-black/60 transition-colors flex-none">
                    <span class="text-[10px] font-bold text-zinc-700 dark:text-zinc-300 uppercase tracking-widest shrink-0">{{ t('sectionBody') }}</span>
                    <svg :class="{'rotate-180':!sections.body}" class="w-3.5 h-3.5 text-zinc-400 transition-transform duration-200" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
                  </div>
                  <div v-show="sections.body" class="p-1.5 bg-white/20 dark:bg-black/20 border-t border-black/5 dark:border-white/5 flex-1 min-h-[100px] flex flex-col relative group">
                    <textarea v-model="bodyJsonStr" @blur="syncParams" spellcheck="false" class="mac-textarea flex-1 text-[11px] p-2 leading-relaxed custom-scrollbar bg-transparent border-dashed border-black/10 dark:border-white/10 group-focus-within:border-[#007aff]/40 m-0 w-full h-full resize-none"></textarea>
                  </div>
                  
                  <div v-if="jsonError" class="px-4 py-2 text-[11px] text-[#ff3b30] bg-[#ff3b30]/10 border-t border-[#ff3b30]/20 font-mono z-10 font-bold flex-none">
                    {{ t('jsonSyntaxError') }} {{ jsonError }}
                  </div>
                </div>
              </div>
            </div>

            <!-- PARSERS -->
            <div v-show="activeTab === 'parsers'" class="space-y-4 animate-fade-in pb-8 min-w-0">
              <div class="flex items-center pb-1 flex-wrap gap-2">
                <div class="flex items-center space-x-1.5">
                  <span class="font-semibold text-zinc-600 dark:text-zinc-400 text-[11px] uppercase">{{ t('parserProvider') }}</span>
                  <select v-model="selectedParserProviderIndex" @change="selectedParserInstrumentIndex = 0" class="mac-select w-32 font-bold text-[#007aff] text-[11px] h-7">
                    <option v-for="(p, i) in config.providers" :key="p.key" :value="i">{{ p.label }}</option>
                  </select>
                </div>
                <div class="flex items-center space-x-1.5">
                  <span class="font-semibold text-zinc-600 dark:text-zinc-400 text-[11px] uppercase ml-1">{{ t('parserInstrument') }}</span>
                  <select v-model="selectedParserInstrumentIndex" class="mac-select w-44 font-bold text-[11px] h-7">
                    <option v-for="(inst, idx) in currentParserProvider?.instruments || []" :key="idx" :value="idx">
                      {{ inst.label }} ({{inst.symbol}})
                    </option>
                  </select>
                </div>
                <div class="flex space-x-1.5 ml-auto">
                  <button @click="addInstrument" class="mac-icon-btn text-[#34c759] bg-[#34c759]/10 border-[#34c759]/20 hover:bg-[#34c759]/20 shadow-none px-3 py-1">{{ t('addInstrument') }}</button>
                  <button @click="removeInstrument" disabled v-if="(currentParserProvider?.instruments?.length || 0) <= 1" class="mac-icon-btn text-[#ff3b30] bg-[#ff3b30]/10 border-[#ff3b30]/20 disabled:opacity-40 shadow-none px-3 py-1">{{ t('deleteInstrument') }}</button>
                  <button @click="removeInstrument" v-else class="mac-icon-btn text-[#ff3b30] bg-[#ff3b30]/10 border-[#ff3b30]/20 hover:bg-[#ff3b30]/20 shadow-none px-3 py-1">{{ t('deleteInstrument') }}</button>
                </div>
              </div>

              <div v-if="currentInstrument" class="mac-card border-[#34c759]/30 border-[1px] min-w-0">
                <div class="px-4 py-2.5 bg-gradient-to-r from-[#34c759]/10 to-transparent border-b border-black/5 dark:border-white/10 text-[11px] font-bold text-[#34c759] uppercase tracking-widest">{{ t('parserSettings') }}</div>
                <div class="mac-row bg-white/30 dark:bg-black/20">
                  <div class="flex flex-wrap gap-2">
                    <label class="flex flex-col space-y-1 w-24 flex-auto">
                      <span class="text-zinc-500 font-semibold text-[10px] uppercase">{{ t('instrLabel') }}</span><input v-model="currentInstrument.label" class="mac-input text-center font-bold text-[12px] py-1" />
                    </label>
                    <label class="flex flex-col space-y-1 w-24 flex-auto">
                      <span class="text-[#007aff] font-semibold text-[10px] uppercase">{{ t('instrSymbol') }}</span><input v-model="currentInstrument.symbol" class="mac-input font-mono text-center text-[#007aff] text-[12px] py-1" />
                    </label>
                    <label class="flex flex-col space-y-1 w-24 flex-auto">
                      <span class="text-zinc-500 font-semibold text-[10px] uppercase">{{ t('instrGroup') }}</span><input v-model="currentInstrument.group" class="mac-input text-center text-[12px] py-1" />
                    </label>
                  </div>
                </div>

                <div class="mac-row bg-black/[0.04] dark:bg-white/[0.04] py-2">
                  <div class="flex items-center">
                    <span class="w-16 font-bold text-zinc-600 dark:text-zinc-300 text-[11px]">{{ t('parserType') }}</span>
                    <div class="relative flex-1 min-w-0">
                      <select v-model="currentInstrument.parser.type" class="mac-select w-full font-mono text-[11px] bg-white dark:bg-[#1a1a1a] shadow-md border-black/10 dark:border-white/10 text-[#5856d6] h-7">
                        <option value="json_fields">{{ t('optJsonFields') }}</option>
                        <option value="sina_forex_text">{{ t('optSinaForex') }}</option>
                        <option value="sina_futures_text">{{ t('optSinaFutures') }}</option>
                        <option value="swissquote_bbo">{{ t('optSwissquote') }}</option>
                        <option value="gold_api_xau">{{ t('optGoldApi') }}</option>
                      </select>
                    </div>
                  </div>
                </div>

                <div v-if="currentInstrument.parser.type === 'json_fields'" class="space-y-0 text-[12px]">
                   <div class="mac-row py-2"><div class="flex items-center min-w-0"><span class="w-16 font-medium text-zinc-500 flex-none text-[11px]">{{ t('pricePath') }}</span><input v-model="currentInstrument.parser.price_path" class="mac-input flex-1 min-w-0 font-mono text-[#34c759] text-[11px] py-1" /></div></div>
                   <div class="mac-row py-2"><div class="flex items-center gap-2 min-w-0 w-full"><span class="w-16 font-medium flex-none text-zinc-500 text-[11px]">{{ t('bidAskPath') }}</span><input v-model="currentInstrument.parser.bid_path" placeholder="Bid" class="mac-input flex-1 min-w-0 font-mono text-[11px] py-1" /><input v-model="currentInstrument.parser.ask_path" placeholder="Ask" class="mac-input flex-1 min-w-0 font-mono text-[11px] py-1" /></div></div>
                   <div class="mac-row py-2"><div class="flex items-center gap-2 min-w-0 w-full"><span class="w-16 font-medium flex-none text-zinc-500 text-[11px]">{{ t('timePath') }}</span><input v-model="currentInstrument.parser.quote_time_path" placeholder="Quote Time" class="mac-input flex-1 min-w-0 font-mono text-[11px] py-1" /><input v-model="currentInstrument.parser.update_time_path" placeholder="Update Time" class="mac-input flex-1 min-w-0 font-mono text-[11px] py-1" /></div></div>
                </div>
                <div v-else class="p-6 bg-[#007aff]/5 dark:bg-[#007aff]/10 backdrop-blur-sm text-center">
                  <div class="text-[13px] font-semibold text-[#007aff]">{{ t('builtinParser') }}</div><div class="text-[11px] text-[#007aff]/70 mt-1 max-w-[240px] mx-auto">{{ t('builtinParserDesc') }}</div>
                </div>
              </div>
            </div>
            
          </div>
        </div>

        <!-- 右半区：测试面板 -->
        <div v-show="activeTab !== 'global'" class="md:w-[45%] flex flex-col min-w-0 min-h-0 bg-white/60 dark:bg-[#1e1e1e]/80 backdrop-blur-3xl border border-white/60 dark:border-white/10 rounded-xl shadow-[0_8px_32px_rgba(0,0,0,0.04)] overflow-hidden relative">
           
           <div class="px-3 py-2 border-b border-black/5 dark:border-white/5 bg-white/40 dark:bg-black/30 flex items-center justify-between flex-none space-x-2">
              <input v-model="testSymbol" class="mac-input flex-1 min-w-0 font-mono text-[#5856d6] text-[12px] py-1 ring-1 ring-black/5" :placeholder="t('testSymbolPlaceholder')" />
              <div class="flex space-x-2">
                 <button v-show="testing" @click="cancelFetch" class="px-3 py-1.5 bg-[#ff3b30] hover:bg-[#ff3b30]/80 text-white rounded-[6px] font-bold text-[11px] shadow-sm transition-all flex items-center whitespace-nowrap">
                    <svg class="w-3.5 h-3.5 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 10l4 4m0-4l-4 4"></path></svg>
                    {{ t('stopRequest') }}
                 </button>
                 <button @click="doTestFetch" :disabled="testing" class="px-4 py-1.5 bg-[#007aff] hover:bg-[#0062cc] disabled:bg-gray-400 disabled:opacity-50 text-white rounded-[6px] font-bold text-[11px] shadow-sm transition-all flex items-center whitespace-nowrap">
                   <span v-if="testing" class="animate-spin h-3.5 w-3.5 mr-1.5 text-white"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg></span>
                   <svg v-else class="w-3.5 h-3.5 mr-1.5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                   {{ testing ? t('requesting') : t('sendRequest') }}
                 </button>
              </div>
           </div>

           <div class="flex items-end px-3 pt-3 border-b border-black/10 dark:border-white/10 flex-none space-x-4 bg-transparent">
              <button 
                @click="testTab = 'body'" 
                :class="testTab === 'body' ? 'border-[#007aff] text-[#007aff] dark:text-[#3a9bfb] font-bold' : 'border-transparent text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300 font-medium'"
                class="pb-2 border-b-2 transition-colors text-[12px] uppercase tracking-wide"
              >
                Response Body
              </button>
              <button 
                @click="testTab = 'headers'" 
                :class="testTab === 'headers' ? 'border-[#007aff] text-[#007aff] dark:text-[#3a9bfb] font-bold' : 'border-transparent text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300 font-medium'"
                class="pb-2 border-b-2 transition-colors text-[12px] uppercase tracking-wide"
              >
                Headers / Info
              </button>
           </div>
           
           <div class="flex-1 overflow-y-auto p-0 relative min-h-0 flex flex-col bg-white dark:bg-[#121212]">
             
             <div class="flex-1 flex flex-col items-center justify-center text-[12px] text-zinc-400 font-medium" v-if="!testResult && !testing">
               <svg class="w-12 h-12 mb-3 text-zinc-300 dark:text-zinc-700" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
               {{ t('testHint') }}
             </div>

             <div v-if="testResult" class="flex-1 flex flex-col min-h-0 animate-fade-in relative">
                
                <div class="absolute right-3 top-3 z-10 font-mono text-[10px] px-2 py-0.5 rounded-[4px] border"
                     :class="testResult.response && testResult.response.status >= 200 && testResult.response.status < 300 ? 'text-[#34c759] border-[#34c759]/30 bg-[#34c759]/5' : (testResult.error_msg ? 'text-[#ff3b30] border-[#ff3b30]/30 bg-[#ff3b30]/5' : 'text-[#ff9500] border-[#ff9500]/30 bg-[#ff9500]/5')">
                    Status: {{testResult.response ? testResult.response.status : (testResult.error_msg ? 'ERROR' : 'UNKNOWN')}}
                </div>

                <div v-show="testTab === 'body'" class="flex-1 min-h-0 overflow-auto p-4 pt-10 custom-scrollbar bg-transparent flex flex-col">
                    <div v-if="testResult.error_msg" class="text-[#ff3b30] font-mono text-[11px] leading-relaxed whitespace-pre-wrap break-all">{{ testResult.error_msg }}</div>
                    <div v-else-if="testResult.response" class="flex-1 min-h-0">
                       <div class="font-mono text-[11px] text-zinc-800 dark:text-[#d4d4d4] whitespace-pre-wrap break-all leading-relaxed" v-html="highlightJsonLightTheme(testResult.response.body)"></div>
                    </div>
                </div>

                <div v-show="testTab === 'headers'" class="flex-1 min-h-0 overflow-auto p-4 custom-scrollbar bg-transparent space-y-4">
                    <div v-if="testResult.request">
                       <h3 class="text-[10px] font-bold text-zinc-400 uppercase mb-2">{{ t('requestInfo') }}</h3>
                       <div class="font-mono text-[11px] text-[#007aff] mb-3 select-all bg-[#007aff]/5 p-2 rounded"><span class="font-bold text-black dark:text-white">{{testResult.request.method}}</span> {{testResult.request.url}}</div>
                       <table class="w-full text-left font-mono text-[11px] border-collapse" v-if="testResult.request.headers && Object.keys(testResult.request.headers).length > 0">
                          <tr class="border-b border-black/5 dark:border-white/5" v-for="(v, k) in testResult.request.headers" :key="k">
                             <td class="py-1.5 pr-4 text-zinc-500 font-medium whitespace-nowrap">{{k}}</td>
                             <td class="py-1.5 text-zinc-800 dark:text-zinc-200 break-all">{{v}}</td>
                          </tr>
                       </table>
                       <div v-else class="text-[10px] text-zinc-400 italic font-mono">- No Req Headers -</div>
                       <div v-if="testResult.request.body" class="mt-3">
                          <h4 class="text-[9px] text-zinc-400 mb-1">{{ t('requestBody') }}</h4>
                          <div class="text-[11px] font-mono text-zinc-600 dark:text-zinc-400 bg-black/5 dark:bg-white/5 p-2 rounded whitespace-pre-wrap">{{testResult.request.body}}</div>
                       </div>
                    </div>
                    <div v-if="testResult.response && !testResult.error_msg" class="pt-2 border-t border-black/10 dark:border-white/10">
                       <h3 class="text-[10px] font-bold text-zinc-400 uppercase mb-2">{{ t('responseHeaders') }}</h3>
                       <table class="w-full text-left font-mono text-[11px] border-collapse" v-if="testResult.response.headers && Object.keys(testResult.response.headers).length > 0">
                          <tr class="border-b border-black/5 dark:border-white/5" v-for="(v, k) in testResult.response.headers" :key="k">
                             <td class="py-1.5 pr-4 text-zinc-500 font-medium whitespace-nowrap">{{k}}</td>
                             <td class="py-1.5 text-zinc-800 dark:text-zinc-200 break-all">{{v}}</td>
                          </tr>
                       </table>
                       <div v-else class="text-[10px] text-zinc-400 italic font-mono">- No Res Headers -</div>
                    </div>
                </div>

             </div>
           </div>
        </div>

      </div>
    </main>

    <div v-if="toastMsg" class="absolute bottom-6 left-1/2 transform -translate-x-1/2 bg-black/80 dark:bg-white/90 backdrop-blur-xl text-white dark:text-black px-4 py-2 rounded-xl shadow-[0_10px_40px_rgba(0,0,0,0.2)] flex items-center space-x-2 transition-all z-[100] animate-fade-up ring-1 ring-white/10 dark:ring-black/10">
      <div :class="toastType === 'success' ? 'bg-[#34c759]' : (toastType === 'warning' ? 'bg-[#ff9500]' : 'bg-[#ff3b30]')" class="w-4 h-4 rounded-full flex items-center justify-center text-white shadow-inner">
         <span v-if="toastType === 'success'" class="text-[10px] font-bold">✓</span>
         <span v-else-if="toastType === 'warning'" class="text-[10px] font-bold">!</span>
         <span v-else class="text-[10px] font-bold">✕</span>
      </div>
      <span class="font-medium tracking-wide text-[11px]">{{ toastMsg }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { t, currentLocale, setLocale, initLocale, type Locale } from './i18n';

interface Config { defaults: any; providers: any[]; }

const activeTab = ref('providers');
const testTab = ref('body');
const loading = ref(true);
const saving = ref(false);
const config = ref<Config>({ defaults: {}, providers: [] });
const jsonError = ref('');

// 语言绑定
const locale = ref<Locale>(currentLocale.value);
function onLocaleChange() {
  setLocale(locale.value);
}

// 手风琴折叠
const sections = ref({ headers: true, params: true, body: true });
function toggleSection(sec: 'headers'|'params'|'body') { sections.value[sec] = !sections.value[sec]; }

const selectedProviderIndex = ref(0);
const selectedParserProviderIndex = ref(0);
const selectedParserInstrumentIndex = ref(0);

const currentProvider = computed(() => activeTab.value === 'providers' ? config.value.providers[selectedProviderIndex.value] : config.value.providers[selectedParserProviderIndex.value]);
const currentParserProvider = computed(() => config.value.providers[selectedParserProviderIndex.value]);
const currentInstrument = computed(() => currentParserProvider.value?.instruments?.[selectedParserInstrumentIndex.value]);

const headersJsonStr = ref('{}');
const paramsJsonStr = ref('');
const bodyJsonStr = ref('');

const testSymbol = ref('default');
const testing = ref(false);
const testResult = ref<any>(null);

const toastMsg = ref('');
const toastType = ref<'success'|'error'|'warning'>('success');
let toastTimer: any = null;
const showToast = (msg: string, type: 'success'|'error'|'warning' = 'success') => {
  toastMsg.value = msg; toastType.value = type;
  if(toastTimer) clearTimeout(toastTimer);
  toastTimer = setTimeout(() => toastMsg.value = '', 2500);
};

const loadConfig = async () => {
  try {
    loading.value = true;
    config.value = await invoke('get_config');
    for (let p of config.value.providers) {
      if (!p.instruments) {
         p.instruments = [];
         if (p.groups) {
           for (let g of p.groups) {
             for (let inst of g.instruments) p.instruments.push({ ...inst, group: g.name });
           }
         }
      }
    }
    updateJsonStrings();
  } catch (err: any) {
    showToast("Failed to load: " + err, 'error');
  } finally {
    loading.value = false;
  }
};

const saveConfig = async () => {
  syncParams();
  if (jsonError.value) { showToast(t('saveFailJsonError'), 'error'); return; }
  try {
    saving.value = true;
    await invoke('save_config', { config: config.value });
    showToast(t('configSaved'));
  } catch(err: any) {
    showToast(t('saveFailed') + err, 'error');
  } finally {
    saving.value = false;
  }
};

watch(selectedProviderIndex, updateJsonStrings);
watch(activeTab, () => {
  updateJsonStrings();
  testSymbol.value = activeTab.value === 'parsers' && currentInstrument.value ? currentInstrument.value.symbol : 'default';
});

watch(selectedParserInstrumentIndex, () => {
   if (activeTab.value === 'parsers' && currentInstrument.value) {
       testSymbol.value = currentInstrument.value.symbol;
   }
});

function updateJsonStrings() {
  jsonError.value = '';
  const targetProvider = currentProvider.value; 
  if (!targetProvider) return;
  const req = targetProvider.request;
  headersJsonStr.value = JSON.stringify(req.headers || {}, null, 2);
  paramsJsonStr.value = req.params ? JSON.stringify(req.params, null, 2) : '';
  bodyJsonStr.value = req.body ? JSON.stringify(req.body, null, 2) : '';
}

function syncParams() {
  try {
    jsonError.value = '';
    const targetProvider = currentProvider.value; 
    if (!targetProvider) return;
    const req = targetProvider.request;
    
    if (headersJsonStr.value.trim()) {
        const p = JSON.parse(headersJsonStr.value);
        req.headers = p;
        headersJsonStr.value = JSON.stringify(p, null, 2);
    } else req.headers = {};

    if (paramsJsonStr.value.trim()) {
        const p = JSON.parse(paramsJsonStr.value);
        req.params = p;
        paramsJsonStr.value = JSON.stringify(p, null, 2);
    } else req.params = null;

    if (bodyJsonStr.value.trim()) {
        const p = JSON.parse(bodyJsonStr.value);
        req.body = p;
        bodyJsonStr.value = JSON.stringify(p, null, 2);
    } else req.body = null;
  } catch (err: any) {
    jsonError.value = "JSON Parse: " + err.message;
  }
}

let abortController: AbortController | null = null;

function cancelFetch() {
   if (testing.value) {
      if (abortController) abortController.abort();
      testing.value = false;
      showToast(t('requestCancelled'), "warning");
   }
}

async function doTestFetch() {
  const targetProvider = currentProvider.value;
  if (!targetProvider) return;
  syncParams();
  if (jsonError.value) {
     showToast(t('jsonCheckError'), "error");
     return;
  }
  
  testing.value = true;
  testResult.value = null; 
  testTab.value = 'body';
  abortController = new AbortController();
  
  try {
     const res: any = await Promise.race([
        invoke('test_fetch', { provider: targetProvider, symbol: testSymbol.value }),
        new Promise((_, reject) => {
           abortController?.signal.addEventListener('abort', () => reject(new Error('USER_ABORTED')));
        })
     ]);
     testResult.value = res;
  } catch (err: any) {
     if (err.message === 'USER_ABORTED') {
         // 用户主动取消
     } else {
         showToast(t('requestFailed') + err, "error");
         testResult.value = {
            request: { method: targetProvider.request.method, url: targetProvider.request.url_template.replace('{symbol}', testSymbol.value), headers: {}, body: '' },
            error_msg: t('connectionError') + err
         };
     }
  } finally {
     testing.value = false;
     abortController = null;
  }
}

function highlightJsonLightTheme(json: string) {
  if (!json) return '';
  let formatJson = json;
  try { formatJson = JSON.stringify(JSON.parse(json), null, 2); } 
  catch { return formatJson.replace(/&/g, '&amp;').replace(/</g, '&lt;'); }
  
  let formatted = formatJson.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
  return formatted.replace(/("(\\u[a-zA-Z0-9]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(true|false|null)\b|-?\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?)/g, function (match) {
        let cls = 'text-[#098658] dark:text-[#b5cea8]';
        if (/^"/.test(match)) {
            if (/:$/.test(match)) cls = 'text-[#0451a5] dark:text-[#9cdcfe] font-semibold';
            else cls = 'text-[#a31515] dark:text-[#ce9178] break-words';
        } else if (/true|false|null/.test(match)) cls = 'text-[#0000ff] dark:text-[#569cd6] font-bold';
        return '<span class="' + cls + '">' + match + '</span>';
  });
}

function addProvider() {
  config.value.providers.push({
    key: `new_provider_${Date.now()}`, label: t('newProviderLabel'),
    request: { method: "GET", url_template: "https://", headers: {}, timeout: 15, verify: false, impersonate: "chrome_122" },
    instruments: [{ label: "Default", symbol: "default", group: t('defaultGroup'), parser: { type: "json_fields" } }]
  });
  selectedProviderIndex.value = config.value.providers.length - 1;
}

function removeProvider() {
  config.value.providers.splice(selectedProviderIndex.value, 1);
  selectedProviderIndex.value = Math.max(0, selectedProviderIndex.value - 1);
}
function addInstrument() {
   currentParserProvider.value.instruments.push({
      label: t('newInstrumentLabel'), symbol: "SYMBOL", group: t('defaultGroup'), parser: { type: "json_fields" }
   });
   selectedParserInstrumentIndex.value = currentParserProvider.value.instruments.length - 1;
}
function removeInstrument() {
   currentParserProvider.value.instruments.splice(selectedParserInstrumentIndex.value, 1);
   selectedParserInstrumentIndex.value = Math.max(0, selectedParserInstrumentIndex.value - 1);
}

onMounted(() => {
  initLocale();
  locale.value = currentLocale.value;
  loadConfig();
});
</script>

<style>
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer components {
  .mac-card {
    @apply bg-white/60 dark:bg-black/20 backdrop-blur-xl rounded-[10px] shadow-sm ring-1 ring-black/5 dark:ring-white/10 overflow-hidden;
  }
  .mac-row {
    @apply px-4 py-[6px] border-b border-black/5 dark:border-white/5 last:border-b-0;
  }
  .mac-input {
    @apply appearance-none bg-white/60 dark:bg-black/40 backdrop-blur-md border border-black/10 dark:border-white/10 rounded-[6px] px-[8px] py-[3px] text-[12px] text-zinc-800 dark:text-zinc-200 outline-none focus:ring-[3px] focus:ring-[#007aff]/30 focus:border-[#007aff] transition-all shadow-[inset_0_1px_2px_rgba(0,0,0,0.04)];
  }
  .mac-select {
    @apply appearance-none bg-white/70 dark:bg-[#3a3a3c]/80 backdrop-blur-md border border-black/10 dark:border-white/10 rounded-[6px] px-[8px] py-[3px] text-[12px] text-zinc-800 dark:text-zinc-200 outline-none focus:ring-[3px] focus:ring-[#007aff]/30 focus:border-[#007aff] shadow-sm cursor-default;
    background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%23888' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
    background-repeat: no-repeat;
    background-position: right 8px center;
    background-size: 11px;
    padding-right: 28px;
  }
  .mac-icon-btn {
    @apply border px-[10px] py-[2px] rounded-[6px] shadow-sm text-[11px] font-bold transition-all cursor-default active:scale-[0.97];
  }
  .mac-textarea {
    @apply appearance-none bg-black/[0.03] dark:bg-white/[0.03] rounded-[6px] border border-transparent outline-none focus:bg-white/40 dark:focus:bg-black/40 text-zinc-800 dark:text-zinc-300 placeholder:text-zinc-400 dark:placeholder:text-zinc-600 focus:shadow-[inset_0_1px_2px_rgba(0,0,0,0.08)] transition-all;
  }
  .mac-checkbox {
    @apply appearance-none w-[14px] h-[14px] border border-black/20 dark:border-white/20 rounded-[3px] bg-white/50 dark:bg-black/50 checked:bg-[#007aff] checked:border-[#007aff] relative outline-none focus:ring-[3px] focus:ring-[#007aff]/30 shadow-sm cursor-default align-middle transition-all;
  }
  .mac-checkbox:checked::after {
    content: '';
    position: absolute;
    left: 4px;
    top: 1px;
    width: 4px;
    height: 8px;
    border: solid white;
    border-width: 0 2px 2px 0;
    transform: rotate(45deg);
  }
}

.pattern-bg {
  background-image: radial-gradient(rgba(0,0,0,0.1) 1px, transparent 1px);
  background-size: 8px 8px;
}
@media (prefers-color-scheme: dark) {
  .pattern-bg {
    background-image: radial-gradient(rgba(255,255,255,0.06) 1px, transparent 1px);
  }
}

.custom-scrollbar::-webkit-scrollbar { width: 6px; height: 6px; }
.custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.15);
  border: 1px solid transparent; background-clip: content-box; border-radius: 99px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover { background: rgba(0, 0, 0, 0.3); border: 1px solid transparent; background-clip: content-box; }
@media (prefers-color-scheme: dark) {
  .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.15); }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: rgba(255, 255, 255, 0.3); }
}

.animate-fade-in { animation: fadeIn 0.15s ease-out; }
.animate-fade-up { animation: fadeUp 0.25s cubic-bezier(0.16, 1, 0.3, 1); }
@keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }
@keyframes fadeUp { from { opacity: 0; transform: translate(-50%, 15px); scale: 0.95; } to { opacity: 1; transform: translate(-50%, 0); scale: 1; } }
</style>