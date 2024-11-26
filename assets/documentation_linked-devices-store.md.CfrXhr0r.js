import{_ as s,c as i,a2 as t,o as a}from"./chunks/framework.DhOvnmKX.js";const c=JSON.parse('{"title":"LinkedDevicesStore","description":"","frontmatter":{},"headers":[],"relativePath":"documentation/linked-devices-store.md","filePath":"documentation/linked-devices-store.md"}'),n={name:"documentation/linked-devices-store.md"};function r(o,e,l,h,k,p){return a(),i("div",null,e[0]||(e[0]=[t(`<h1 id="linkeddevicesstore" tabindex="-1">LinkedDevicesStore <a class="header-anchor" href="#linkeddevicesstore" aria-label="Permalink to &quot;LinkedDevicesStore&quot;">​</a></h1><p>The <code>LinkedDevicesStore</code> is a typescript class that contains <a href="https://www.npmjs.com/package/async-signals" target="_blank" rel="noreferrer">async signals</a>, which you can watch to get reactive updates in your elements.</p><div class="language-js vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang">js</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="--shiki-light:#D73A49;--shiki-dark:#F97583;">import</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;"> { LinkedDevicesStore, LinkedDevicesClient } </span><span style="--shiki-light:#D73A49;--shiki-dark:#F97583;">from</span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;"> &quot;@darksoil-studio/linked-devices&quot;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">;</span></span>
<span class="line"><span style="--shiki-light:#D73A49;--shiki-dark:#F97583;">const</span><span style="--shiki-light:#005CC5;--shiki-dark:#79B8FF;"> store</span><span style="--shiki-light:#D73A49;--shiki-dark:#F97583;"> =</span><span style="--shiki-light:#D73A49;--shiki-dark:#F97583;"> new</span><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;"> LinkedDevicesStore</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">(</span><span style="--shiki-light:#D73A49;--shiki-dark:#F97583;">new</span><span style="--shiki-light:#6F42C1;--shiki-dark:#B392F0;"> LinkedDevicesClient</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">(appClient, </span><span style="--shiki-light:#032F62;--shiki-dark:#9ECBFF;">&#39;my-role-name&#39;</span><span style="--shiki-light:#24292E;--shiki-dark:#E1E4E8;">));</span></span></code></pre></div><blockquote><p>Learn how to setup the <code>AppClient</code> object <a href="https://www.npmjs.com/package/@holochain/client" target="_blank" rel="noreferrer">here</a>.</p></blockquote><p>Learn more about the stores and how to integrate them in different frameworks <a href="https://darksoil.studio/tnesh-stack/guides/signals#Stores" target="_blank" rel="noreferrer">here</a>.</p>`,5)]))}const g=s(n,[["render",r]]);export{c as __pageData,g as default};