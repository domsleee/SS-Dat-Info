var jl=Object.defineProperty;var Kl=(n,t,e)=>t in n?jl(n,t,{enumerable:!0,configurable:!0,writable:!0,value:e}):n[t]=e;var Zl=(n,t)=>()=>(t||n((t={exports:{}}).exports,t),t.exports);var Qr=(n,t,e)=>Kl(n,typeof t!="symbol"?t+"":t,e);import"./modulepreload-polyfill-B5Qt9EMX.js";var Pv=Zl((fn,hn)=>{var ts={},Ri={},eo;function Jl(){if(eo)return Ri;eo=1,Ri.byteLength=a,Ri.toByteArray=u,Ri.fromByteArray=m;for(var n=[],t=[],e=typeof Uint8Array<"u"?Uint8Array:Array,i="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",r=0,s=i.length;r<s;++r)n[r]=i[r],t[i.charCodeAt(r)]=r;t[45]=62,t[95]=63;function o(_){var x=_.length;if(x%4>0)throw new Error("Invalid string. Length must be a multiple of 4");var y=_.indexOf("=");y===-1&&(y=x);var g=y===x?0:4-y%4;return[y,g]}function a(_){var x=o(_),y=x[0],g=x[1];return(y+g)*3/4-g}function l(_,x,y){return(x+y)*3/4-y}function u(_){var x,y=o(_),g=y[0],d=y[1],P=new e(l(_,g,d)),b=0,w=d>0?g-4:g,D;for(D=0;D<w;D+=4)x=t[_.charCodeAt(D)]<<18|t[_.charCodeAt(D+1)]<<12|t[_.charCodeAt(D+2)]<<6|t[_.charCodeAt(D+3)],P[b++]=x>>16&255,P[b++]=x>>8&255,P[b++]=x&255;return d===2&&(x=t[_.charCodeAt(D)]<<2|t[_.charCodeAt(D+1)]>>4,P[b++]=x&255),d===1&&(x=t[_.charCodeAt(D)]<<10|t[_.charCodeAt(D+1)]<<4|t[_.charCodeAt(D+2)]>>2,P[b++]=x>>8&255,P[b++]=x&255),P}function h(_){return n[_>>18&63]+n[_>>12&63]+n[_>>6&63]+n[_&63]}function p(_,x,y){for(var g,d=[],P=x;P<y;P+=3)g=(_[P]<<16&16711680)+(_[P+1]<<8&65280)+(_[P+2]&255),d.push(h(g));return d.join("")}function m(_){for(var x,y=_.length,g=y%3,d=[],P=16383,b=0,w=y-g;b<w;b+=P)d.push(p(_,b,b+P>w?w:b+P));return g===1?(x=_[y-1],d.push(n[x>>2]+n[x<<4&63]+"==")):g===2&&(x=(_[y-2]<<8)+_[y-1],d.push(n[x>>10]+n[x>>4&63]+n[x<<2&63]+"=")),d.join("")}return Ri}var Qi={};/*! ieee754. BSD-3-Clause License. Feross Aboukhadijeh <https://feross.org/opensource> */var no;function Ql(){return no||(no=1,Qi.read=function(n,t,e,i,r){var s,o,a=r*8-i-1,l=(1<<a)-1,u=l>>1,h=-7,p=e?r-1:0,m=e?-1:1,_=n[t+p];for(p+=m,s=_&(1<<-h)-1,_>>=-h,h+=a;h>0;s=s*256+n[t+p],p+=m,h-=8);for(o=s&(1<<-h)-1,s>>=-h,h+=i;h>0;o=o*256+n[t+p],p+=m,h-=8);if(s===0)s=1-u;else{if(s===l)return o?NaN:(_?-1:1)*(1/0);o=o+Math.pow(2,i),s=s-u}return(_?-1:1)*o*Math.pow(2,s-i)},Qi.write=function(n,t,e,i,r,s){var o,a,l,u=s*8-r-1,h=(1<<u)-1,p=h>>1,m=r===23?Math.pow(2,-24)-Math.pow(2,-77):0,_=i?0:s-1,x=i?1:-1,y=t<0||t===0&&1/t<0?1:0;for(t=Math.abs(t),isNaN(t)||t===1/0?(a=isNaN(t)?1:0,o=h):(o=Math.floor(Math.log(t)/Math.LN2),t*(l=Math.pow(2,-o))<1&&(o--,l*=2),o+p>=1?t+=m/l:t+=m*Math.pow(2,1-p),t*l>=2&&(o++,l/=2),o+p>=h?(a=0,o=h):o+p>=1?(a=(t*l-1)*Math.pow(2,r),o=o+p):(a=t*Math.pow(2,p-1)*Math.pow(2,r),o=0));r>=8;n[e+_]=a&255,_+=x,a/=256,r-=8);for(o=o<<r|a,u+=r;u>0;n[e+_]=o&255,_+=x,o/=256,u-=8);n[e+_-x]|=y*128}),Qi}/*!
 * The buffer module from node.js, for the browser.
 *
 * @author   Feross Aboukhadijeh <https://feross.org>
 * @license  MIT
 */var io;function tu(){return io||(io=1,function(n){const t=Jl(),e=Ql(),i=typeof Symbol=="function"&&typeof Symbol.for=="function"?Symbol.for("nodejs.util.inspect.custom"):null;n.Buffer=a,n.SlowBuffer=P,n.INSPECT_MAX_BYTES=50;const r=2147483647;n.kMaxLength=r,a.TYPED_ARRAY_SUPPORT=s(),!a.TYPED_ARRAY_SUPPORT&&typeof console<"u"&&typeof console.error=="function"&&console.error("This browser lacks typed array (Uint8Array) support which is required by `buffer` v5.x. Use `buffer` v4.x if you require old browser support.");function s(){try{const v=new Uint8Array(1),c={foo:function(){return 42}};return Object.setPrototypeOf(c,Uint8Array.prototype),Object.setPrototypeOf(v,c),v.foo()===42}catch{return!1}}Object.defineProperty(a.prototype,"parent",{enumerable:!0,get:function(){if(a.isBuffer(this))return this.buffer}}),Object.defineProperty(a.prototype,"offset",{enumerable:!0,get:function(){if(a.isBuffer(this))return this.byteOffset}});function o(v){if(v>r)throw new RangeError('The value "'+v+'" is invalid for option "size"');const c=new Uint8Array(v);return Object.setPrototypeOf(c,a.prototype),c}function a(v,c,f){if(typeof v=="number"){if(typeof c=="string")throw new TypeError('The "string" argument must be of type string. Received type number');return p(v)}return l(v,c,f)}a.poolSize=8192;function l(v,c,f){if(typeof v=="string")return m(v,c);if(ArrayBuffer.isView(v))return x(v);if(v==null)throw new TypeError("The first argument must be one of type string, Buffer, ArrayBuffer, Array, or Array-like Object. Received type "+typeof v);if(S(v,ArrayBuffer)||v&&S(v.buffer,ArrayBuffer)||typeof SharedArrayBuffer<"u"&&(S(v,SharedArrayBuffer)||v&&S(v.buffer,SharedArrayBuffer)))return y(v,c,f);if(typeof v=="number")throw new TypeError('The "value" argument must not be of type number. Received type number');const E=v.valueOf&&v.valueOf();if(E!=null&&E!==v)return a.from(E,c,f);const C=g(v);if(C)return C;if(typeof Symbol<"u"&&Symbol.toPrimitive!=null&&typeof v[Symbol.toPrimitive]=="function")return a.from(v[Symbol.toPrimitive]("string"),c,f);throw new TypeError("The first argument must be one of type string, Buffer, ArrayBuffer, Array, or Array-like Object. Received type "+typeof v)}a.from=function(v,c,f){return l(v,c,f)},Object.setPrototypeOf(a.prototype,Uint8Array.prototype),Object.setPrototypeOf(a,Uint8Array);function u(v){if(typeof v!="number")throw new TypeError('"size" argument must be of type number');if(v<0)throw new RangeError('The value "'+v+'" is invalid for option "size"')}function h(v,c,f){return u(v),v<=0?o(v):c!==void 0?typeof f=="string"?o(v).fill(c,f):o(v).fill(c):o(v)}a.alloc=function(v,c,f){return h(v,c,f)};function p(v){return u(v),o(v<0?0:d(v)|0)}a.allocUnsafe=function(v){return p(v)},a.allocUnsafeSlow=function(v){return p(v)};function m(v,c){if((typeof c!="string"||c==="")&&(c="utf8"),!a.isEncoding(c))throw new TypeError("Unknown encoding: "+c);const f=b(v,c)|0;let E=o(f);const C=E.write(v,c);return C!==f&&(E=E.slice(0,C)),E}function _(v){const c=v.length<0?0:d(v.length)|0,f=o(c);for(let E=0;E<c;E+=1)f[E]=v[E]&255;return f}function x(v){if(S(v,Uint8Array)){const c=new Uint8Array(v);return y(c.buffer,c.byteOffset,c.byteLength)}return _(v)}function y(v,c,f){if(c<0||v.byteLength<c)throw new RangeError('"offset" is outside of buffer bounds');if(v.byteLength<c+(f||0))throw new RangeError('"length" is outside of buffer bounds');let E;return c===void 0&&f===void 0?E=new Uint8Array(v):f===void 0?E=new Uint8Array(v,c):E=new Uint8Array(v,c,f),Object.setPrototypeOf(E,a.prototype),E}function g(v){if(a.isBuffer(v)){const c=d(v.length)|0,f=o(c);return f.length===0||v.copy(f,0,0,c),f}if(v.length!==void 0)return typeof v.length!="number"||G(v.length)?o(0):_(v);if(v.type==="Buffer"&&Array.isArray(v.data))return _(v.data)}function d(v){if(v>=r)throw new RangeError("Attempt to allocate Buffer larger than maximum size: 0x"+r.toString(16)+" bytes");return v|0}function P(v){return+v!=v&&(v=0),a.alloc(+v)}a.isBuffer=function(c){return c!=null&&c._isBuffer===!0&&c!==a.prototype},a.compare=function(c,f){if(S(c,Uint8Array)&&(c=a.from(c,c.offset,c.byteLength)),S(f,Uint8Array)&&(f=a.from(f,f.offset,f.byteLength)),!a.isBuffer(c)||!a.isBuffer(f))throw new TypeError('The "buf1", "buf2" arguments must be one of type Buffer or Uint8Array');if(c===f)return 0;let E=c.length,C=f.length;for(let F=0,V=Math.min(E,C);F<V;++F)if(c[F]!==f[F]){E=c[F],C=f[F];break}return E<C?-1:C<E?1:0},a.isEncoding=function(c){switch(String(c).toLowerCase()){case"hex":case"utf8":case"utf-8":case"ascii":case"latin1":case"binary":case"base64":case"ucs2":case"ucs-2":case"utf16le":case"utf-16le":return!0;default:return!1}},a.concat=function(c,f){if(!Array.isArray(c))throw new TypeError('"list" argument must be an Array of Buffers');if(c.length===0)return a.alloc(0);let E;if(f===void 0)for(f=0,E=0;E<c.length;++E)f+=c[E].length;const C=a.allocUnsafe(f);let F=0;for(E=0;E<c.length;++E){let V=c[E];if(S(V,Uint8Array))F+V.length>C.length?(a.isBuffer(V)||(V=a.from(V)),V.copy(C,F)):Uint8Array.prototype.set.call(C,V,F);else if(a.isBuffer(V))V.copy(C,F);else throw new TypeError('"list" argument must be an Array of Buffers');F+=V.length}return C};function b(v,c){if(a.isBuffer(v))return v.length;if(ArrayBuffer.isView(v)||S(v,ArrayBuffer))return v.byteLength;if(typeof v!="string")throw new TypeError('The "string" argument must be one of type string, Buffer, or ArrayBuffer. Received type '+typeof v);const f=v.length,E=arguments.length>2&&arguments[2]===!0;if(!E&&f===0)return 0;let C=!1;for(;;)switch(c){case"ascii":case"latin1":case"binary":return f;case"utf8":case"utf-8":return Ot(v).length;case"ucs2":case"ucs-2":case"utf16le":case"utf-16le":return f*2;case"hex":return f>>>1;case"base64":return Et(v).length;default:if(C)return E?-1:Ot(v).length;c=(""+c).toLowerCase(),C=!0}}a.byteLength=b;function w(v,c,f){let E=!1;if((c===void 0||c<0)&&(c=0),c>this.length||((f===void 0||f>this.length)&&(f=this.length),f<=0)||(f>>>=0,c>>>=0,f<=c))return"";for(v||(v="utf8");;)switch(v){case"hex":return ft(this,c,f);case"utf8":case"utf-8":return K(this,c,f);case"ascii":return at(this,c,f);case"latin1":case"binary":return Y(this,c,f);case"base64":return W(this,c,f);case"ucs2":case"ucs-2":case"utf16le":case"utf-16le":return gt(this,c,f);default:if(E)throw new TypeError("Unknown encoding: "+v);v=(v+"").toLowerCase(),E=!0}}a.prototype._isBuffer=!0;function D(v,c,f){const E=v[c];v[c]=v[f],v[f]=E}a.prototype.swap16=function(){const c=this.length;if(c%2!==0)throw new RangeError("Buffer size must be a multiple of 16-bits");for(let f=0;f<c;f+=2)D(this,f,f+1);return this},a.prototype.swap32=function(){const c=this.length;if(c%4!==0)throw new RangeError("Buffer size must be a multiple of 32-bits");for(let f=0;f<c;f+=4)D(this,f,f+3),D(this,f+1,f+2);return this},a.prototype.swap64=function(){const c=this.length;if(c%8!==0)throw new RangeError("Buffer size must be a multiple of 64-bits");for(let f=0;f<c;f+=8)D(this,f,f+7),D(this,f+1,f+6),D(this,f+2,f+5),D(this,f+3,f+4);return this},a.prototype.toString=function(){const c=this.length;return c===0?"":arguments.length===0?K(this,0,c):w.apply(this,arguments)},a.prototype.toLocaleString=a.prototype.toString,a.prototype.equals=function(c){if(!a.isBuffer(c))throw new TypeError("Argument must be a Buffer");return this===c?!0:a.compare(this,c)===0},a.prototype.inspect=function(){let c="";const f=n.INSPECT_MAX_BYTES;return c=this.toString("hex",0,f).replace(/(.{2})/g,"$1 ").trim(),this.length>f&&(c+=" ... "),"<Buffer "+c+">"},i&&(a.prototype[i]=a.prototype.inspect),a.prototype.compare=function(c,f,E,C,F){if(S(c,Uint8Array)&&(c=a.from(c,c.offset,c.byteLength)),!a.isBuffer(c))throw new TypeError('The "target" argument must be one of type Buffer or Uint8Array. Received type '+typeof c);if(f===void 0&&(f=0),E===void 0&&(E=c?c.length:0),C===void 0&&(C=0),F===void 0&&(F=this.length),f<0||E>c.length||C<0||F>this.length)throw new RangeError("out of range index");if(C>=F&&f>=E)return 0;if(C>=F)return-1;if(f>=E)return 1;if(f>>>=0,E>>>=0,C>>>=0,F>>>=0,this===c)return 0;let V=F-C,ot=E-f;const st=Math.min(V,ot),Mt=this.slice(C,F),vt=c.slice(f,E);for(let At=0;At<st;++At)if(Mt[At]!==vt[At]){V=Mt[At],ot=vt[At];break}return V<ot?-1:ot<V?1:0};function L(v,c,f,E,C){if(v.length===0)return-1;if(typeof f=="string"?(E=f,f=0):f>2147483647?f=2147483647:f<-2147483648&&(f=-2147483648),f=+f,G(f)&&(f=C?0:v.length-1),f<0&&(f=v.length+f),f>=v.length){if(C)return-1;f=v.length-1}else if(f<0)if(C)f=0;else return-1;if(typeof c=="string"&&(c=a.from(c,E)),a.isBuffer(c))return c.length===0?-1:I(v,c,f,E,C);if(typeof c=="number")return c=c&255,typeof Uint8Array.prototype.indexOf=="function"?C?Uint8Array.prototype.indexOf.call(v,c,f):Uint8Array.prototype.lastIndexOf.call(v,c,f):I(v,[c],f,E,C);throw new TypeError("val must be string, number or Buffer")}function I(v,c,f,E,C){let F=1,V=v.length,ot=c.length;if(E!==void 0&&(E=String(E).toLowerCase(),E==="ucs2"||E==="ucs-2"||E==="utf16le"||E==="utf-16le")){if(v.length<2||c.length<2)return-1;F=2,V/=2,ot/=2,f/=2}function st(vt,At){return F===1?vt[At]:vt.readUInt16BE(At*F)}let Mt;if(C){let vt=-1;for(Mt=f;Mt<V;Mt++)if(st(v,Mt)===st(c,vt===-1?0:Mt-vt)){if(vt===-1&&(vt=Mt),Mt-vt+1===ot)return vt*F}else vt!==-1&&(Mt-=Mt-vt),vt=-1}else for(f+ot>V&&(f=V-ot),Mt=f;Mt>=0;Mt--){let vt=!0;for(let At=0;At<ot;At++)if(st(v,Mt+At)!==st(c,At)){vt=!1;break}if(vt)return Mt}return-1}a.prototype.includes=function(c,f,E){return this.indexOf(c,f,E)!==-1},a.prototype.indexOf=function(c,f,E){return L(this,c,f,E,!0)},a.prototype.lastIndexOf=function(c,f,E){return L(this,c,f,E,!1)};function N(v,c,f,E){f=Number(f)||0;const C=v.length-f;E?(E=Number(E),E>C&&(E=C)):E=C;const F=c.length;E>F/2&&(E=F/2);let V;for(V=0;V<E;++V){const ot=parseInt(c.substr(V*2,2),16);if(G(ot))return V;v[f+V]=ot}return V}function A(v,c,f,E){return R(Ot(c,v.length-f),v,f,E)}function T(v,c,f,E){return R(wt(c),v,f,E)}function O(v,c,f,E){return R(Et(c),v,f,E)}function $(v,c,f,E){return R(Zt(c,v.length-f),v,f,E)}a.prototype.write=function(c,f,E,C){if(f===void 0)C="utf8",E=this.length,f=0;else if(E===void 0&&typeof f=="string")C=f,E=this.length,f=0;else if(isFinite(f))f=f>>>0,isFinite(E)?(E=E>>>0,C===void 0&&(C="utf8")):(C=E,E=void 0);else throw new Error("Buffer.write(string, encoding, offset[, length]) is no longer supported");const F=this.length-f;if((E===void 0||E>F)&&(E=F),c.length>0&&(E<0||f<0)||f>this.length)throw new RangeError("Attempt to write outside buffer bounds");C||(C="utf8");let V=!1;for(;;)switch(C){case"hex":return N(this,c,f,E);case"utf8":case"utf-8":return A(this,c,f,E);case"ascii":case"latin1":case"binary":return T(this,c,f,E);case"base64":return O(this,c,f,E);case"ucs2":case"ucs-2":case"utf16le":case"utf-16le":return $(this,c,f,E);default:if(V)throw new TypeError("Unknown encoding: "+C);C=(""+C).toLowerCase(),V=!0}},a.prototype.toJSON=function(){return{type:"Buffer",data:Array.prototype.slice.call(this._arr||this,0)}};function W(v,c,f){return c===0&&f===v.length?t.fromByteArray(v):t.fromByteArray(v.slice(c,f))}function K(v,c,f){f=Math.min(v.length,f);const E=[];let C=c;for(;C<f;){const F=v[C];let V=null,ot=F>239?4:F>223?3:F>191?2:1;if(C+ot<=f){let st,Mt,vt,At;switch(ot){case 1:F<128&&(V=F);break;case 2:st=v[C+1],(st&192)===128&&(At=(F&31)<<6|st&63,At>127&&(V=At));break;case 3:st=v[C+1],Mt=v[C+2],(st&192)===128&&(Mt&192)===128&&(At=(F&15)<<12|(st&63)<<6|Mt&63,At>2047&&(At<55296||At>57343)&&(V=At));break;case 4:st=v[C+1],Mt=v[C+2],vt=v[C+3],(st&192)===128&&(Mt&192)===128&&(vt&192)===128&&(At=(F&15)<<18|(st&63)<<12|(Mt&63)<<6|vt&63,At>65535&&At<1114112&&(V=At))}}V===null?(V=65533,ot=1):V>65535&&(V-=65536,E.push(V>>>10&1023|55296),V=56320|V&1023),E.push(V),C+=ot}return Z(E)}const it=4096;function Z(v){const c=v.length;if(c<=it)return String.fromCharCode.apply(String,v);let f="",E=0;for(;E<c;)f+=String.fromCharCode.apply(String,v.slice(E,E+=it));return f}function at(v,c,f){let E="";f=Math.min(v.length,f);for(let C=c;C<f;++C)E+=String.fromCharCode(v[C]&127);return E}function Y(v,c,f){let E="";f=Math.min(v.length,f);for(let C=c;C<f;++C)E+=String.fromCharCode(v[C]);return E}function ft(v,c,f){const E=v.length;(!c||c<0)&&(c=0),(!f||f<0||f>E)&&(f=E);let C="";for(let F=c;F<f;++F)C+=tt[v[F]];return C}function gt(v,c,f){const E=v.slice(c,f);let C="";for(let F=0;F<E.length-1;F+=2)C+=String.fromCharCode(E[F]+E[F+1]*256);return C}a.prototype.slice=function(c,f){const E=this.length;c=~~c,f=f===void 0?E:~~f,c<0?(c+=E,c<0&&(c=0)):c>E&&(c=E),f<0?(f+=E,f<0&&(f=0)):f>E&&(f=E),f<c&&(f=c);const C=this.subarray(c,f);return Object.setPrototypeOf(C,a.prototype),C};function ht(v,c,f){if(v%1!==0||v<0)throw new RangeError("offset is not uint");if(v+c>f)throw new RangeError("Trying to access beyond buffer length")}a.prototype.readUintLE=a.prototype.readUIntLE=function(c,f,E){c=c>>>0,f=f>>>0,E||ht(c,f,this.length);let C=this[c],F=1,V=0;for(;++V<f&&(F*=256);)C+=this[c+V]*F;return C},a.prototype.readUintBE=a.prototype.readUIntBE=function(c,f,E){c=c>>>0,f=f>>>0,E||ht(c,f,this.length);let C=this[c+--f],F=1;for(;f>0&&(F*=256);)C+=this[c+--f]*F;return C},a.prototype.readUint8=a.prototype.readUInt8=function(c,f){return c=c>>>0,f||ht(c,1,this.length),this[c]},a.prototype.readUint16LE=a.prototype.readUInt16LE=function(c,f){return c=c>>>0,f||ht(c,2,this.length),this[c]|this[c+1]<<8},a.prototype.readUint16BE=a.prototype.readUInt16BE=function(c,f){return c=c>>>0,f||ht(c,2,this.length),this[c]<<8|this[c+1]},a.prototype.readUint32LE=a.prototype.readUInt32LE=function(c,f){return c=c>>>0,f||ht(c,4,this.length),(this[c]|this[c+1]<<8|this[c+2]<<16)+this[c+3]*16777216},a.prototype.readUint32BE=a.prototype.readUInt32BE=function(c,f){return c=c>>>0,f||ht(c,4,this.length),this[c]*16777216+(this[c+1]<<16|this[c+2]<<8|this[c+3])},a.prototype.readBigUInt64LE=et(function(c){c=c>>>0,It(c,"offset");const f=this[c],E=this[c+7];(f===void 0||E===void 0)&&U(c,this.length-8);const C=f+this[++c]*2**8+this[++c]*2**16+this[++c]*2**24,F=this[++c]+this[++c]*2**8+this[++c]*2**16+E*2**24;return BigInt(C)+(BigInt(F)<<BigInt(32))}),a.prototype.readBigUInt64BE=et(function(c){c=c>>>0,It(c,"offset");const f=this[c],E=this[c+7];(f===void 0||E===void 0)&&U(c,this.length-8);const C=f*2**24+this[++c]*2**16+this[++c]*2**8+this[++c],F=this[++c]*2**24+this[++c]*2**16+this[++c]*2**8+E;return(BigInt(C)<<BigInt(32))+BigInt(F)}),a.prototype.readIntLE=function(c,f,E){c=c>>>0,f=f>>>0,E||ht(c,f,this.length);let C=this[c],F=1,V=0;for(;++V<f&&(F*=256);)C+=this[c+V]*F;return F*=128,C>=F&&(C-=Math.pow(2,8*f)),C},a.prototype.readIntBE=function(c,f,E){c=c>>>0,f=f>>>0,E||ht(c,f,this.length);let C=f,F=1,V=this[c+--C];for(;C>0&&(F*=256);)V+=this[c+--C]*F;return F*=128,V>=F&&(V-=Math.pow(2,8*f)),V},a.prototype.readInt8=function(c,f){return c=c>>>0,f||ht(c,1,this.length),this[c]&128?(255-this[c]+1)*-1:this[c]},a.prototype.readInt16LE=function(c,f){c=c>>>0,f||ht(c,2,this.length);const E=this[c]|this[c+1]<<8;return E&32768?E|4294901760:E},a.prototype.readInt16BE=function(c,f){c=c>>>0,f||ht(c,2,this.length);const E=this[c+1]|this[c]<<8;return E&32768?E|4294901760:E},a.prototype.readInt32LE=function(c,f){return c=c>>>0,f||ht(c,4,this.length),this[c]|this[c+1]<<8|this[c+2]<<16|this[c+3]<<24},a.prototype.readInt32BE=function(c,f){return c=c>>>0,f||ht(c,4,this.length),this[c]<<24|this[c+1]<<16|this[c+2]<<8|this[c+3]},a.prototype.readBigInt64LE=et(function(c){c=c>>>0,It(c,"offset");const f=this[c],E=this[c+7];(f===void 0||E===void 0)&&U(c,this.length-8);const C=this[c+4]+this[c+5]*2**8+this[c+6]*2**16+(E<<24);return(BigInt(C)<<BigInt(32))+BigInt(f+this[++c]*2**8+this[++c]*2**16+this[++c]*2**24)}),a.prototype.readBigInt64BE=et(function(c){c=c>>>0,It(c,"offset");const f=this[c],E=this[c+7];(f===void 0||E===void 0)&&U(c,this.length-8);const C=(f<<24)+this[++c]*2**16+this[++c]*2**8+this[++c];return(BigInt(C)<<BigInt(32))+BigInt(this[++c]*2**24+this[++c]*2**16+this[++c]*2**8+E)}),a.prototype.readFloatLE=function(c,f){return c=c>>>0,f||ht(c,4,this.length),e.read(this,c,!0,23,4)},a.prototype.readFloatBE=function(c,f){return c=c>>>0,f||ht(c,4,this.length),e.read(this,c,!1,23,4)},a.prototype.readDoubleLE=function(c,f){return c=c>>>0,f||ht(c,8,this.length),e.read(this,c,!0,52,8)},a.prototype.readDoubleBE=function(c,f){return c=c>>>0,f||ht(c,8,this.length),e.read(this,c,!1,52,8)};function bt(v,c,f,E,C,F){if(!a.isBuffer(v))throw new TypeError('"buffer" argument must be a Buffer instance');if(c>C||c<F)throw new RangeError('"value" argument is out of bounds');if(f+E>v.length)throw new RangeError("Index out of range")}a.prototype.writeUintLE=a.prototype.writeUIntLE=function(c,f,E,C){if(c=+c,f=f>>>0,E=E>>>0,!C){const ot=Math.pow(2,8*E)-1;bt(this,c,f,E,ot,0)}let F=1,V=0;for(this[f]=c&255;++V<E&&(F*=256);)this[f+V]=c/F&255;return f+E},a.prototype.writeUintBE=a.prototype.writeUIntBE=function(c,f,E,C){if(c=+c,f=f>>>0,E=E>>>0,!C){const ot=Math.pow(2,8*E)-1;bt(this,c,f,E,ot,0)}let F=E-1,V=1;for(this[f+F]=c&255;--F>=0&&(V*=256);)this[f+F]=c/V&255;return f+E},a.prototype.writeUint8=a.prototype.writeUInt8=function(c,f,E){return c=+c,f=f>>>0,E||bt(this,c,f,1,255,0),this[f]=c&255,f+1},a.prototype.writeUint16LE=a.prototype.writeUInt16LE=function(c,f,E){return c=+c,f=f>>>0,E||bt(this,c,f,2,65535,0),this[f]=c&255,this[f+1]=c>>>8,f+2},a.prototype.writeUint16BE=a.prototype.writeUInt16BE=function(c,f,E){return c=+c,f=f>>>0,E||bt(this,c,f,2,65535,0),this[f]=c>>>8,this[f+1]=c&255,f+2},a.prototype.writeUint32LE=a.prototype.writeUInt32LE=function(c,f,E){return c=+c,f=f>>>0,E||bt(this,c,f,4,4294967295,0),this[f+3]=c>>>24,this[f+2]=c>>>16,this[f+1]=c>>>8,this[f]=c&255,f+4},a.prototype.writeUint32BE=a.prototype.writeUInt32BE=function(c,f,E){return c=+c,f=f>>>0,E||bt(this,c,f,4,4294967295,0),this[f]=c>>>24,this[f+1]=c>>>16,this[f+2]=c>>>8,this[f+3]=c&255,f+4};function qt(v,c,f,E,C){Jt(c,E,C,v,f,7);let F=Number(c&BigInt(4294967295));v[f++]=F,F=F>>8,v[f++]=F,F=F>>8,v[f++]=F,F=F>>8,v[f++]=F;let V=Number(c>>BigInt(32)&BigInt(4294967295));return v[f++]=V,V=V>>8,v[f++]=V,V=V>>8,v[f++]=V,V=V>>8,v[f++]=V,f}function Q(v,c,f,E,C){Jt(c,E,C,v,f,7);let F=Number(c&BigInt(4294967295));v[f+7]=F,F=F>>8,v[f+6]=F,F=F>>8,v[f+5]=F,F=F>>8,v[f+4]=F;let V=Number(c>>BigInt(32)&BigInt(4294967295));return v[f+3]=V,V=V>>8,v[f+2]=V,V=V>>8,v[f+1]=V,V=V>>8,v[f]=V,f+8}a.prototype.writeBigUInt64LE=et(function(c,f=0){return qt(this,c,f,BigInt(0),BigInt("0xffffffffffffffff"))}),a.prototype.writeBigUInt64BE=et(function(c,f=0){return Q(this,c,f,BigInt(0),BigInt("0xffffffffffffffff"))}),a.prototype.writeIntLE=function(c,f,E,C){if(c=+c,f=f>>>0,!C){const st=Math.pow(2,8*E-1);bt(this,c,f,E,st-1,-st)}let F=0,V=1,ot=0;for(this[f]=c&255;++F<E&&(V*=256);)c<0&&ot===0&&this[f+F-1]!==0&&(ot=1),this[f+F]=(c/V>>0)-ot&255;return f+E},a.prototype.writeIntBE=function(c,f,E,C){if(c=+c,f=f>>>0,!C){const st=Math.pow(2,8*E-1);bt(this,c,f,E,st-1,-st)}let F=E-1,V=1,ot=0;for(this[f+F]=c&255;--F>=0&&(V*=256);)c<0&&ot===0&&this[f+F+1]!==0&&(ot=1),this[f+F]=(c/V>>0)-ot&255;return f+E},a.prototype.writeInt8=function(c,f,E){return c=+c,f=f>>>0,E||bt(this,c,f,1,127,-128),c<0&&(c=255+c+1),this[f]=c&255,f+1},a.prototype.writeInt16LE=function(c,f,E){return c=+c,f=f>>>0,E||bt(this,c,f,2,32767,-32768),this[f]=c&255,this[f+1]=c>>>8,f+2},a.prototype.writeInt16BE=function(c,f,E){return c=+c,f=f>>>0,E||bt(this,c,f,2,32767,-32768),this[f]=c>>>8,this[f+1]=c&255,f+2},a.prototype.writeInt32LE=function(c,f,E){return c=+c,f=f>>>0,E||bt(this,c,f,4,2147483647,-2147483648),this[f]=c&255,this[f+1]=c>>>8,this[f+2]=c>>>16,this[f+3]=c>>>24,f+4},a.prototype.writeInt32BE=function(c,f,E){return c=+c,f=f>>>0,E||bt(this,c,f,4,2147483647,-2147483648),c<0&&(c=4294967295+c+1),this[f]=c>>>24,this[f+1]=c>>>16,this[f+2]=c>>>8,this[f+3]=c&255,f+4},a.prototype.writeBigInt64LE=et(function(c,f=0){return qt(this,c,f,-BigInt("0x8000000000000000"),BigInt("0x7fffffffffffffff"))}),a.prototype.writeBigInt64BE=et(function(c,f=0){return Q(this,c,f,-BigInt("0x8000000000000000"),BigInt("0x7fffffffffffffff"))});function ct(v,c,f,E,C,F){if(f+E>v.length)throw new RangeError("Index out of range");if(f<0)throw new RangeError("Index out of range")}function St(v,c,f,E,C){return c=+c,f=f>>>0,C||ct(v,c,f,4),e.write(v,c,f,E,23,4),f+4}a.prototype.writeFloatLE=function(c,f,E){return St(this,c,f,!0,E)},a.prototype.writeFloatBE=function(c,f,E){return St(this,c,f,!1,E)};function ut(v,c,f,E,C){return c=+c,f=f>>>0,C||ct(v,c,f,8),e.write(v,c,f,E,52,8),f+8}a.prototype.writeDoubleLE=function(c,f,E){return ut(this,c,f,!0,E)},a.prototype.writeDoubleBE=function(c,f,E){return ut(this,c,f,!1,E)},a.prototype.copy=function(c,f,E,C){if(!a.isBuffer(c))throw new TypeError("argument should be a Buffer");if(E||(E=0),!C&&C!==0&&(C=this.length),f>=c.length&&(f=c.length),f||(f=0),C>0&&C<E&&(C=E),C===E||c.length===0||this.length===0)return 0;if(f<0)throw new RangeError("targetStart out of bounds");if(E<0||E>=this.length)throw new RangeError("Index out of range");if(C<0)throw new RangeError("sourceEnd out of bounds");C>this.length&&(C=this.length),c.length-f<C-E&&(C=c.length-f+E);const F=C-E;return this===c&&typeof Uint8Array.prototype.copyWithin=="function"?this.copyWithin(f,E,C):Uint8Array.prototype.set.call(c,this.subarray(E,C),f),F},a.prototype.fill=function(c,f,E,C){if(typeof c=="string"){if(typeof f=="string"?(C=f,f=0,E=this.length):typeof E=="string"&&(C=E,E=this.length),C!==void 0&&typeof C!="string")throw new TypeError("encoding must be a string");if(typeof C=="string"&&!a.isEncoding(C))throw new TypeError("Unknown encoding: "+C);if(c.length===1){const V=c.charCodeAt(0);(C==="utf8"&&V<128||C==="latin1")&&(c=V)}}else typeof c=="number"?c=c&255:typeof c=="boolean"&&(c=Number(c));if(f<0||this.length<f||this.length<E)throw new RangeError("Out of range index");if(E<=f)return this;f=f>>>0,E=E===void 0?this.length:E>>>0,c||(c=0);let F;if(typeof c=="number")for(F=f;F<E;++F)this[F]=c;else{const V=a.isBuffer(c)?c:a.from(c,C),ot=V.length;if(ot===0)throw new TypeError('The value "'+c+'" is invalid for argument "value"');for(F=0;F<E-f;++F)this[F+f]=V[F%ot]}return this};const Tt={};function Bt(v,c,f){Tt[v]=class extends f{constructor(){super(),Object.defineProperty(this,"message",{value:c.apply(this,arguments),writable:!0,configurable:!0}),this.name=`${this.name} [${v}]`,this.stack,delete this.name}get code(){return v}set code(C){Object.defineProperty(this,"code",{configurable:!0,enumerable:!0,value:C,writable:!0})}toString(){return`${this.name} [${v}]: ${this.message}`}}}Bt("ERR_BUFFER_OUT_OF_BOUNDS",function(v){return v?`${v} is outside of buffer bounds`:"Attempt to access memory outside buffer bounds"},RangeError),Bt("ERR_INVALID_ARG_TYPE",function(v,c){return`The "${v}" argument must be of type number. Received type ${typeof c}`},TypeError),Bt("ERR_OUT_OF_RANGE",function(v,c,f){let E=`The value of "${v}" is out of range.`,C=f;return Number.isInteger(f)&&Math.abs(f)>2**32?C=Pt(String(f)):typeof f=="bigint"&&(C=String(f),(f>BigInt(2)**BigInt(32)||f<-(BigInt(2)**BigInt(32)))&&(C=Pt(C)),C+="n"),E+=` It must be ${c}. Received ${C}`,E},RangeError);function Pt(v){let c="",f=v.length;const E=v[0]==="-"?1:0;for(;f>=E+4;f-=3)c=`_${v.slice(f-3,f)}${c}`;return`${v.slice(0,f)}${c}`}function ne(v,c,f){It(c,"offset"),(v[c]===void 0||v[c+f]===void 0)&&U(c,v.length-(f+1))}function Jt(v,c,f,E,C,F){if(v>f||v<c){const V=typeof c=="bigint"?"n":"";let ot;throw c===0||c===BigInt(0)?ot=`>= 0${V} and < 2${V} ** ${(F+1)*8}${V}`:ot=`>= -(2${V} ** ${(F+1)*8-1}${V}) and < 2 ** ${(F+1)*8-1}${V}`,new Tt.ERR_OUT_OF_RANGE("value",ot,v)}ne(E,C,F)}function It(v,c){if(typeof v!="number")throw new Tt.ERR_INVALID_ARG_TYPE(c,"number",v)}function U(v,c,f){throw Math.floor(v)!==v?(It(v,f),new Tt.ERR_OUT_OF_RANGE("offset","an integer",v)):c<0?new Tt.ERR_BUFFER_OUT_OF_BOUNDS:new Tt.ERR_OUT_OF_RANGE("offset",`>= 0 and <= ${c}`,v)}const Ee=/[^+/0-9A-Za-z-_]/g;function zt(v){if(v=v.split("=")[0],v=v.trim().replace(Ee,""),v.length<2)return"";for(;v.length%4!==0;)v=v+"=";return v}function Ot(v,c){c=c||1/0;let f;const E=v.length;let C=null;const F=[];for(let V=0;V<E;++V){if(f=v.charCodeAt(V),f>55295&&f<57344){if(!C){if(f>56319){(c-=3)>-1&&F.push(239,191,189);continue}else if(V+1===E){(c-=3)>-1&&F.push(239,191,189);continue}C=f;continue}if(f<56320){(c-=3)>-1&&F.push(239,191,189),C=f;continue}f=(C-55296<<10|f-56320)+65536}else C&&(c-=3)>-1&&F.push(239,191,189);if(C=null,f<128){if((c-=1)<0)break;F.push(f)}else if(f<2048){if((c-=2)<0)break;F.push(f>>6|192,f&63|128)}else if(f<65536){if((c-=3)<0)break;F.push(f>>12|224,f>>6&63|128,f&63|128)}else if(f<1114112){if((c-=4)<0)break;F.push(f>>18|240,f>>12&63|128,f>>6&63|128,f&63|128)}else throw new Error("Invalid code point")}return F}function wt(v){const c=[];for(let f=0;f<v.length;++f)c.push(v.charCodeAt(f)&255);return c}function Zt(v,c){let f,E,C;const F=[];for(let V=0;V<v.length&&!((c-=2)<0);++V)f=v.charCodeAt(V),E=f>>8,C=f%256,F.push(C),F.push(E);return F}function Et(v){return t.toByteArray(zt(v))}function R(v,c,f,E){let C;for(C=0;C<E&&!(C+f>=c.length||C>=v.length);++C)c[C+f]=v[C];return C}function S(v,c){return v instanceof c||v!=null&&v.constructor!=null&&v.constructor.name!=null&&v.constructor.name===c.name}function G(v){return v!==v}const tt=function(){const v="0123456789abcdef",c=new Array(256);for(let f=0;f<16;++f){const E=f*16;for(let C=0;C<16;++C)c[E+C]=v[f]+v[C]}return c}();function et(v){return typeof BigInt>"u"?J:v}function J(){throw new Error("BigInt not supported")}}(ts)),ts}var eu=tu();window.Buffer=eu.Buffer;/**
 * @license
 * Copyright 2010-2025 Three.js Authors
 * SPDX-License-Identifier: MIT
 */const La="176",nu=0,ro=1,iu=2,Uc=1,ru=2,an=3,Pn=0,Ae=1,ve=2,Cn=0,_i=1,so=2,ao=3,oo=4,su=5,Wn=100,au=101,ou=102,cu=103,lu=104,uu=200,fu=201,hu=202,du=203,zs=204,ks=205,pu=206,mu=207,_u=208,gu=209,xu=210,vu=211,yu=212,Su=213,Mu=214,Hs=0,Vs=1,Gs=2,xi=3,Ws=4,Xs=5,qs=6,$s=7,Fc=0,Eu=1,Tu=2,Rn=0,wu=1,Au=2,bu=3,Cu=4,Ru=5,Pu=6,Lu=7,Nc=300,vi=301,yi=302,Ys=303,js=304,Gr=306,Ks=1e3,qn=1001,Zs=1002,qe=1003,Du=1004,tr=1005,Ke=1006,es=1007,$n=1008,Je=1009,Oc=1010,Bc=1011,ki=1012,Da=1013,Yn=1014,cn=1015,qi=1016,Ia=1017,Ua=1018,Hi=1020,zc=35902,kc=1021,Hc=1022,We=1023,Vi=1026,Gi=1027,Vc=1028,Fa=1029,Gc=1030,Na=1031,Oa=1033,Cr=33776,Rr=33777,Pr=33778,Lr=33779,Js=35840,Qs=35841,ta=35842,ea=35843,na=36196,ia=37492,ra=37496,sa=37808,aa=37809,oa=37810,ca=37811,la=37812,ua=37813,fa=37814,ha=37815,da=37816,pa=37817,ma=37818,_a=37819,ga=37820,xa=37821,Dr=36492,va=36494,ya=36495,Wc=36283,Sa=36284,Ma=36285,Ea=36286,Iu=3200,Uu=3201,Ba=0,Fu=1,An="",Fe="srgb",Si="srgb-linear",Nr="linear",Kt="srgb",ei=7680,co=519,Nu=512,Ou=513,Bu=514,Xc=515,zu=516,ku=517,Hu=518,Vu=519,lo=35044,uo="300 es",ln=2e3,Or=2001;class Ti{addEventListener(t,e){this._listeners===void 0&&(this._listeners={});const i=this._listeners;i[t]===void 0&&(i[t]=[]),i[t].indexOf(e)===-1&&i[t].push(e)}hasEventListener(t,e){const i=this._listeners;return i===void 0?!1:i[t]!==void 0&&i[t].indexOf(e)!==-1}removeEventListener(t,e){const i=this._listeners;if(i===void 0)return;const r=i[t];if(r!==void 0){const s=r.indexOf(e);s!==-1&&r.splice(s,1)}}dispatchEvent(t){const e=this._listeners;if(e===void 0)return;const i=e[t.type];if(i!==void 0){t.target=this;const r=i.slice(0);for(let s=0,o=r.length;s<o;s++)r[s].call(this,t);t.target=null}}}const me=["00","01","02","03","04","05","06","07","08","09","0a","0b","0c","0d","0e","0f","10","11","12","13","14","15","16","17","18","19","1a","1b","1c","1d","1e","1f","20","21","22","23","24","25","26","27","28","29","2a","2b","2c","2d","2e","2f","30","31","32","33","34","35","36","37","38","39","3a","3b","3c","3d","3e","3f","40","41","42","43","44","45","46","47","48","49","4a","4b","4c","4d","4e","4f","50","51","52","53","54","55","56","57","58","59","5a","5b","5c","5d","5e","5f","60","61","62","63","64","65","66","67","68","69","6a","6b","6c","6d","6e","6f","70","71","72","73","74","75","76","77","78","79","7a","7b","7c","7d","7e","7f","80","81","82","83","84","85","86","87","88","89","8a","8b","8c","8d","8e","8f","90","91","92","93","94","95","96","97","98","99","9a","9b","9c","9d","9e","9f","a0","a1","a2","a3","a4","a5","a6","a7","a8","a9","aa","ab","ac","ad","ae","af","b0","b1","b2","b3","b4","b5","b6","b7","b8","b9","ba","bb","bc","bd","be","bf","c0","c1","c2","c3","c4","c5","c6","c7","c8","c9","ca","cb","cc","cd","ce","cf","d0","d1","d2","d3","d4","d5","d6","d7","d8","d9","da","db","dc","dd","de","df","e0","e1","e2","e3","e4","e5","e6","e7","e8","e9","ea","eb","ec","ed","ee","ef","f0","f1","f2","f3","f4","f5","f6","f7","f8","f9","fa","fb","fc","fd","fe","ff"],ns=Math.PI/180,Ta=180/Math.PI;function $i(){const n=Math.random()*4294967295|0,t=Math.random()*4294967295|0,e=Math.random()*4294967295|0,i=Math.random()*4294967295|0;return(me[n&255]+me[n>>8&255]+me[n>>16&255]+me[n>>24&255]+"-"+me[t&255]+me[t>>8&255]+"-"+me[t>>16&15|64]+me[t>>24&255]+"-"+me[e&63|128]+me[e>>8&255]+"-"+me[e>>16&255]+me[e>>24&255]+me[i&255]+me[i>>8&255]+me[i>>16&255]+me[i>>24&255]).toLowerCase()}function kt(n,t,e){return Math.max(t,Math.min(e,n))}function Gu(n,t){return(n%t+t)%t}function is(n,t,e){return(1-e)*n+e*t}function Pi(n,t){switch(t.constructor){case Float32Array:return n;case Uint32Array:return n/4294967295;case Uint16Array:return n/65535;case Uint8Array:return n/255;case Int32Array:return Math.max(n/2147483647,-1);case Int16Array:return Math.max(n/32767,-1);case Int8Array:return Math.max(n/127,-1);default:throw new Error("Invalid component type.")}}function we(n,t){switch(t.constructor){case Float32Array:return n;case Uint32Array:return Math.round(n*4294967295);case Uint16Array:return Math.round(n*65535);case Uint8Array:return Math.round(n*255);case Int32Array:return Math.round(n*2147483647);case Int16Array:return Math.round(n*32767);case Int8Array:return Math.round(n*127);default:throw new Error("Invalid component type.")}}class Xt{constructor(t=0,e=0){Xt.prototype.isVector2=!0,this.x=t,this.y=e}get width(){return this.x}set width(t){this.x=t}get height(){return this.y}set height(t){this.y=t}set(t,e){return this.x=t,this.y=e,this}setScalar(t){return this.x=t,this.y=t,this}setX(t){return this.x=t,this}setY(t){return this.y=t,this}setComponent(t,e){switch(t){case 0:this.x=e;break;case 1:this.y=e;break;default:throw new Error("index is out of range: "+t)}return this}getComponent(t){switch(t){case 0:return this.x;case 1:return this.y;default:throw new Error("index is out of range: "+t)}}clone(){return new this.constructor(this.x,this.y)}copy(t){return this.x=t.x,this.y=t.y,this}add(t){return this.x+=t.x,this.y+=t.y,this}addScalar(t){return this.x+=t,this.y+=t,this}addVectors(t,e){return this.x=t.x+e.x,this.y=t.y+e.y,this}addScaledVector(t,e){return this.x+=t.x*e,this.y+=t.y*e,this}sub(t){return this.x-=t.x,this.y-=t.y,this}subScalar(t){return this.x-=t,this.y-=t,this}subVectors(t,e){return this.x=t.x-e.x,this.y=t.y-e.y,this}multiply(t){return this.x*=t.x,this.y*=t.y,this}multiplyScalar(t){return this.x*=t,this.y*=t,this}divide(t){return this.x/=t.x,this.y/=t.y,this}divideScalar(t){return this.multiplyScalar(1/t)}applyMatrix3(t){const e=this.x,i=this.y,r=t.elements;return this.x=r[0]*e+r[3]*i+r[6],this.y=r[1]*e+r[4]*i+r[7],this}min(t){return this.x=Math.min(this.x,t.x),this.y=Math.min(this.y,t.y),this}max(t){return this.x=Math.max(this.x,t.x),this.y=Math.max(this.y,t.y),this}clamp(t,e){return this.x=kt(this.x,t.x,e.x),this.y=kt(this.y,t.y,e.y),this}clampScalar(t,e){return this.x=kt(this.x,t,e),this.y=kt(this.y,t,e),this}clampLength(t,e){const i=this.length();return this.divideScalar(i||1).multiplyScalar(kt(i,t,e))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this}negate(){return this.x=-this.x,this.y=-this.y,this}dot(t){return this.x*t.x+this.y*t.y}cross(t){return this.x*t.y-this.y*t.x}lengthSq(){return this.x*this.x+this.y*this.y}length(){return Math.sqrt(this.x*this.x+this.y*this.y)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)}normalize(){return this.divideScalar(this.length()||1)}angle(){return Math.atan2(-this.y,-this.x)+Math.PI}angleTo(t){const e=Math.sqrt(this.lengthSq()*t.lengthSq());if(e===0)return Math.PI/2;const i=this.dot(t)/e;return Math.acos(kt(i,-1,1))}distanceTo(t){return Math.sqrt(this.distanceToSquared(t))}distanceToSquared(t){const e=this.x-t.x,i=this.y-t.y;return e*e+i*i}manhattanDistanceTo(t){return Math.abs(this.x-t.x)+Math.abs(this.y-t.y)}setLength(t){return this.normalize().multiplyScalar(t)}lerp(t,e){return this.x+=(t.x-this.x)*e,this.y+=(t.y-this.y)*e,this}lerpVectors(t,e,i){return this.x=t.x+(e.x-t.x)*i,this.y=t.y+(e.y-t.y)*i,this}equals(t){return t.x===this.x&&t.y===this.y}fromArray(t,e=0){return this.x=t[e],this.y=t[e+1],this}toArray(t=[],e=0){return t[e]=this.x,t[e+1]=this.y,t}fromBufferAttribute(t,e){return this.x=t.getX(e),this.y=t.getY(e),this}rotateAround(t,e){const i=Math.cos(e),r=Math.sin(e),s=this.x-t.x,o=this.y-t.y;return this.x=s*i-o*r+t.x,this.y=s*r+o*i+t.y,this}random(){return this.x=Math.random(),this.y=Math.random(),this}*[Symbol.iterator](){yield this.x,yield this.y}}class Ft{constructor(t,e,i,r,s,o,a,l,u){Ft.prototype.isMatrix3=!0,this.elements=[1,0,0,0,1,0,0,0,1],t!==void 0&&this.set(t,e,i,r,s,o,a,l,u)}set(t,e,i,r,s,o,a,l,u){const h=this.elements;return h[0]=t,h[1]=r,h[2]=a,h[3]=e,h[4]=s,h[5]=l,h[6]=i,h[7]=o,h[8]=u,this}identity(){return this.set(1,0,0,0,1,0,0,0,1),this}copy(t){const e=this.elements,i=t.elements;return e[0]=i[0],e[1]=i[1],e[2]=i[2],e[3]=i[3],e[4]=i[4],e[5]=i[5],e[6]=i[6],e[7]=i[7],e[8]=i[8],this}extractBasis(t,e,i){return t.setFromMatrix3Column(this,0),e.setFromMatrix3Column(this,1),i.setFromMatrix3Column(this,2),this}setFromMatrix4(t){const e=t.elements;return this.set(e[0],e[4],e[8],e[1],e[5],e[9],e[2],e[6],e[10]),this}multiply(t){return this.multiplyMatrices(this,t)}premultiply(t){return this.multiplyMatrices(t,this)}multiplyMatrices(t,e){const i=t.elements,r=e.elements,s=this.elements,o=i[0],a=i[3],l=i[6],u=i[1],h=i[4],p=i[7],m=i[2],_=i[5],x=i[8],y=r[0],g=r[3],d=r[6],P=r[1],b=r[4],w=r[7],D=r[2],L=r[5],I=r[8];return s[0]=o*y+a*P+l*D,s[3]=o*g+a*b+l*L,s[6]=o*d+a*w+l*I,s[1]=u*y+h*P+p*D,s[4]=u*g+h*b+p*L,s[7]=u*d+h*w+p*I,s[2]=m*y+_*P+x*D,s[5]=m*g+_*b+x*L,s[8]=m*d+_*w+x*I,this}multiplyScalar(t){const e=this.elements;return e[0]*=t,e[3]*=t,e[6]*=t,e[1]*=t,e[4]*=t,e[7]*=t,e[2]*=t,e[5]*=t,e[8]*=t,this}determinant(){const t=this.elements,e=t[0],i=t[1],r=t[2],s=t[3],o=t[4],a=t[5],l=t[6],u=t[7],h=t[8];return e*o*h-e*a*u-i*s*h+i*a*l+r*s*u-r*o*l}invert(){const t=this.elements,e=t[0],i=t[1],r=t[2],s=t[3],o=t[4],a=t[5],l=t[6],u=t[7],h=t[8],p=h*o-a*u,m=a*l-h*s,_=u*s-o*l,x=e*p+i*m+r*_;if(x===0)return this.set(0,0,0,0,0,0,0,0,0);const y=1/x;return t[0]=p*y,t[1]=(r*u-h*i)*y,t[2]=(a*i-r*o)*y,t[3]=m*y,t[4]=(h*e-r*l)*y,t[5]=(r*s-a*e)*y,t[6]=_*y,t[7]=(i*l-u*e)*y,t[8]=(o*e-i*s)*y,this}transpose(){let t;const e=this.elements;return t=e[1],e[1]=e[3],e[3]=t,t=e[2],e[2]=e[6],e[6]=t,t=e[5],e[5]=e[7],e[7]=t,this}getNormalMatrix(t){return this.setFromMatrix4(t).invert().transpose()}transposeIntoArray(t){const e=this.elements;return t[0]=e[0],t[1]=e[3],t[2]=e[6],t[3]=e[1],t[4]=e[4],t[5]=e[7],t[6]=e[2],t[7]=e[5],t[8]=e[8],this}setUvTransform(t,e,i,r,s,o,a){const l=Math.cos(s),u=Math.sin(s);return this.set(i*l,i*u,-i*(l*o+u*a)+o+t,-r*u,r*l,-r*(-u*o+l*a)+a+e,0,0,1),this}scale(t,e){return this.premultiply(rs.makeScale(t,e)),this}rotate(t){return this.premultiply(rs.makeRotation(-t)),this}translate(t,e){return this.premultiply(rs.makeTranslation(t,e)),this}makeTranslation(t,e){return t.isVector2?this.set(1,0,t.x,0,1,t.y,0,0,1):this.set(1,0,t,0,1,e,0,0,1),this}makeRotation(t){const e=Math.cos(t),i=Math.sin(t);return this.set(e,-i,0,i,e,0,0,0,1),this}makeScale(t,e){return this.set(t,0,0,0,e,0,0,0,1),this}equals(t){const e=this.elements,i=t.elements;for(let r=0;r<9;r++)if(e[r]!==i[r])return!1;return!0}fromArray(t,e=0){for(let i=0;i<9;i++)this.elements[i]=t[i+e];return this}toArray(t=[],e=0){const i=this.elements;return t[e]=i[0],t[e+1]=i[1],t[e+2]=i[2],t[e+3]=i[3],t[e+4]=i[4],t[e+5]=i[5],t[e+6]=i[6],t[e+7]=i[7],t[e+8]=i[8],t}clone(){return new this.constructor().fromArray(this.elements)}}const rs=new Ft;function qc(n){for(let t=n.length-1;t>=0;--t)if(n[t]>=65535)return!0;return!1}function Br(n){return document.createElementNS("http://www.w3.org/1999/xhtml",n)}function Wu(){const n=Br("canvas");return n.style.display="block",n}const fo={};function Ir(n){n in fo||(fo[n]=!0,console.warn(n))}function Xu(n,t,e){return new Promise(function(i,r){function s(){switch(n.clientWaitSync(t,n.SYNC_FLUSH_COMMANDS_BIT,0)){case n.WAIT_FAILED:r();break;case n.TIMEOUT_EXPIRED:setTimeout(s,e);break;default:i()}}setTimeout(s,e)})}function qu(n){const t=n.elements;t[2]=.5*t[2]+.5*t[3],t[6]=.5*t[6]+.5*t[7],t[10]=.5*t[10]+.5*t[11],t[14]=.5*t[14]+.5*t[15]}function $u(n){const t=n.elements;t[11]===-1?(t[10]=-t[10]-1,t[14]=-t[14]):(t[10]=-t[10],t[14]=-t[14]+1)}const ho=new Ft().set(.4123908,.3575843,.1804808,.212639,.7151687,.0721923,.0193308,.1191948,.9505322),po=new Ft().set(3.2409699,-1.5373832,-.4986108,-.9692436,1.8759675,.0415551,.0556301,-.203977,1.0569715);function Yu(){const n={enabled:!0,workingColorSpace:Si,spaces:{},convert:function(r,s,o){return this.enabled===!1||s===o||!s||!o||(this.spaces[s].transfer===Kt&&(r.r=dn(r.r),r.g=dn(r.g),r.b=dn(r.b)),this.spaces[s].primaries!==this.spaces[o].primaries&&(r.applyMatrix3(this.spaces[s].toXYZ),r.applyMatrix3(this.spaces[o].fromXYZ)),this.spaces[o].transfer===Kt&&(r.r=gi(r.r),r.g=gi(r.g),r.b=gi(r.b))),r},fromWorkingColorSpace:function(r,s){return this.convert(r,this.workingColorSpace,s)},toWorkingColorSpace:function(r,s){return this.convert(r,s,this.workingColorSpace)},getPrimaries:function(r){return this.spaces[r].primaries},getTransfer:function(r){return r===An?Nr:this.spaces[r].transfer},getLuminanceCoefficients:function(r,s=this.workingColorSpace){return r.fromArray(this.spaces[s].luminanceCoefficients)},define:function(r){Object.assign(this.spaces,r)},_getMatrix:function(r,s,o){return r.copy(this.spaces[s].toXYZ).multiply(this.spaces[o].fromXYZ)},_getDrawingBufferColorSpace:function(r){return this.spaces[r].outputColorSpaceConfig.drawingBufferColorSpace},_getUnpackColorSpace:function(r=this.workingColorSpace){return this.spaces[r].workingColorSpaceConfig.unpackColorSpace}},t=[.64,.33,.3,.6,.15,.06],e=[.2126,.7152,.0722],i=[.3127,.329];return n.define({[Si]:{primaries:t,whitePoint:i,transfer:Nr,toXYZ:ho,fromXYZ:po,luminanceCoefficients:e,workingColorSpaceConfig:{unpackColorSpace:Fe},outputColorSpaceConfig:{drawingBufferColorSpace:Fe}},[Fe]:{primaries:t,whitePoint:i,transfer:Kt,toXYZ:ho,fromXYZ:po,luminanceCoefficients:e,outputColorSpaceConfig:{drawingBufferColorSpace:Fe}}}),n}const Gt=Yu();function dn(n){return n<.04045?n*.0773993808:Math.pow(n*.9478672986+.0521327014,2.4)}function gi(n){return n<.0031308?n*12.92:1.055*Math.pow(n,.41666)-.055}let ni;class ju{static getDataURL(t,e="image/png"){if(/^data:/i.test(t.src)||typeof HTMLCanvasElement>"u")return t.src;let i;if(t instanceof HTMLCanvasElement)i=t;else{ni===void 0&&(ni=Br("canvas")),ni.width=t.width,ni.height=t.height;const r=ni.getContext("2d");t instanceof ImageData?r.putImageData(t,0,0):r.drawImage(t,0,0,t.width,t.height),i=ni}return i.toDataURL(e)}static sRGBToLinear(t){if(typeof HTMLImageElement<"u"&&t instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&t instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&t instanceof ImageBitmap){const e=Br("canvas");e.width=t.width,e.height=t.height;const i=e.getContext("2d");i.drawImage(t,0,0,t.width,t.height);const r=i.getImageData(0,0,t.width,t.height),s=r.data;for(let o=0;o<s.length;o++)s[o]=dn(s[o]/255)*255;return i.putImageData(r,0,0),e}else if(t.data){const e=t.data.slice(0);for(let i=0;i<e.length;i++)e instanceof Uint8Array||e instanceof Uint8ClampedArray?e[i]=Math.floor(dn(e[i]/255)*255):e[i]=dn(e[i]);return{data:e,width:t.width,height:t.height}}else return console.warn("THREE.ImageUtils.sRGBToLinear(): Unsupported image type. No color space conversion applied."),t}}let Ku=0;class za{constructor(t=null){this.isSource=!0,Object.defineProperty(this,"id",{value:Ku++}),this.uuid=$i(),this.data=t,this.dataReady=!0,this.version=0}set needsUpdate(t){t===!0&&this.version++}toJSON(t){const e=t===void 0||typeof t=="string";if(!e&&t.images[this.uuid]!==void 0)return t.images[this.uuid];const i={uuid:this.uuid,url:""},r=this.data;if(r!==null){let s;if(Array.isArray(r)){s=[];for(let o=0,a=r.length;o<a;o++)r[o].isDataTexture?s.push(ss(r[o].image)):s.push(ss(r[o]))}else s=ss(r);i.url=s}return e||(t.images[this.uuid]=i),i}}function ss(n){return typeof HTMLImageElement<"u"&&n instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&n instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&n instanceof ImageBitmap?ju.getDataURL(n):n.data?{data:Array.from(n.data),width:n.width,height:n.height,type:n.data.constructor.name}:(console.warn("THREE.Texture: Unable to serialize Texture."),{})}let Zu=0;class be extends Ti{constructor(t=be.DEFAULT_IMAGE,e=be.DEFAULT_MAPPING,i=qn,r=qn,s=Ke,o=$n,a=We,l=Je,u=be.DEFAULT_ANISOTROPY,h=An){super(),this.isTexture=!0,Object.defineProperty(this,"id",{value:Zu++}),this.uuid=$i(),this.name="",this.source=new za(t),this.mipmaps=[],this.mapping=e,this.channel=0,this.wrapS=i,this.wrapT=r,this.magFilter=s,this.minFilter=o,this.anisotropy=u,this.format=a,this.internalFormat=null,this.type=l,this.offset=new Xt(0,0),this.repeat=new Xt(1,1),this.center=new Xt(0,0),this.rotation=0,this.matrixAutoUpdate=!0,this.matrix=new Ft,this.generateMipmaps=!0,this.premultiplyAlpha=!1,this.flipY=!0,this.unpackAlignment=4,this.colorSpace=h,this.userData={},this.version=0,this.onUpdate=null,this.renderTarget=null,this.isRenderTargetTexture=!1,this.isTextureArray=!1,this.pmremVersion=0}get image(){return this.source.data}set image(t=null){this.source.data=t}updateMatrix(){this.matrix.setUvTransform(this.offset.x,this.offset.y,this.repeat.x,this.repeat.y,this.rotation,this.center.x,this.center.y)}clone(){return new this.constructor().copy(this)}copy(t){return this.name=t.name,this.source=t.source,this.mipmaps=t.mipmaps.slice(0),this.mapping=t.mapping,this.channel=t.channel,this.wrapS=t.wrapS,this.wrapT=t.wrapT,this.magFilter=t.magFilter,this.minFilter=t.minFilter,this.anisotropy=t.anisotropy,this.format=t.format,this.internalFormat=t.internalFormat,this.type=t.type,this.offset.copy(t.offset),this.repeat.copy(t.repeat),this.center.copy(t.center),this.rotation=t.rotation,this.matrixAutoUpdate=t.matrixAutoUpdate,this.matrix.copy(t.matrix),this.generateMipmaps=t.generateMipmaps,this.premultiplyAlpha=t.premultiplyAlpha,this.flipY=t.flipY,this.unpackAlignment=t.unpackAlignment,this.colorSpace=t.colorSpace,this.renderTarget=t.renderTarget,this.isRenderTargetTexture=t.isRenderTargetTexture,this.isTextureArray=t.isTextureArray,this.userData=JSON.parse(JSON.stringify(t.userData)),this.needsUpdate=!0,this}toJSON(t){const e=t===void 0||typeof t=="string";if(!e&&t.textures[this.uuid]!==void 0)return t.textures[this.uuid];const i={metadata:{version:4.6,type:"Texture",generator:"Texture.toJSON"},uuid:this.uuid,name:this.name,image:this.source.toJSON(t).uuid,mapping:this.mapping,channel:this.channel,repeat:[this.repeat.x,this.repeat.y],offset:[this.offset.x,this.offset.y],center:[this.center.x,this.center.y],rotation:this.rotation,wrap:[this.wrapS,this.wrapT],format:this.format,internalFormat:this.internalFormat,type:this.type,colorSpace:this.colorSpace,minFilter:this.minFilter,magFilter:this.magFilter,anisotropy:this.anisotropy,flipY:this.flipY,generateMipmaps:this.generateMipmaps,premultiplyAlpha:this.premultiplyAlpha,unpackAlignment:this.unpackAlignment};return Object.keys(this.userData).length>0&&(i.userData=this.userData),e||(t.textures[this.uuid]=i),i}dispose(){this.dispatchEvent({type:"dispose"})}transformUv(t){if(this.mapping!==Nc)return t;if(t.applyMatrix3(this.matrix),t.x<0||t.x>1)switch(this.wrapS){case Ks:t.x=t.x-Math.floor(t.x);break;case qn:t.x=t.x<0?0:1;break;case Zs:Math.abs(Math.floor(t.x)%2)===1?t.x=Math.ceil(t.x)-t.x:t.x=t.x-Math.floor(t.x);break}if(t.y<0||t.y>1)switch(this.wrapT){case Ks:t.y=t.y-Math.floor(t.y);break;case qn:t.y=t.y<0?0:1;break;case Zs:Math.abs(Math.floor(t.y)%2)===1?t.y=Math.ceil(t.y)-t.y:t.y=t.y-Math.floor(t.y);break}return this.flipY&&(t.y=1-t.y),t}set needsUpdate(t){t===!0&&(this.version++,this.source.needsUpdate=!0)}set needsPMREMUpdate(t){t===!0&&this.pmremVersion++}}be.DEFAULT_IMAGE=null;be.DEFAULT_MAPPING=Nc;be.DEFAULT_ANISOTROPY=1;class se{constructor(t=0,e=0,i=0,r=1){se.prototype.isVector4=!0,this.x=t,this.y=e,this.z=i,this.w=r}get width(){return this.z}set width(t){this.z=t}get height(){return this.w}set height(t){this.w=t}set(t,e,i,r){return this.x=t,this.y=e,this.z=i,this.w=r,this}setScalar(t){return this.x=t,this.y=t,this.z=t,this.w=t,this}setX(t){return this.x=t,this}setY(t){return this.y=t,this}setZ(t){return this.z=t,this}setW(t){return this.w=t,this}setComponent(t,e){switch(t){case 0:this.x=e;break;case 1:this.y=e;break;case 2:this.z=e;break;case 3:this.w=e;break;default:throw new Error("index is out of range: "+t)}return this}getComponent(t){switch(t){case 0:return this.x;case 1:return this.y;case 2:return this.z;case 3:return this.w;default:throw new Error("index is out of range: "+t)}}clone(){return new this.constructor(this.x,this.y,this.z,this.w)}copy(t){return this.x=t.x,this.y=t.y,this.z=t.z,this.w=t.w!==void 0?t.w:1,this}add(t){return this.x+=t.x,this.y+=t.y,this.z+=t.z,this.w+=t.w,this}addScalar(t){return this.x+=t,this.y+=t,this.z+=t,this.w+=t,this}addVectors(t,e){return this.x=t.x+e.x,this.y=t.y+e.y,this.z=t.z+e.z,this.w=t.w+e.w,this}addScaledVector(t,e){return this.x+=t.x*e,this.y+=t.y*e,this.z+=t.z*e,this.w+=t.w*e,this}sub(t){return this.x-=t.x,this.y-=t.y,this.z-=t.z,this.w-=t.w,this}subScalar(t){return this.x-=t,this.y-=t,this.z-=t,this.w-=t,this}subVectors(t,e){return this.x=t.x-e.x,this.y=t.y-e.y,this.z=t.z-e.z,this.w=t.w-e.w,this}multiply(t){return this.x*=t.x,this.y*=t.y,this.z*=t.z,this.w*=t.w,this}multiplyScalar(t){return this.x*=t,this.y*=t,this.z*=t,this.w*=t,this}applyMatrix4(t){const e=this.x,i=this.y,r=this.z,s=this.w,o=t.elements;return this.x=o[0]*e+o[4]*i+o[8]*r+o[12]*s,this.y=o[1]*e+o[5]*i+o[9]*r+o[13]*s,this.z=o[2]*e+o[6]*i+o[10]*r+o[14]*s,this.w=o[3]*e+o[7]*i+o[11]*r+o[15]*s,this}divide(t){return this.x/=t.x,this.y/=t.y,this.z/=t.z,this.w/=t.w,this}divideScalar(t){return this.multiplyScalar(1/t)}setAxisAngleFromQuaternion(t){this.w=2*Math.acos(t.w);const e=Math.sqrt(1-t.w*t.w);return e<1e-4?(this.x=1,this.y=0,this.z=0):(this.x=t.x/e,this.y=t.y/e,this.z=t.z/e),this}setAxisAngleFromRotationMatrix(t){let e,i,r,s;const l=t.elements,u=l[0],h=l[4],p=l[8],m=l[1],_=l[5],x=l[9],y=l[2],g=l[6],d=l[10];if(Math.abs(h-m)<.01&&Math.abs(p-y)<.01&&Math.abs(x-g)<.01){if(Math.abs(h+m)<.1&&Math.abs(p+y)<.1&&Math.abs(x+g)<.1&&Math.abs(u+_+d-3)<.1)return this.set(1,0,0,0),this;e=Math.PI;const b=(u+1)/2,w=(_+1)/2,D=(d+1)/2,L=(h+m)/4,I=(p+y)/4,N=(x+g)/4;return b>w&&b>D?b<.01?(i=0,r=.707106781,s=.707106781):(i=Math.sqrt(b),r=L/i,s=I/i):w>D?w<.01?(i=.707106781,r=0,s=.707106781):(r=Math.sqrt(w),i=L/r,s=N/r):D<.01?(i=.707106781,r=.707106781,s=0):(s=Math.sqrt(D),i=I/s,r=N/s),this.set(i,r,s,e),this}let P=Math.sqrt((g-x)*(g-x)+(p-y)*(p-y)+(m-h)*(m-h));return Math.abs(P)<.001&&(P=1),this.x=(g-x)/P,this.y=(p-y)/P,this.z=(m-h)/P,this.w=Math.acos((u+_+d-1)/2),this}setFromMatrixPosition(t){const e=t.elements;return this.x=e[12],this.y=e[13],this.z=e[14],this.w=e[15],this}min(t){return this.x=Math.min(this.x,t.x),this.y=Math.min(this.y,t.y),this.z=Math.min(this.z,t.z),this.w=Math.min(this.w,t.w),this}max(t){return this.x=Math.max(this.x,t.x),this.y=Math.max(this.y,t.y),this.z=Math.max(this.z,t.z),this.w=Math.max(this.w,t.w),this}clamp(t,e){return this.x=kt(this.x,t.x,e.x),this.y=kt(this.y,t.y,e.y),this.z=kt(this.z,t.z,e.z),this.w=kt(this.w,t.w,e.w),this}clampScalar(t,e){return this.x=kt(this.x,t,e),this.y=kt(this.y,t,e),this.z=kt(this.z,t,e),this.w=kt(this.w,t,e),this}clampLength(t,e){const i=this.length();return this.divideScalar(i||1).multiplyScalar(kt(i,t,e))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this.z=Math.floor(this.z),this.w=Math.floor(this.w),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this.z=Math.ceil(this.z),this.w=Math.ceil(this.w),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this.z=Math.round(this.z),this.w=Math.round(this.w),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this.z=Math.trunc(this.z),this.w=Math.trunc(this.w),this}negate(){return this.x=-this.x,this.y=-this.y,this.z=-this.z,this.w=-this.w,this}dot(t){return this.x*t.x+this.y*t.y+this.z*t.z+this.w*t.w}lengthSq(){return this.x*this.x+this.y*this.y+this.z*this.z+this.w*this.w}length(){return Math.sqrt(this.x*this.x+this.y*this.y+this.z*this.z+this.w*this.w)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)+Math.abs(this.z)+Math.abs(this.w)}normalize(){return this.divideScalar(this.length()||1)}setLength(t){return this.normalize().multiplyScalar(t)}lerp(t,e){return this.x+=(t.x-this.x)*e,this.y+=(t.y-this.y)*e,this.z+=(t.z-this.z)*e,this.w+=(t.w-this.w)*e,this}lerpVectors(t,e,i){return this.x=t.x+(e.x-t.x)*i,this.y=t.y+(e.y-t.y)*i,this.z=t.z+(e.z-t.z)*i,this.w=t.w+(e.w-t.w)*i,this}equals(t){return t.x===this.x&&t.y===this.y&&t.z===this.z&&t.w===this.w}fromArray(t,e=0){return this.x=t[e],this.y=t[e+1],this.z=t[e+2],this.w=t[e+3],this}toArray(t=[],e=0){return t[e]=this.x,t[e+1]=this.y,t[e+2]=this.z,t[e+3]=this.w,t}fromBufferAttribute(t,e){return this.x=t.getX(e),this.y=t.getY(e),this.z=t.getZ(e),this.w=t.getW(e),this}random(){return this.x=Math.random(),this.y=Math.random(),this.z=Math.random(),this.w=Math.random(),this}*[Symbol.iterator](){yield this.x,yield this.y,yield this.z,yield this.w}}class Ju extends Ti{constructor(t=1,e=1,i={}){super(),this.isRenderTarget=!0,this.width=t,this.height=e,this.depth=i.depth?i.depth:1,this.scissor=new se(0,0,t,e),this.scissorTest=!1,this.viewport=new se(0,0,t,e);const r={width:t,height:e,depth:this.depth};i=Object.assign({generateMipmaps:!1,internalFormat:null,minFilter:Ke,depthBuffer:!0,stencilBuffer:!1,resolveDepthBuffer:!0,resolveStencilBuffer:!0,depthTexture:null,samples:0,count:1,multiview:!1},i);const s=new be(r,i.mapping,i.wrapS,i.wrapT,i.magFilter,i.minFilter,i.format,i.type,i.anisotropy,i.colorSpace);s.flipY=!1,s.generateMipmaps=i.generateMipmaps,s.internalFormat=i.internalFormat,this.textures=[];const o=i.count;for(let a=0;a<o;a++)this.textures[a]=s.clone(),this.textures[a].isRenderTargetTexture=!0,this.textures[a].renderTarget=this;this.depthBuffer=i.depthBuffer,this.stencilBuffer=i.stencilBuffer,this.resolveDepthBuffer=i.resolveDepthBuffer,this.resolveStencilBuffer=i.resolveStencilBuffer,this._depthTexture=null,this.depthTexture=i.depthTexture,this.samples=i.samples,this.multiview=i.multiview}get texture(){return this.textures[0]}set texture(t){this.textures[0]=t}set depthTexture(t){this._depthTexture!==null&&(this._depthTexture.renderTarget=null),t!==null&&(t.renderTarget=this),this._depthTexture=t}get depthTexture(){return this._depthTexture}setSize(t,e,i=1){if(this.width!==t||this.height!==e||this.depth!==i){this.width=t,this.height=e,this.depth=i;for(let r=0,s=this.textures.length;r<s;r++)this.textures[r].image.width=t,this.textures[r].image.height=e,this.textures[r].image.depth=i;this.dispose()}this.viewport.set(0,0,t,e),this.scissor.set(0,0,t,e)}clone(){return new this.constructor().copy(this)}copy(t){this.width=t.width,this.height=t.height,this.depth=t.depth,this.scissor.copy(t.scissor),this.scissorTest=t.scissorTest,this.viewport.copy(t.viewport),this.textures.length=0;for(let e=0,i=t.textures.length;e<i;e++){this.textures[e]=t.textures[e].clone(),this.textures[e].isRenderTargetTexture=!0,this.textures[e].renderTarget=this;const r=Object.assign({},t.textures[e].image);this.textures[e].source=new za(r)}return this.depthBuffer=t.depthBuffer,this.stencilBuffer=t.stencilBuffer,this.resolveDepthBuffer=t.resolveDepthBuffer,this.resolveStencilBuffer=t.resolveStencilBuffer,t.depthTexture!==null&&(this.depthTexture=t.depthTexture.clone()),this.samples=t.samples,this}dispose(){this.dispatchEvent({type:"dispose"})}}class jn extends Ju{constructor(t=1,e=1,i={}){super(t,e,i),this.isWebGLRenderTarget=!0}}class $c extends be{constructor(t=null,e=1,i=1,r=1){super(null),this.isDataArrayTexture=!0,this.image={data:t,width:e,height:i,depth:r},this.magFilter=qe,this.minFilter=qe,this.wrapR=qn,this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1,this.layerUpdates=new Set}addLayerUpdate(t){this.layerUpdates.add(t)}clearLayerUpdates(){this.layerUpdates.clear()}}class Qu extends be{constructor(t=null,e=1,i=1,r=1){super(null),this.isData3DTexture=!0,this.image={data:t,width:e,height:i,depth:r},this.magFilter=qe,this.minFilter=qe,this.wrapR=qn,this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1}}class Yi{constructor(t=0,e=0,i=0,r=1){this.isQuaternion=!0,this._x=t,this._y=e,this._z=i,this._w=r}static slerpFlat(t,e,i,r,s,o,a){let l=i[r+0],u=i[r+1],h=i[r+2],p=i[r+3];const m=s[o+0],_=s[o+1],x=s[o+2],y=s[o+3];if(a===0){t[e+0]=l,t[e+1]=u,t[e+2]=h,t[e+3]=p;return}if(a===1){t[e+0]=m,t[e+1]=_,t[e+2]=x,t[e+3]=y;return}if(p!==y||l!==m||u!==_||h!==x){let g=1-a;const d=l*m+u*_+h*x+p*y,P=d>=0?1:-1,b=1-d*d;if(b>Number.EPSILON){const D=Math.sqrt(b),L=Math.atan2(D,d*P);g=Math.sin(g*L)/D,a=Math.sin(a*L)/D}const w=a*P;if(l=l*g+m*w,u=u*g+_*w,h=h*g+x*w,p=p*g+y*w,g===1-a){const D=1/Math.sqrt(l*l+u*u+h*h+p*p);l*=D,u*=D,h*=D,p*=D}}t[e]=l,t[e+1]=u,t[e+2]=h,t[e+3]=p}static multiplyQuaternionsFlat(t,e,i,r,s,o){const a=i[r],l=i[r+1],u=i[r+2],h=i[r+3],p=s[o],m=s[o+1],_=s[o+2],x=s[o+3];return t[e]=a*x+h*p+l*_-u*m,t[e+1]=l*x+h*m+u*p-a*_,t[e+2]=u*x+h*_+a*m-l*p,t[e+3]=h*x-a*p-l*m-u*_,t}get x(){return this._x}set x(t){this._x=t,this._onChangeCallback()}get y(){return this._y}set y(t){this._y=t,this._onChangeCallback()}get z(){return this._z}set z(t){this._z=t,this._onChangeCallback()}get w(){return this._w}set w(t){this._w=t,this._onChangeCallback()}set(t,e,i,r){return this._x=t,this._y=e,this._z=i,this._w=r,this._onChangeCallback(),this}clone(){return new this.constructor(this._x,this._y,this._z,this._w)}copy(t){return this._x=t.x,this._y=t.y,this._z=t.z,this._w=t.w,this._onChangeCallback(),this}setFromEuler(t,e=!0){const i=t._x,r=t._y,s=t._z,o=t._order,a=Math.cos,l=Math.sin,u=a(i/2),h=a(r/2),p=a(s/2),m=l(i/2),_=l(r/2),x=l(s/2);switch(o){case"XYZ":this._x=m*h*p+u*_*x,this._y=u*_*p-m*h*x,this._z=u*h*x+m*_*p,this._w=u*h*p-m*_*x;break;case"YXZ":this._x=m*h*p+u*_*x,this._y=u*_*p-m*h*x,this._z=u*h*x-m*_*p,this._w=u*h*p+m*_*x;break;case"ZXY":this._x=m*h*p-u*_*x,this._y=u*_*p+m*h*x,this._z=u*h*x+m*_*p,this._w=u*h*p-m*_*x;break;case"ZYX":this._x=m*h*p-u*_*x,this._y=u*_*p+m*h*x,this._z=u*h*x-m*_*p,this._w=u*h*p+m*_*x;break;case"YZX":this._x=m*h*p+u*_*x,this._y=u*_*p+m*h*x,this._z=u*h*x-m*_*p,this._w=u*h*p-m*_*x;break;case"XZY":this._x=m*h*p-u*_*x,this._y=u*_*p-m*h*x,this._z=u*h*x+m*_*p,this._w=u*h*p+m*_*x;break;default:console.warn("THREE.Quaternion: .setFromEuler() encountered an unknown order: "+o)}return e===!0&&this._onChangeCallback(),this}setFromAxisAngle(t,e){const i=e/2,r=Math.sin(i);return this._x=t.x*r,this._y=t.y*r,this._z=t.z*r,this._w=Math.cos(i),this._onChangeCallback(),this}setFromRotationMatrix(t){const e=t.elements,i=e[0],r=e[4],s=e[8],o=e[1],a=e[5],l=e[9],u=e[2],h=e[6],p=e[10],m=i+a+p;if(m>0){const _=.5/Math.sqrt(m+1);this._w=.25/_,this._x=(h-l)*_,this._y=(s-u)*_,this._z=(o-r)*_}else if(i>a&&i>p){const _=2*Math.sqrt(1+i-a-p);this._w=(h-l)/_,this._x=.25*_,this._y=(r+o)/_,this._z=(s+u)/_}else if(a>p){const _=2*Math.sqrt(1+a-i-p);this._w=(s-u)/_,this._x=(r+o)/_,this._y=.25*_,this._z=(l+h)/_}else{const _=2*Math.sqrt(1+p-i-a);this._w=(o-r)/_,this._x=(s+u)/_,this._y=(l+h)/_,this._z=.25*_}return this._onChangeCallback(),this}setFromUnitVectors(t,e){let i=t.dot(e)+1;return i<Number.EPSILON?(i=0,Math.abs(t.x)>Math.abs(t.z)?(this._x=-t.y,this._y=t.x,this._z=0,this._w=i):(this._x=0,this._y=-t.z,this._z=t.y,this._w=i)):(this._x=t.y*e.z-t.z*e.y,this._y=t.z*e.x-t.x*e.z,this._z=t.x*e.y-t.y*e.x,this._w=i),this.normalize()}angleTo(t){return 2*Math.acos(Math.abs(kt(this.dot(t),-1,1)))}rotateTowards(t,e){const i=this.angleTo(t);if(i===0)return this;const r=Math.min(1,e/i);return this.slerp(t,r),this}identity(){return this.set(0,0,0,1)}invert(){return this.conjugate()}conjugate(){return this._x*=-1,this._y*=-1,this._z*=-1,this._onChangeCallback(),this}dot(t){return this._x*t._x+this._y*t._y+this._z*t._z+this._w*t._w}lengthSq(){return this._x*this._x+this._y*this._y+this._z*this._z+this._w*this._w}length(){return Math.sqrt(this._x*this._x+this._y*this._y+this._z*this._z+this._w*this._w)}normalize(){let t=this.length();return t===0?(this._x=0,this._y=0,this._z=0,this._w=1):(t=1/t,this._x=this._x*t,this._y=this._y*t,this._z=this._z*t,this._w=this._w*t),this._onChangeCallback(),this}multiply(t){return this.multiplyQuaternions(this,t)}premultiply(t){return this.multiplyQuaternions(t,this)}multiplyQuaternions(t,e){const i=t._x,r=t._y,s=t._z,o=t._w,a=e._x,l=e._y,u=e._z,h=e._w;return this._x=i*h+o*a+r*u-s*l,this._y=r*h+o*l+s*a-i*u,this._z=s*h+o*u+i*l-r*a,this._w=o*h-i*a-r*l-s*u,this._onChangeCallback(),this}slerp(t,e){if(e===0)return this;if(e===1)return this.copy(t);const i=this._x,r=this._y,s=this._z,o=this._w;let a=o*t._w+i*t._x+r*t._y+s*t._z;if(a<0?(this._w=-t._w,this._x=-t._x,this._y=-t._y,this._z=-t._z,a=-a):this.copy(t),a>=1)return this._w=o,this._x=i,this._y=r,this._z=s,this;const l=1-a*a;if(l<=Number.EPSILON){const _=1-e;return this._w=_*o+e*this._w,this._x=_*i+e*this._x,this._y=_*r+e*this._y,this._z=_*s+e*this._z,this.normalize(),this}const u=Math.sqrt(l),h=Math.atan2(u,a),p=Math.sin((1-e)*h)/u,m=Math.sin(e*h)/u;return this._w=o*p+this._w*m,this._x=i*p+this._x*m,this._y=r*p+this._y*m,this._z=s*p+this._z*m,this._onChangeCallback(),this}slerpQuaternions(t,e,i){return this.copy(t).slerp(e,i)}random(){const t=2*Math.PI*Math.random(),e=2*Math.PI*Math.random(),i=Math.random(),r=Math.sqrt(1-i),s=Math.sqrt(i);return this.set(r*Math.sin(t),r*Math.cos(t),s*Math.sin(e),s*Math.cos(e))}equals(t){return t._x===this._x&&t._y===this._y&&t._z===this._z&&t._w===this._w}fromArray(t,e=0){return this._x=t[e],this._y=t[e+1],this._z=t[e+2],this._w=t[e+3],this._onChangeCallback(),this}toArray(t=[],e=0){return t[e]=this._x,t[e+1]=this._y,t[e+2]=this._z,t[e+3]=this._w,t}fromBufferAttribute(t,e){return this._x=t.getX(e),this._y=t.getY(e),this._z=t.getZ(e),this._w=t.getW(e),this._onChangeCallback(),this}toJSON(){return this.toArray()}_onChange(t){return this._onChangeCallback=t,this}_onChangeCallback(){}*[Symbol.iterator](){yield this._x,yield this._y,yield this._z,yield this._w}}class H{constructor(t=0,e=0,i=0){H.prototype.isVector3=!0,this.x=t,this.y=e,this.z=i}set(t,e,i){return i===void 0&&(i=this.z),this.x=t,this.y=e,this.z=i,this}setScalar(t){return this.x=t,this.y=t,this.z=t,this}setX(t){return this.x=t,this}setY(t){return this.y=t,this}setZ(t){return this.z=t,this}setComponent(t,e){switch(t){case 0:this.x=e;break;case 1:this.y=e;break;case 2:this.z=e;break;default:throw new Error("index is out of range: "+t)}return this}getComponent(t){switch(t){case 0:return this.x;case 1:return this.y;case 2:return this.z;default:throw new Error("index is out of range: "+t)}}clone(){return new this.constructor(this.x,this.y,this.z)}copy(t){return this.x=t.x,this.y=t.y,this.z=t.z,this}add(t){return this.x+=t.x,this.y+=t.y,this.z+=t.z,this}addScalar(t){return this.x+=t,this.y+=t,this.z+=t,this}addVectors(t,e){return this.x=t.x+e.x,this.y=t.y+e.y,this.z=t.z+e.z,this}addScaledVector(t,e){return this.x+=t.x*e,this.y+=t.y*e,this.z+=t.z*e,this}sub(t){return this.x-=t.x,this.y-=t.y,this.z-=t.z,this}subScalar(t){return this.x-=t,this.y-=t,this.z-=t,this}subVectors(t,e){return this.x=t.x-e.x,this.y=t.y-e.y,this.z=t.z-e.z,this}multiply(t){return this.x*=t.x,this.y*=t.y,this.z*=t.z,this}multiplyScalar(t){return this.x*=t,this.y*=t,this.z*=t,this}multiplyVectors(t,e){return this.x=t.x*e.x,this.y=t.y*e.y,this.z=t.z*e.z,this}applyEuler(t){return this.applyQuaternion(mo.setFromEuler(t))}applyAxisAngle(t,e){return this.applyQuaternion(mo.setFromAxisAngle(t,e))}applyMatrix3(t){const e=this.x,i=this.y,r=this.z,s=t.elements;return this.x=s[0]*e+s[3]*i+s[6]*r,this.y=s[1]*e+s[4]*i+s[7]*r,this.z=s[2]*e+s[5]*i+s[8]*r,this}applyNormalMatrix(t){return this.applyMatrix3(t).normalize()}applyMatrix4(t){const e=this.x,i=this.y,r=this.z,s=t.elements,o=1/(s[3]*e+s[7]*i+s[11]*r+s[15]);return this.x=(s[0]*e+s[4]*i+s[8]*r+s[12])*o,this.y=(s[1]*e+s[5]*i+s[9]*r+s[13])*o,this.z=(s[2]*e+s[6]*i+s[10]*r+s[14])*o,this}applyQuaternion(t){const e=this.x,i=this.y,r=this.z,s=t.x,o=t.y,a=t.z,l=t.w,u=2*(o*r-a*i),h=2*(a*e-s*r),p=2*(s*i-o*e);return this.x=e+l*u+o*p-a*h,this.y=i+l*h+a*u-s*p,this.z=r+l*p+s*h-o*u,this}project(t){return this.applyMatrix4(t.matrixWorldInverse).applyMatrix4(t.projectionMatrix)}unproject(t){return this.applyMatrix4(t.projectionMatrixInverse).applyMatrix4(t.matrixWorld)}transformDirection(t){const e=this.x,i=this.y,r=this.z,s=t.elements;return this.x=s[0]*e+s[4]*i+s[8]*r,this.y=s[1]*e+s[5]*i+s[9]*r,this.z=s[2]*e+s[6]*i+s[10]*r,this.normalize()}divide(t){return this.x/=t.x,this.y/=t.y,this.z/=t.z,this}divideScalar(t){return this.multiplyScalar(1/t)}min(t){return this.x=Math.min(this.x,t.x),this.y=Math.min(this.y,t.y),this.z=Math.min(this.z,t.z),this}max(t){return this.x=Math.max(this.x,t.x),this.y=Math.max(this.y,t.y),this.z=Math.max(this.z,t.z),this}clamp(t,e){return this.x=kt(this.x,t.x,e.x),this.y=kt(this.y,t.y,e.y),this.z=kt(this.z,t.z,e.z),this}clampScalar(t,e){return this.x=kt(this.x,t,e),this.y=kt(this.y,t,e),this.z=kt(this.z,t,e),this}clampLength(t,e){const i=this.length();return this.divideScalar(i||1).multiplyScalar(kt(i,t,e))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this.z=Math.floor(this.z),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this.z=Math.ceil(this.z),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this.z=Math.round(this.z),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this.z=Math.trunc(this.z),this}negate(){return this.x=-this.x,this.y=-this.y,this.z=-this.z,this}dot(t){return this.x*t.x+this.y*t.y+this.z*t.z}lengthSq(){return this.x*this.x+this.y*this.y+this.z*this.z}length(){return Math.sqrt(this.x*this.x+this.y*this.y+this.z*this.z)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)+Math.abs(this.z)}normalize(){return this.divideScalar(this.length()||1)}setLength(t){return this.normalize().multiplyScalar(t)}lerp(t,e){return this.x+=(t.x-this.x)*e,this.y+=(t.y-this.y)*e,this.z+=(t.z-this.z)*e,this}lerpVectors(t,e,i){return this.x=t.x+(e.x-t.x)*i,this.y=t.y+(e.y-t.y)*i,this.z=t.z+(e.z-t.z)*i,this}cross(t){return this.crossVectors(this,t)}crossVectors(t,e){const i=t.x,r=t.y,s=t.z,o=e.x,a=e.y,l=e.z;return this.x=r*l-s*a,this.y=s*o-i*l,this.z=i*a-r*o,this}projectOnVector(t){const e=t.lengthSq();if(e===0)return this.set(0,0,0);const i=t.dot(this)/e;return this.copy(t).multiplyScalar(i)}projectOnPlane(t){return as.copy(this).projectOnVector(t),this.sub(as)}reflect(t){return this.sub(as.copy(t).multiplyScalar(2*this.dot(t)))}angleTo(t){const e=Math.sqrt(this.lengthSq()*t.lengthSq());if(e===0)return Math.PI/2;const i=this.dot(t)/e;return Math.acos(kt(i,-1,1))}distanceTo(t){return Math.sqrt(this.distanceToSquared(t))}distanceToSquared(t){const e=this.x-t.x,i=this.y-t.y,r=this.z-t.z;return e*e+i*i+r*r}manhattanDistanceTo(t){return Math.abs(this.x-t.x)+Math.abs(this.y-t.y)+Math.abs(this.z-t.z)}setFromSpherical(t){return this.setFromSphericalCoords(t.radius,t.phi,t.theta)}setFromSphericalCoords(t,e,i){const r=Math.sin(e)*t;return this.x=r*Math.sin(i),this.y=Math.cos(e)*t,this.z=r*Math.cos(i),this}setFromCylindrical(t){return this.setFromCylindricalCoords(t.radius,t.theta,t.y)}setFromCylindricalCoords(t,e,i){return this.x=t*Math.sin(e),this.y=i,this.z=t*Math.cos(e),this}setFromMatrixPosition(t){const e=t.elements;return this.x=e[12],this.y=e[13],this.z=e[14],this}setFromMatrixScale(t){const e=this.setFromMatrixColumn(t,0).length(),i=this.setFromMatrixColumn(t,1).length(),r=this.setFromMatrixColumn(t,2).length();return this.x=e,this.y=i,this.z=r,this}setFromMatrixColumn(t,e){return this.fromArray(t.elements,e*4)}setFromMatrix3Column(t,e){return this.fromArray(t.elements,e*3)}setFromEuler(t){return this.x=t._x,this.y=t._y,this.z=t._z,this}setFromColor(t){return this.x=t.r,this.y=t.g,this.z=t.b,this}equals(t){return t.x===this.x&&t.y===this.y&&t.z===this.z}fromArray(t,e=0){return this.x=t[e],this.y=t[e+1],this.z=t[e+2],this}toArray(t=[],e=0){return t[e]=this.x,t[e+1]=this.y,t[e+2]=this.z,t}fromBufferAttribute(t,e){return this.x=t.getX(e),this.y=t.getY(e),this.z=t.getZ(e),this}random(){return this.x=Math.random(),this.y=Math.random(),this.z=Math.random(),this}randomDirection(){const t=Math.random()*Math.PI*2,e=Math.random()*2-1,i=Math.sqrt(1-e*e);return this.x=i*Math.cos(t),this.y=e,this.z=i*Math.sin(t),this}*[Symbol.iterator](){yield this.x,yield this.y,yield this.z}}const as=new H,mo=new Yi;class ji{constructor(t=new H(1/0,1/0,1/0),e=new H(-1/0,-1/0,-1/0)){this.isBox3=!0,this.min=t,this.max=e}set(t,e){return this.min.copy(t),this.max.copy(e),this}setFromArray(t){this.makeEmpty();for(let e=0,i=t.length;e<i;e+=3)this.expandByPoint(ze.fromArray(t,e));return this}setFromBufferAttribute(t){this.makeEmpty();for(let e=0,i=t.count;e<i;e++)this.expandByPoint(ze.fromBufferAttribute(t,e));return this}setFromPoints(t){this.makeEmpty();for(let e=0,i=t.length;e<i;e++)this.expandByPoint(t[e]);return this}setFromCenterAndSize(t,e){const i=ze.copy(e).multiplyScalar(.5);return this.min.copy(t).sub(i),this.max.copy(t).add(i),this}setFromObject(t,e=!1){return this.makeEmpty(),this.expandByObject(t,e)}clone(){return new this.constructor().copy(this)}copy(t){return this.min.copy(t.min),this.max.copy(t.max),this}makeEmpty(){return this.min.x=this.min.y=this.min.z=1/0,this.max.x=this.max.y=this.max.z=-1/0,this}isEmpty(){return this.max.x<this.min.x||this.max.y<this.min.y||this.max.z<this.min.z}getCenter(t){return this.isEmpty()?t.set(0,0,0):t.addVectors(this.min,this.max).multiplyScalar(.5)}getSize(t){return this.isEmpty()?t.set(0,0,0):t.subVectors(this.max,this.min)}expandByPoint(t){return this.min.min(t),this.max.max(t),this}expandByVector(t){return this.min.sub(t),this.max.add(t),this}expandByScalar(t){return this.min.addScalar(-t),this.max.addScalar(t),this}expandByObject(t,e=!1){t.updateWorldMatrix(!1,!1);const i=t.geometry;if(i!==void 0){const s=i.getAttribute("position");if(e===!0&&s!==void 0&&t.isInstancedMesh!==!0)for(let o=0,a=s.count;o<a;o++)t.isMesh===!0?t.getVertexPosition(o,ze):ze.fromBufferAttribute(s,o),ze.applyMatrix4(t.matrixWorld),this.expandByPoint(ze);else t.boundingBox!==void 0?(t.boundingBox===null&&t.computeBoundingBox(),er.copy(t.boundingBox)):(i.boundingBox===null&&i.computeBoundingBox(),er.copy(i.boundingBox)),er.applyMatrix4(t.matrixWorld),this.union(er)}const r=t.children;for(let s=0,o=r.length;s<o;s++)this.expandByObject(r[s],e);return this}containsPoint(t){return t.x>=this.min.x&&t.x<=this.max.x&&t.y>=this.min.y&&t.y<=this.max.y&&t.z>=this.min.z&&t.z<=this.max.z}containsBox(t){return this.min.x<=t.min.x&&t.max.x<=this.max.x&&this.min.y<=t.min.y&&t.max.y<=this.max.y&&this.min.z<=t.min.z&&t.max.z<=this.max.z}getParameter(t,e){return e.set((t.x-this.min.x)/(this.max.x-this.min.x),(t.y-this.min.y)/(this.max.y-this.min.y),(t.z-this.min.z)/(this.max.z-this.min.z))}intersectsBox(t){return t.max.x>=this.min.x&&t.min.x<=this.max.x&&t.max.y>=this.min.y&&t.min.y<=this.max.y&&t.max.z>=this.min.z&&t.min.z<=this.max.z}intersectsSphere(t){return this.clampPoint(t.center,ze),ze.distanceToSquared(t.center)<=t.radius*t.radius}intersectsPlane(t){let e,i;return t.normal.x>0?(e=t.normal.x*this.min.x,i=t.normal.x*this.max.x):(e=t.normal.x*this.max.x,i=t.normal.x*this.min.x),t.normal.y>0?(e+=t.normal.y*this.min.y,i+=t.normal.y*this.max.y):(e+=t.normal.y*this.max.y,i+=t.normal.y*this.min.y),t.normal.z>0?(e+=t.normal.z*this.min.z,i+=t.normal.z*this.max.z):(e+=t.normal.z*this.max.z,i+=t.normal.z*this.min.z),e<=-t.constant&&i>=-t.constant}intersectsTriangle(t){if(this.isEmpty())return!1;this.getCenter(Li),nr.subVectors(this.max,Li),ii.subVectors(t.a,Li),ri.subVectors(t.b,Li),si.subVectors(t.c,Li),vn.subVectors(ri,ii),yn.subVectors(si,ri),Nn.subVectors(ii,si);let e=[0,-vn.z,vn.y,0,-yn.z,yn.y,0,-Nn.z,Nn.y,vn.z,0,-vn.x,yn.z,0,-yn.x,Nn.z,0,-Nn.x,-vn.y,vn.x,0,-yn.y,yn.x,0,-Nn.y,Nn.x,0];return!os(e,ii,ri,si,nr)||(e=[1,0,0,0,1,0,0,0,1],!os(e,ii,ri,si,nr))?!1:(ir.crossVectors(vn,yn),e=[ir.x,ir.y,ir.z],os(e,ii,ri,si,nr))}clampPoint(t,e){return e.copy(t).clamp(this.min,this.max)}distanceToPoint(t){return this.clampPoint(t,ze).distanceTo(t)}getBoundingSphere(t){return this.isEmpty()?t.makeEmpty():(this.getCenter(t.center),t.radius=this.getSize(ze).length()*.5),t}intersect(t){return this.min.max(t.min),this.max.min(t.max),this.isEmpty()&&this.makeEmpty(),this}union(t){return this.min.min(t.min),this.max.max(t.max),this}applyMatrix4(t){return this.isEmpty()?this:(tn[0].set(this.min.x,this.min.y,this.min.z).applyMatrix4(t),tn[1].set(this.min.x,this.min.y,this.max.z).applyMatrix4(t),tn[2].set(this.min.x,this.max.y,this.min.z).applyMatrix4(t),tn[3].set(this.min.x,this.max.y,this.max.z).applyMatrix4(t),tn[4].set(this.max.x,this.min.y,this.min.z).applyMatrix4(t),tn[5].set(this.max.x,this.min.y,this.max.z).applyMatrix4(t),tn[6].set(this.max.x,this.max.y,this.min.z).applyMatrix4(t),tn[7].set(this.max.x,this.max.y,this.max.z).applyMatrix4(t),this.setFromPoints(tn),this)}translate(t){return this.min.add(t),this.max.add(t),this}equals(t){return t.min.equals(this.min)&&t.max.equals(this.max)}}const tn=[new H,new H,new H,new H,new H,new H,new H,new H],ze=new H,er=new ji,ii=new H,ri=new H,si=new H,vn=new H,yn=new H,Nn=new H,Li=new H,nr=new H,ir=new H,On=new H;function os(n,t,e,i,r){for(let s=0,o=n.length-3;s<=o;s+=3){On.fromArray(n,s);const a=r.x*Math.abs(On.x)+r.y*Math.abs(On.y)+r.z*Math.abs(On.z),l=t.dot(On),u=e.dot(On),h=i.dot(On);if(Math.max(-Math.max(l,u,h),Math.min(l,u,h))>a)return!1}return!0}const tf=new ji,Di=new H,cs=new H;class Wr{constructor(t=new H,e=-1){this.isSphere=!0,this.center=t,this.radius=e}set(t,e){return this.center.copy(t),this.radius=e,this}setFromPoints(t,e){const i=this.center;e!==void 0?i.copy(e):tf.setFromPoints(t).getCenter(i);let r=0;for(let s=0,o=t.length;s<o;s++)r=Math.max(r,i.distanceToSquared(t[s]));return this.radius=Math.sqrt(r),this}copy(t){return this.center.copy(t.center),this.radius=t.radius,this}isEmpty(){return this.radius<0}makeEmpty(){return this.center.set(0,0,0),this.radius=-1,this}containsPoint(t){return t.distanceToSquared(this.center)<=this.radius*this.radius}distanceToPoint(t){return t.distanceTo(this.center)-this.radius}intersectsSphere(t){const e=this.radius+t.radius;return t.center.distanceToSquared(this.center)<=e*e}intersectsBox(t){return t.intersectsSphere(this)}intersectsPlane(t){return Math.abs(t.distanceToPoint(this.center))<=this.radius}clampPoint(t,e){const i=this.center.distanceToSquared(t);return e.copy(t),i>this.radius*this.radius&&(e.sub(this.center).normalize(),e.multiplyScalar(this.radius).add(this.center)),e}getBoundingBox(t){return this.isEmpty()?(t.makeEmpty(),t):(t.set(this.center,this.center),t.expandByScalar(this.radius),t)}applyMatrix4(t){return this.center.applyMatrix4(t),this.radius=this.radius*t.getMaxScaleOnAxis(),this}translate(t){return this.center.add(t),this}expandByPoint(t){if(this.isEmpty())return this.center.copy(t),this.radius=0,this;Di.subVectors(t,this.center);const e=Di.lengthSq();if(e>this.radius*this.radius){const i=Math.sqrt(e),r=(i-this.radius)*.5;this.center.addScaledVector(Di,r/i),this.radius+=r}return this}union(t){return t.isEmpty()?this:this.isEmpty()?(this.copy(t),this):(this.center.equals(t.center)===!0?this.radius=Math.max(this.radius,t.radius):(cs.subVectors(t.center,this.center).setLength(t.radius),this.expandByPoint(Di.copy(t.center).add(cs)),this.expandByPoint(Di.copy(t.center).sub(cs))),this)}equals(t){return t.center.equals(this.center)&&t.radius===this.radius}clone(){return new this.constructor().copy(this)}}const en=new H,ls=new H,rr=new H,Sn=new H,us=new H,sr=new H,fs=new H;class Yc{constructor(t=new H,e=new H(0,0,-1)){this.origin=t,this.direction=e}set(t,e){return this.origin.copy(t),this.direction.copy(e),this}copy(t){return this.origin.copy(t.origin),this.direction.copy(t.direction),this}at(t,e){return e.copy(this.origin).addScaledVector(this.direction,t)}lookAt(t){return this.direction.copy(t).sub(this.origin).normalize(),this}recast(t){return this.origin.copy(this.at(t,en)),this}closestPointToPoint(t,e){e.subVectors(t,this.origin);const i=e.dot(this.direction);return i<0?e.copy(this.origin):e.copy(this.origin).addScaledVector(this.direction,i)}distanceToPoint(t){return Math.sqrt(this.distanceSqToPoint(t))}distanceSqToPoint(t){const e=en.subVectors(t,this.origin).dot(this.direction);return e<0?this.origin.distanceToSquared(t):(en.copy(this.origin).addScaledVector(this.direction,e),en.distanceToSquared(t))}distanceSqToSegment(t,e,i,r){ls.copy(t).add(e).multiplyScalar(.5),rr.copy(e).sub(t).normalize(),Sn.copy(this.origin).sub(ls);const s=t.distanceTo(e)*.5,o=-this.direction.dot(rr),a=Sn.dot(this.direction),l=-Sn.dot(rr),u=Sn.lengthSq(),h=Math.abs(1-o*o);let p,m,_,x;if(h>0)if(p=o*l-a,m=o*a-l,x=s*h,p>=0)if(m>=-x)if(m<=x){const y=1/h;p*=y,m*=y,_=p*(p+o*m+2*a)+m*(o*p+m+2*l)+u}else m=s,p=Math.max(0,-(o*m+a)),_=-p*p+m*(m+2*l)+u;else m=-s,p=Math.max(0,-(o*m+a)),_=-p*p+m*(m+2*l)+u;else m<=-x?(p=Math.max(0,-(-o*s+a)),m=p>0?-s:Math.min(Math.max(-s,-l),s),_=-p*p+m*(m+2*l)+u):m<=x?(p=0,m=Math.min(Math.max(-s,-l),s),_=m*(m+2*l)+u):(p=Math.max(0,-(o*s+a)),m=p>0?s:Math.min(Math.max(-s,-l),s),_=-p*p+m*(m+2*l)+u);else m=o>0?-s:s,p=Math.max(0,-(o*m+a)),_=-p*p+m*(m+2*l)+u;return i&&i.copy(this.origin).addScaledVector(this.direction,p),r&&r.copy(ls).addScaledVector(rr,m),_}intersectSphere(t,e){en.subVectors(t.center,this.origin);const i=en.dot(this.direction),r=en.dot(en)-i*i,s=t.radius*t.radius;if(r>s)return null;const o=Math.sqrt(s-r),a=i-o,l=i+o;return l<0?null:a<0?this.at(l,e):this.at(a,e)}intersectsSphere(t){return this.distanceSqToPoint(t.center)<=t.radius*t.radius}distanceToPlane(t){const e=t.normal.dot(this.direction);if(e===0)return t.distanceToPoint(this.origin)===0?0:null;const i=-(this.origin.dot(t.normal)+t.constant)/e;return i>=0?i:null}intersectPlane(t,e){const i=this.distanceToPlane(t);return i===null?null:this.at(i,e)}intersectsPlane(t){const e=t.distanceToPoint(this.origin);return e===0||t.normal.dot(this.direction)*e<0}intersectBox(t,e){let i,r,s,o,a,l;const u=1/this.direction.x,h=1/this.direction.y,p=1/this.direction.z,m=this.origin;return u>=0?(i=(t.min.x-m.x)*u,r=(t.max.x-m.x)*u):(i=(t.max.x-m.x)*u,r=(t.min.x-m.x)*u),h>=0?(s=(t.min.y-m.y)*h,o=(t.max.y-m.y)*h):(s=(t.max.y-m.y)*h,o=(t.min.y-m.y)*h),i>o||s>r||((s>i||isNaN(i))&&(i=s),(o<r||isNaN(r))&&(r=o),p>=0?(a=(t.min.z-m.z)*p,l=(t.max.z-m.z)*p):(a=(t.max.z-m.z)*p,l=(t.min.z-m.z)*p),i>l||a>r)||((a>i||i!==i)&&(i=a),(l<r||r!==r)&&(r=l),r<0)?null:this.at(i>=0?i:r,e)}intersectsBox(t){return this.intersectBox(t,en)!==null}intersectTriangle(t,e,i,r,s){us.subVectors(e,t),sr.subVectors(i,t),fs.crossVectors(us,sr);let o=this.direction.dot(fs),a;if(o>0){if(r)return null;a=1}else if(o<0)a=-1,o=-o;else return null;Sn.subVectors(this.origin,t);const l=a*this.direction.dot(sr.crossVectors(Sn,sr));if(l<0)return null;const u=a*this.direction.dot(us.cross(Sn));if(u<0||l+u>o)return null;const h=-a*Sn.dot(fs);return h<0?null:this.at(h/o,s)}applyMatrix4(t){return this.origin.applyMatrix4(t),this.direction.transformDirection(t),this}equals(t){return t.origin.equals(this.origin)&&t.direction.equals(this.direction)}clone(){return new this.constructor().copy(this)}}class te{constructor(t,e,i,r,s,o,a,l,u,h,p,m,_,x,y,g){te.prototype.isMatrix4=!0,this.elements=[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1],t!==void 0&&this.set(t,e,i,r,s,o,a,l,u,h,p,m,_,x,y,g)}set(t,e,i,r,s,o,a,l,u,h,p,m,_,x,y,g){const d=this.elements;return d[0]=t,d[4]=e,d[8]=i,d[12]=r,d[1]=s,d[5]=o,d[9]=a,d[13]=l,d[2]=u,d[6]=h,d[10]=p,d[14]=m,d[3]=_,d[7]=x,d[11]=y,d[15]=g,this}identity(){return this.set(1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1),this}clone(){return new te().fromArray(this.elements)}copy(t){const e=this.elements,i=t.elements;return e[0]=i[0],e[1]=i[1],e[2]=i[2],e[3]=i[3],e[4]=i[4],e[5]=i[5],e[6]=i[6],e[7]=i[7],e[8]=i[8],e[9]=i[9],e[10]=i[10],e[11]=i[11],e[12]=i[12],e[13]=i[13],e[14]=i[14],e[15]=i[15],this}copyPosition(t){const e=this.elements,i=t.elements;return e[12]=i[12],e[13]=i[13],e[14]=i[14],this}setFromMatrix3(t){const e=t.elements;return this.set(e[0],e[3],e[6],0,e[1],e[4],e[7],0,e[2],e[5],e[8],0,0,0,0,1),this}extractBasis(t,e,i){return t.setFromMatrixColumn(this,0),e.setFromMatrixColumn(this,1),i.setFromMatrixColumn(this,2),this}makeBasis(t,e,i){return this.set(t.x,e.x,i.x,0,t.y,e.y,i.y,0,t.z,e.z,i.z,0,0,0,0,1),this}extractRotation(t){const e=this.elements,i=t.elements,r=1/ai.setFromMatrixColumn(t,0).length(),s=1/ai.setFromMatrixColumn(t,1).length(),o=1/ai.setFromMatrixColumn(t,2).length();return e[0]=i[0]*r,e[1]=i[1]*r,e[2]=i[2]*r,e[3]=0,e[4]=i[4]*s,e[5]=i[5]*s,e[6]=i[6]*s,e[7]=0,e[8]=i[8]*o,e[9]=i[9]*o,e[10]=i[10]*o,e[11]=0,e[12]=0,e[13]=0,e[14]=0,e[15]=1,this}makeRotationFromEuler(t){const e=this.elements,i=t.x,r=t.y,s=t.z,o=Math.cos(i),a=Math.sin(i),l=Math.cos(r),u=Math.sin(r),h=Math.cos(s),p=Math.sin(s);if(t.order==="XYZ"){const m=o*h,_=o*p,x=a*h,y=a*p;e[0]=l*h,e[4]=-l*p,e[8]=u,e[1]=_+x*u,e[5]=m-y*u,e[9]=-a*l,e[2]=y-m*u,e[6]=x+_*u,e[10]=o*l}else if(t.order==="YXZ"){const m=l*h,_=l*p,x=u*h,y=u*p;e[0]=m+y*a,e[4]=x*a-_,e[8]=o*u,e[1]=o*p,e[5]=o*h,e[9]=-a,e[2]=_*a-x,e[6]=y+m*a,e[10]=o*l}else if(t.order==="ZXY"){const m=l*h,_=l*p,x=u*h,y=u*p;e[0]=m-y*a,e[4]=-o*p,e[8]=x+_*a,e[1]=_+x*a,e[5]=o*h,e[9]=y-m*a,e[2]=-o*u,e[6]=a,e[10]=o*l}else if(t.order==="ZYX"){const m=o*h,_=o*p,x=a*h,y=a*p;e[0]=l*h,e[4]=x*u-_,e[8]=m*u+y,e[1]=l*p,e[5]=y*u+m,e[9]=_*u-x,e[2]=-u,e[6]=a*l,e[10]=o*l}else if(t.order==="YZX"){const m=o*l,_=o*u,x=a*l,y=a*u;e[0]=l*h,e[4]=y-m*p,e[8]=x*p+_,e[1]=p,e[5]=o*h,e[9]=-a*h,e[2]=-u*h,e[6]=_*p+x,e[10]=m-y*p}else if(t.order==="XZY"){const m=o*l,_=o*u,x=a*l,y=a*u;e[0]=l*h,e[4]=-p,e[8]=u*h,e[1]=m*p+y,e[5]=o*h,e[9]=_*p-x,e[2]=x*p-_,e[6]=a*h,e[10]=y*p+m}return e[3]=0,e[7]=0,e[11]=0,e[12]=0,e[13]=0,e[14]=0,e[15]=1,this}makeRotationFromQuaternion(t){return this.compose(ef,t,nf)}lookAt(t,e,i){const r=this.elements;return Re.subVectors(t,e),Re.lengthSq()===0&&(Re.z=1),Re.normalize(),Mn.crossVectors(i,Re),Mn.lengthSq()===0&&(Math.abs(i.z)===1?Re.x+=1e-4:Re.z+=1e-4,Re.normalize(),Mn.crossVectors(i,Re)),Mn.normalize(),ar.crossVectors(Re,Mn),r[0]=Mn.x,r[4]=ar.x,r[8]=Re.x,r[1]=Mn.y,r[5]=ar.y,r[9]=Re.y,r[2]=Mn.z,r[6]=ar.z,r[10]=Re.z,this}multiply(t){return this.multiplyMatrices(this,t)}premultiply(t){return this.multiplyMatrices(t,this)}multiplyMatrices(t,e){const i=t.elements,r=e.elements,s=this.elements,o=i[0],a=i[4],l=i[8],u=i[12],h=i[1],p=i[5],m=i[9],_=i[13],x=i[2],y=i[6],g=i[10],d=i[14],P=i[3],b=i[7],w=i[11],D=i[15],L=r[0],I=r[4],N=r[8],A=r[12],T=r[1],O=r[5],$=r[9],W=r[13],K=r[2],it=r[6],Z=r[10],at=r[14],Y=r[3],ft=r[7],gt=r[11],ht=r[15];return s[0]=o*L+a*T+l*K+u*Y,s[4]=o*I+a*O+l*it+u*ft,s[8]=o*N+a*$+l*Z+u*gt,s[12]=o*A+a*W+l*at+u*ht,s[1]=h*L+p*T+m*K+_*Y,s[5]=h*I+p*O+m*it+_*ft,s[9]=h*N+p*$+m*Z+_*gt,s[13]=h*A+p*W+m*at+_*ht,s[2]=x*L+y*T+g*K+d*Y,s[6]=x*I+y*O+g*it+d*ft,s[10]=x*N+y*$+g*Z+d*gt,s[14]=x*A+y*W+g*at+d*ht,s[3]=P*L+b*T+w*K+D*Y,s[7]=P*I+b*O+w*it+D*ft,s[11]=P*N+b*$+w*Z+D*gt,s[15]=P*A+b*W+w*at+D*ht,this}multiplyScalar(t){const e=this.elements;return e[0]*=t,e[4]*=t,e[8]*=t,e[12]*=t,e[1]*=t,e[5]*=t,e[9]*=t,e[13]*=t,e[2]*=t,e[6]*=t,e[10]*=t,e[14]*=t,e[3]*=t,e[7]*=t,e[11]*=t,e[15]*=t,this}determinant(){const t=this.elements,e=t[0],i=t[4],r=t[8],s=t[12],o=t[1],a=t[5],l=t[9],u=t[13],h=t[2],p=t[6],m=t[10],_=t[14],x=t[3],y=t[7],g=t[11],d=t[15];return x*(+s*l*p-r*u*p-s*a*m+i*u*m+r*a*_-i*l*_)+y*(+e*l*_-e*u*m+s*o*m-r*o*_+r*u*h-s*l*h)+g*(+e*u*p-e*a*_-s*o*p+i*o*_+s*a*h-i*u*h)+d*(-r*a*h-e*l*p+e*a*m+r*o*p-i*o*m+i*l*h)}transpose(){const t=this.elements;let e;return e=t[1],t[1]=t[4],t[4]=e,e=t[2],t[2]=t[8],t[8]=e,e=t[6],t[6]=t[9],t[9]=e,e=t[3],t[3]=t[12],t[12]=e,e=t[7],t[7]=t[13],t[13]=e,e=t[11],t[11]=t[14],t[14]=e,this}setPosition(t,e,i){const r=this.elements;return t.isVector3?(r[12]=t.x,r[13]=t.y,r[14]=t.z):(r[12]=t,r[13]=e,r[14]=i),this}invert(){const t=this.elements,e=t[0],i=t[1],r=t[2],s=t[3],o=t[4],a=t[5],l=t[6],u=t[7],h=t[8],p=t[9],m=t[10],_=t[11],x=t[12],y=t[13],g=t[14],d=t[15],P=p*g*u-y*m*u+y*l*_-a*g*_-p*l*d+a*m*d,b=x*m*u-h*g*u-x*l*_+o*g*_+h*l*d-o*m*d,w=h*y*u-x*p*u+x*a*_-o*y*_-h*a*d+o*p*d,D=x*p*l-h*y*l-x*a*m+o*y*m+h*a*g-o*p*g,L=e*P+i*b+r*w+s*D;if(L===0)return this.set(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);const I=1/L;return t[0]=P*I,t[1]=(y*m*s-p*g*s-y*r*_+i*g*_+p*r*d-i*m*d)*I,t[2]=(a*g*s-y*l*s+y*r*u-i*g*u-a*r*d+i*l*d)*I,t[3]=(p*l*s-a*m*s-p*r*u+i*m*u+a*r*_-i*l*_)*I,t[4]=b*I,t[5]=(h*g*s-x*m*s+x*r*_-e*g*_-h*r*d+e*m*d)*I,t[6]=(x*l*s-o*g*s-x*r*u+e*g*u+o*r*d-e*l*d)*I,t[7]=(o*m*s-h*l*s+h*r*u-e*m*u-o*r*_+e*l*_)*I,t[8]=w*I,t[9]=(x*p*s-h*y*s-x*i*_+e*y*_+h*i*d-e*p*d)*I,t[10]=(o*y*s-x*a*s+x*i*u-e*y*u-o*i*d+e*a*d)*I,t[11]=(h*a*s-o*p*s-h*i*u+e*p*u+o*i*_-e*a*_)*I,t[12]=D*I,t[13]=(h*y*r-x*p*r+x*i*m-e*y*m-h*i*g+e*p*g)*I,t[14]=(x*a*r-o*y*r-x*i*l+e*y*l+o*i*g-e*a*g)*I,t[15]=(o*p*r-h*a*r+h*i*l-e*p*l-o*i*m+e*a*m)*I,this}scale(t){const e=this.elements,i=t.x,r=t.y,s=t.z;return e[0]*=i,e[4]*=r,e[8]*=s,e[1]*=i,e[5]*=r,e[9]*=s,e[2]*=i,e[6]*=r,e[10]*=s,e[3]*=i,e[7]*=r,e[11]*=s,this}getMaxScaleOnAxis(){const t=this.elements,e=t[0]*t[0]+t[1]*t[1]+t[2]*t[2],i=t[4]*t[4]+t[5]*t[5]+t[6]*t[6],r=t[8]*t[8]+t[9]*t[9]+t[10]*t[10];return Math.sqrt(Math.max(e,i,r))}makeTranslation(t,e,i){return t.isVector3?this.set(1,0,0,t.x,0,1,0,t.y,0,0,1,t.z,0,0,0,1):this.set(1,0,0,t,0,1,0,e,0,0,1,i,0,0,0,1),this}makeRotationX(t){const e=Math.cos(t),i=Math.sin(t);return this.set(1,0,0,0,0,e,-i,0,0,i,e,0,0,0,0,1),this}makeRotationY(t){const e=Math.cos(t),i=Math.sin(t);return this.set(e,0,i,0,0,1,0,0,-i,0,e,0,0,0,0,1),this}makeRotationZ(t){const e=Math.cos(t),i=Math.sin(t);return this.set(e,-i,0,0,i,e,0,0,0,0,1,0,0,0,0,1),this}makeRotationAxis(t,e){const i=Math.cos(e),r=Math.sin(e),s=1-i,o=t.x,a=t.y,l=t.z,u=s*o,h=s*a;return this.set(u*o+i,u*a-r*l,u*l+r*a,0,u*a+r*l,h*a+i,h*l-r*o,0,u*l-r*a,h*l+r*o,s*l*l+i,0,0,0,0,1),this}makeScale(t,e,i){return this.set(t,0,0,0,0,e,0,0,0,0,i,0,0,0,0,1),this}makeShear(t,e,i,r,s,o){return this.set(1,i,s,0,t,1,o,0,e,r,1,0,0,0,0,1),this}compose(t,e,i){const r=this.elements,s=e._x,o=e._y,a=e._z,l=e._w,u=s+s,h=o+o,p=a+a,m=s*u,_=s*h,x=s*p,y=o*h,g=o*p,d=a*p,P=l*u,b=l*h,w=l*p,D=i.x,L=i.y,I=i.z;return r[0]=(1-(y+d))*D,r[1]=(_+w)*D,r[2]=(x-b)*D,r[3]=0,r[4]=(_-w)*L,r[5]=(1-(m+d))*L,r[6]=(g+P)*L,r[7]=0,r[8]=(x+b)*I,r[9]=(g-P)*I,r[10]=(1-(m+y))*I,r[11]=0,r[12]=t.x,r[13]=t.y,r[14]=t.z,r[15]=1,this}decompose(t,e,i){const r=this.elements;let s=ai.set(r[0],r[1],r[2]).length();const o=ai.set(r[4],r[5],r[6]).length(),a=ai.set(r[8],r[9],r[10]).length();this.determinant()<0&&(s=-s),t.x=r[12],t.y=r[13],t.z=r[14],ke.copy(this);const u=1/s,h=1/o,p=1/a;return ke.elements[0]*=u,ke.elements[1]*=u,ke.elements[2]*=u,ke.elements[4]*=h,ke.elements[5]*=h,ke.elements[6]*=h,ke.elements[8]*=p,ke.elements[9]*=p,ke.elements[10]*=p,e.setFromRotationMatrix(ke),i.x=s,i.y=o,i.z=a,this}makePerspective(t,e,i,r,s,o,a=ln){const l=this.elements,u=2*s/(e-t),h=2*s/(i-r),p=(e+t)/(e-t),m=(i+r)/(i-r);let _,x;if(a===ln)_=-(o+s)/(o-s),x=-2*o*s/(o-s);else if(a===Or)_=-o/(o-s),x=-o*s/(o-s);else throw new Error("THREE.Matrix4.makePerspective(): Invalid coordinate system: "+a);return l[0]=u,l[4]=0,l[8]=p,l[12]=0,l[1]=0,l[5]=h,l[9]=m,l[13]=0,l[2]=0,l[6]=0,l[10]=_,l[14]=x,l[3]=0,l[7]=0,l[11]=-1,l[15]=0,this}makeOrthographic(t,e,i,r,s,o,a=ln){const l=this.elements,u=1/(e-t),h=1/(i-r),p=1/(o-s),m=(e+t)*u,_=(i+r)*h;let x,y;if(a===ln)x=(o+s)*p,y=-2*p;else if(a===Or)x=s*p,y=-1*p;else throw new Error("THREE.Matrix4.makeOrthographic(): Invalid coordinate system: "+a);return l[0]=2*u,l[4]=0,l[8]=0,l[12]=-m,l[1]=0,l[5]=2*h,l[9]=0,l[13]=-_,l[2]=0,l[6]=0,l[10]=y,l[14]=-x,l[3]=0,l[7]=0,l[11]=0,l[15]=1,this}equals(t){const e=this.elements,i=t.elements;for(let r=0;r<16;r++)if(e[r]!==i[r])return!1;return!0}fromArray(t,e=0){for(let i=0;i<16;i++)this.elements[i]=t[i+e];return this}toArray(t=[],e=0){const i=this.elements;return t[e]=i[0],t[e+1]=i[1],t[e+2]=i[2],t[e+3]=i[3],t[e+4]=i[4],t[e+5]=i[5],t[e+6]=i[6],t[e+7]=i[7],t[e+8]=i[8],t[e+9]=i[9],t[e+10]=i[10],t[e+11]=i[11],t[e+12]=i[12],t[e+13]=i[13],t[e+14]=i[14],t[e+15]=i[15],t}}const ai=new H,ke=new te,ef=new H(0,0,0),nf=new H(1,1,1),Mn=new H,ar=new H,Re=new H,_o=new te,go=new Yi;class Ye{constructor(t=0,e=0,i=0,r=Ye.DEFAULT_ORDER){this.isEuler=!0,this._x=t,this._y=e,this._z=i,this._order=r}get x(){return this._x}set x(t){this._x=t,this._onChangeCallback()}get y(){return this._y}set y(t){this._y=t,this._onChangeCallback()}get z(){return this._z}set z(t){this._z=t,this._onChangeCallback()}get order(){return this._order}set order(t){this._order=t,this._onChangeCallback()}set(t,e,i,r=this._order){return this._x=t,this._y=e,this._z=i,this._order=r,this._onChangeCallback(),this}clone(){return new this.constructor(this._x,this._y,this._z,this._order)}copy(t){return this._x=t._x,this._y=t._y,this._z=t._z,this._order=t._order,this._onChangeCallback(),this}setFromRotationMatrix(t,e=this._order,i=!0){const r=t.elements,s=r[0],o=r[4],a=r[8],l=r[1],u=r[5],h=r[9],p=r[2],m=r[6],_=r[10];switch(e){case"XYZ":this._y=Math.asin(kt(a,-1,1)),Math.abs(a)<.9999999?(this._x=Math.atan2(-h,_),this._z=Math.atan2(-o,s)):(this._x=Math.atan2(m,u),this._z=0);break;case"YXZ":this._x=Math.asin(-kt(h,-1,1)),Math.abs(h)<.9999999?(this._y=Math.atan2(a,_),this._z=Math.atan2(l,u)):(this._y=Math.atan2(-p,s),this._z=0);break;case"ZXY":this._x=Math.asin(kt(m,-1,1)),Math.abs(m)<.9999999?(this._y=Math.atan2(-p,_),this._z=Math.atan2(-o,u)):(this._y=0,this._z=Math.atan2(l,s));break;case"ZYX":this._y=Math.asin(-kt(p,-1,1)),Math.abs(p)<.9999999?(this._x=Math.atan2(m,_),this._z=Math.atan2(l,s)):(this._x=0,this._z=Math.atan2(-o,u));break;case"YZX":this._z=Math.asin(kt(l,-1,1)),Math.abs(l)<.9999999?(this._x=Math.atan2(-h,u),this._y=Math.atan2(-p,s)):(this._x=0,this._y=Math.atan2(a,_));break;case"XZY":this._z=Math.asin(-kt(o,-1,1)),Math.abs(o)<.9999999?(this._x=Math.atan2(m,u),this._y=Math.atan2(a,s)):(this._x=Math.atan2(-h,_),this._y=0);break;default:console.warn("THREE.Euler: .setFromRotationMatrix() encountered an unknown order: "+e)}return this._order=e,i===!0&&this._onChangeCallback(),this}setFromQuaternion(t,e,i){return _o.makeRotationFromQuaternion(t),this.setFromRotationMatrix(_o,e,i)}setFromVector3(t,e=this._order){return this.set(t.x,t.y,t.z,e)}reorder(t){return go.setFromEuler(this),this.setFromQuaternion(go,t)}equals(t){return t._x===this._x&&t._y===this._y&&t._z===this._z&&t._order===this._order}fromArray(t){return this._x=t[0],this._y=t[1],this._z=t[2],t[3]!==void 0&&(this._order=t[3]),this._onChangeCallback(),this}toArray(t=[],e=0){return t[e]=this._x,t[e+1]=this._y,t[e+2]=this._z,t[e+3]=this._order,t}_onChange(t){return this._onChangeCallback=t,this}_onChangeCallback(){}*[Symbol.iterator](){yield this._x,yield this._y,yield this._z,yield this._order}}Ye.DEFAULT_ORDER="XYZ";class jc{constructor(){this.mask=1}set(t){this.mask=(1<<t|0)>>>0}enable(t){this.mask|=1<<t|0}enableAll(){this.mask=-1}toggle(t){this.mask^=1<<t|0}disable(t){this.mask&=~(1<<t|0)}disableAll(){this.mask=0}test(t){return(this.mask&t.mask)!==0}isEnabled(t){return(this.mask&(1<<t|0))!==0}}let rf=0;const xo=new H,oi=new Yi,nn=new te,or=new H,Ii=new H,sf=new H,af=new Yi,vo=new H(1,0,0),yo=new H(0,1,0),So=new H(0,0,1),Mo={type:"added"},of={type:"removed"},ci={type:"childadded",child:null},hs={type:"childremoved",child:null};class de extends Ti{constructor(){super(),this.isObject3D=!0,Object.defineProperty(this,"id",{value:rf++}),this.uuid=$i(),this.name="",this.type="Object3D",this.parent=null,this.children=[],this.up=de.DEFAULT_UP.clone();const t=new H,e=new Ye,i=new Yi,r=new H(1,1,1);function s(){i.setFromEuler(e,!1)}function o(){e.setFromQuaternion(i,void 0,!1)}e._onChange(s),i._onChange(o),Object.defineProperties(this,{position:{configurable:!0,enumerable:!0,value:t},rotation:{configurable:!0,enumerable:!0,value:e},quaternion:{configurable:!0,enumerable:!0,value:i},scale:{configurable:!0,enumerable:!0,value:r},modelViewMatrix:{value:new te},normalMatrix:{value:new Ft}}),this.matrix=new te,this.matrixWorld=new te,this.matrixAutoUpdate=de.DEFAULT_MATRIX_AUTO_UPDATE,this.matrixWorldAutoUpdate=de.DEFAULT_MATRIX_WORLD_AUTO_UPDATE,this.matrixWorldNeedsUpdate=!1,this.layers=new jc,this.visible=!0,this.castShadow=!1,this.receiveShadow=!1,this.frustumCulled=!0,this.renderOrder=0,this.animations=[],this.customDepthMaterial=void 0,this.customDistanceMaterial=void 0,this.userData={}}onBeforeShadow(){}onAfterShadow(){}onBeforeRender(){}onAfterRender(){}applyMatrix4(t){this.matrixAutoUpdate&&this.updateMatrix(),this.matrix.premultiply(t),this.matrix.decompose(this.position,this.quaternion,this.scale)}applyQuaternion(t){return this.quaternion.premultiply(t),this}setRotationFromAxisAngle(t,e){this.quaternion.setFromAxisAngle(t,e)}setRotationFromEuler(t){this.quaternion.setFromEuler(t,!0)}setRotationFromMatrix(t){this.quaternion.setFromRotationMatrix(t)}setRotationFromQuaternion(t){this.quaternion.copy(t)}rotateOnAxis(t,e){return oi.setFromAxisAngle(t,e),this.quaternion.multiply(oi),this}rotateOnWorldAxis(t,e){return oi.setFromAxisAngle(t,e),this.quaternion.premultiply(oi),this}rotateX(t){return this.rotateOnAxis(vo,t)}rotateY(t){return this.rotateOnAxis(yo,t)}rotateZ(t){return this.rotateOnAxis(So,t)}translateOnAxis(t,e){return xo.copy(t).applyQuaternion(this.quaternion),this.position.add(xo.multiplyScalar(e)),this}translateX(t){return this.translateOnAxis(vo,t)}translateY(t){return this.translateOnAxis(yo,t)}translateZ(t){return this.translateOnAxis(So,t)}localToWorld(t){return this.updateWorldMatrix(!0,!1),t.applyMatrix4(this.matrixWorld)}worldToLocal(t){return this.updateWorldMatrix(!0,!1),t.applyMatrix4(nn.copy(this.matrixWorld).invert())}lookAt(t,e,i){t.isVector3?or.copy(t):or.set(t,e,i);const r=this.parent;this.updateWorldMatrix(!0,!1),Ii.setFromMatrixPosition(this.matrixWorld),this.isCamera||this.isLight?nn.lookAt(Ii,or,this.up):nn.lookAt(or,Ii,this.up),this.quaternion.setFromRotationMatrix(nn),r&&(nn.extractRotation(r.matrixWorld),oi.setFromRotationMatrix(nn),this.quaternion.premultiply(oi.invert()))}add(t){if(arguments.length>1){for(let e=0;e<arguments.length;e++)this.add(arguments[e]);return this}return t===this?(console.error("THREE.Object3D.add: object can't be added as a child of itself.",t),this):(t&&t.isObject3D?(t.removeFromParent(),t.parent=this,this.children.push(t),t.dispatchEvent(Mo),ci.child=t,this.dispatchEvent(ci),ci.child=null):console.error("THREE.Object3D.add: object not an instance of THREE.Object3D.",t),this)}remove(t){if(arguments.length>1){for(let i=0;i<arguments.length;i++)this.remove(arguments[i]);return this}const e=this.children.indexOf(t);return e!==-1&&(t.parent=null,this.children.splice(e,1),t.dispatchEvent(of),hs.child=t,this.dispatchEvent(hs),hs.child=null),this}removeFromParent(){const t=this.parent;return t!==null&&t.remove(this),this}clear(){return this.remove(...this.children)}attach(t){return this.updateWorldMatrix(!0,!1),nn.copy(this.matrixWorld).invert(),t.parent!==null&&(t.parent.updateWorldMatrix(!0,!1),nn.multiply(t.parent.matrixWorld)),t.applyMatrix4(nn),t.removeFromParent(),t.parent=this,this.children.push(t),t.updateWorldMatrix(!1,!0),t.dispatchEvent(Mo),ci.child=t,this.dispatchEvent(ci),ci.child=null,this}getObjectById(t){return this.getObjectByProperty("id",t)}getObjectByName(t){return this.getObjectByProperty("name",t)}getObjectByProperty(t,e){if(this[t]===e)return this;for(let i=0,r=this.children.length;i<r;i++){const o=this.children[i].getObjectByProperty(t,e);if(o!==void 0)return o}}getObjectsByProperty(t,e,i=[]){this[t]===e&&i.push(this);const r=this.children;for(let s=0,o=r.length;s<o;s++)r[s].getObjectsByProperty(t,e,i);return i}getWorldPosition(t){return this.updateWorldMatrix(!0,!1),t.setFromMatrixPosition(this.matrixWorld)}getWorldQuaternion(t){return this.updateWorldMatrix(!0,!1),this.matrixWorld.decompose(Ii,t,sf),t}getWorldScale(t){return this.updateWorldMatrix(!0,!1),this.matrixWorld.decompose(Ii,af,t),t}getWorldDirection(t){this.updateWorldMatrix(!0,!1);const e=this.matrixWorld.elements;return t.set(e[8],e[9],e[10]).normalize()}raycast(){}traverse(t){t(this);const e=this.children;for(let i=0,r=e.length;i<r;i++)e[i].traverse(t)}traverseVisible(t){if(this.visible===!1)return;t(this);const e=this.children;for(let i=0,r=e.length;i<r;i++)e[i].traverseVisible(t)}traverseAncestors(t){const e=this.parent;e!==null&&(t(e),e.traverseAncestors(t))}updateMatrix(){this.matrix.compose(this.position,this.quaternion,this.scale),this.matrixWorldNeedsUpdate=!0}updateMatrixWorld(t){this.matrixAutoUpdate&&this.updateMatrix(),(this.matrixWorldNeedsUpdate||t)&&(this.matrixWorldAutoUpdate===!0&&(this.parent===null?this.matrixWorld.copy(this.matrix):this.matrixWorld.multiplyMatrices(this.parent.matrixWorld,this.matrix)),this.matrixWorldNeedsUpdate=!1,t=!0);const e=this.children;for(let i=0,r=e.length;i<r;i++)e[i].updateMatrixWorld(t)}updateWorldMatrix(t,e){const i=this.parent;if(t===!0&&i!==null&&i.updateWorldMatrix(!0,!1),this.matrixAutoUpdate&&this.updateMatrix(),this.matrixWorldAutoUpdate===!0&&(this.parent===null?this.matrixWorld.copy(this.matrix):this.matrixWorld.multiplyMatrices(this.parent.matrixWorld,this.matrix)),e===!0){const r=this.children;for(let s=0,o=r.length;s<o;s++)r[s].updateWorldMatrix(!1,!0)}}toJSON(t){const e=t===void 0||typeof t=="string",i={};e&&(t={geometries:{},materials:{},textures:{},images:{},shapes:{},skeletons:{},animations:{},nodes:{}},i.metadata={version:4.6,type:"Object",generator:"Object3D.toJSON"});const r={};r.uuid=this.uuid,r.type=this.type,this.name!==""&&(r.name=this.name),this.castShadow===!0&&(r.castShadow=!0),this.receiveShadow===!0&&(r.receiveShadow=!0),this.visible===!1&&(r.visible=!1),this.frustumCulled===!1&&(r.frustumCulled=!1),this.renderOrder!==0&&(r.renderOrder=this.renderOrder),Object.keys(this.userData).length>0&&(r.userData=this.userData),r.layers=this.layers.mask,r.matrix=this.matrix.toArray(),r.up=this.up.toArray(),this.matrixAutoUpdate===!1&&(r.matrixAutoUpdate=!1),this.isInstancedMesh&&(r.type="InstancedMesh",r.count=this.count,r.instanceMatrix=this.instanceMatrix.toJSON(),this.instanceColor!==null&&(r.instanceColor=this.instanceColor.toJSON())),this.isBatchedMesh&&(r.type="BatchedMesh",r.perObjectFrustumCulled=this.perObjectFrustumCulled,r.sortObjects=this.sortObjects,r.drawRanges=this._drawRanges,r.reservedRanges=this._reservedRanges,r.geometryInfo=this._geometryInfo.map(a=>({...a,boundingBox:a.boundingBox?{min:a.boundingBox.min.toArray(),max:a.boundingBox.max.toArray()}:void 0,boundingSphere:a.boundingSphere?{radius:a.boundingSphere.radius,center:a.boundingSphere.center.toArray()}:void 0})),r.instanceInfo=this._instanceInfo.map(a=>({...a})),r.availableInstanceIds=this._availableInstanceIds.slice(),r.availableGeometryIds=this._availableGeometryIds.slice(),r.nextIndexStart=this._nextIndexStart,r.nextVertexStart=this._nextVertexStart,r.geometryCount=this._geometryCount,r.maxInstanceCount=this._maxInstanceCount,r.maxVertexCount=this._maxVertexCount,r.maxIndexCount=this._maxIndexCount,r.geometryInitialized=this._geometryInitialized,r.matricesTexture=this._matricesTexture.toJSON(t),r.indirectTexture=this._indirectTexture.toJSON(t),this._colorsTexture!==null&&(r.colorsTexture=this._colorsTexture.toJSON(t)),this.boundingSphere!==null&&(r.boundingSphere={center:this.boundingSphere.center.toArray(),radius:this.boundingSphere.radius}),this.boundingBox!==null&&(r.boundingBox={min:this.boundingBox.min.toArray(),max:this.boundingBox.max.toArray()}));function s(a,l){return a[l.uuid]===void 0&&(a[l.uuid]=l.toJSON(t)),l.uuid}if(this.isScene)this.background&&(this.background.isColor?r.background=this.background.toJSON():this.background.isTexture&&(r.background=this.background.toJSON(t).uuid)),this.environment&&this.environment.isTexture&&this.environment.isRenderTargetTexture!==!0&&(r.environment=this.environment.toJSON(t).uuid);else if(this.isMesh||this.isLine||this.isPoints){r.geometry=s(t.geometries,this.geometry);const a=this.geometry.parameters;if(a!==void 0&&a.shapes!==void 0){const l=a.shapes;if(Array.isArray(l))for(let u=0,h=l.length;u<h;u++){const p=l[u];s(t.shapes,p)}else s(t.shapes,l)}}if(this.isSkinnedMesh&&(r.bindMode=this.bindMode,r.bindMatrix=this.bindMatrix.toArray(),this.skeleton!==void 0&&(s(t.skeletons,this.skeleton),r.skeleton=this.skeleton.uuid)),this.material!==void 0)if(Array.isArray(this.material)){const a=[];for(let l=0,u=this.material.length;l<u;l++)a.push(s(t.materials,this.material[l]));r.material=a}else r.material=s(t.materials,this.material);if(this.children.length>0){r.children=[];for(let a=0;a<this.children.length;a++)r.children.push(this.children[a].toJSON(t).object)}if(this.animations.length>0){r.animations=[];for(let a=0;a<this.animations.length;a++){const l=this.animations[a];r.animations.push(s(t.animations,l))}}if(e){const a=o(t.geometries),l=o(t.materials),u=o(t.textures),h=o(t.images),p=o(t.shapes),m=o(t.skeletons),_=o(t.animations),x=o(t.nodes);a.length>0&&(i.geometries=a),l.length>0&&(i.materials=l),u.length>0&&(i.textures=u),h.length>0&&(i.images=h),p.length>0&&(i.shapes=p),m.length>0&&(i.skeletons=m),_.length>0&&(i.animations=_),x.length>0&&(i.nodes=x)}return i.object=r,i;function o(a){const l=[];for(const u in a){const h=a[u];delete h.metadata,l.push(h)}return l}}clone(t){return new this.constructor().copy(this,t)}copy(t,e=!0){if(this.name=t.name,this.up.copy(t.up),this.position.copy(t.position),this.rotation.order=t.rotation.order,this.quaternion.copy(t.quaternion),this.scale.copy(t.scale),this.matrix.copy(t.matrix),this.matrixWorld.copy(t.matrixWorld),this.matrixAutoUpdate=t.matrixAutoUpdate,this.matrixWorldAutoUpdate=t.matrixWorldAutoUpdate,this.matrixWorldNeedsUpdate=t.matrixWorldNeedsUpdate,this.layers.mask=t.layers.mask,this.visible=t.visible,this.castShadow=t.castShadow,this.receiveShadow=t.receiveShadow,this.frustumCulled=t.frustumCulled,this.renderOrder=t.renderOrder,this.animations=t.animations.slice(),this.userData=JSON.parse(JSON.stringify(t.userData)),e===!0)for(let i=0;i<t.children.length;i++){const r=t.children[i];this.add(r.clone())}return this}}de.DEFAULT_UP=new H(0,1,0);de.DEFAULT_MATRIX_AUTO_UPDATE=!0;de.DEFAULT_MATRIX_WORLD_AUTO_UPDATE=!0;const He=new H,rn=new H,ds=new H,sn=new H,li=new H,ui=new H,Eo=new H,ps=new H,ms=new H,_s=new H,gs=new se,xs=new se,vs=new se;class Ve{constructor(t=new H,e=new H,i=new H){this.a=t,this.b=e,this.c=i}static getNormal(t,e,i,r){r.subVectors(i,e),He.subVectors(t,e),r.cross(He);const s=r.lengthSq();return s>0?r.multiplyScalar(1/Math.sqrt(s)):r.set(0,0,0)}static getBarycoord(t,e,i,r,s){He.subVectors(r,e),rn.subVectors(i,e),ds.subVectors(t,e);const o=He.dot(He),a=He.dot(rn),l=He.dot(ds),u=rn.dot(rn),h=rn.dot(ds),p=o*u-a*a;if(p===0)return s.set(0,0,0),null;const m=1/p,_=(u*l-a*h)*m,x=(o*h-a*l)*m;return s.set(1-_-x,x,_)}static containsPoint(t,e,i,r){return this.getBarycoord(t,e,i,r,sn)===null?!1:sn.x>=0&&sn.y>=0&&sn.x+sn.y<=1}static getInterpolation(t,e,i,r,s,o,a,l){return this.getBarycoord(t,e,i,r,sn)===null?(l.x=0,l.y=0,"z"in l&&(l.z=0),"w"in l&&(l.w=0),null):(l.setScalar(0),l.addScaledVector(s,sn.x),l.addScaledVector(o,sn.y),l.addScaledVector(a,sn.z),l)}static getInterpolatedAttribute(t,e,i,r,s,o){return gs.setScalar(0),xs.setScalar(0),vs.setScalar(0),gs.fromBufferAttribute(t,e),xs.fromBufferAttribute(t,i),vs.fromBufferAttribute(t,r),o.setScalar(0),o.addScaledVector(gs,s.x),o.addScaledVector(xs,s.y),o.addScaledVector(vs,s.z),o}static isFrontFacing(t,e,i,r){return He.subVectors(i,e),rn.subVectors(t,e),He.cross(rn).dot(r)<0}set(t,e,i){return this.a.copy(t),this.b.copy(e),this.c.copy(i),this}setFromPointsAndIndices(t,e,i,r){return this.a.copy(t[e]),this.b.copy(t[i]),this.c.copy(t[r]),this}setFromAttributeAndIndices(t,e,i,r){return this.a.fromBufferAttribute(t,e),this.b.fromBufferAttribute(t,i),this.c.fromBufferAttribute(t,r),this}clone(){return new this.constructor().copy(this)}copy(t){return this.a.copy(t.a),this.b.copy(t.b),this.c.copy(t.c),this}getArea(){return He.subVectors(this.c,this.b),rn.subVectors(this.a,this.b),He.cross(rn).length()*.5}getMidpoint(t){return t.addVectors(this.a,this.b).add(this.c).multiplyScalar(1/3)}getNormal(t){return Ve.getNormal(this.a,this.b,this.c,t)}getPlane(t){return t.setFromCoplanarPoints(this.a,this.b,this.c)}getBarycoord(t,e){return Ve.getBarycoord(t,this.a,this.b,this.c,e)}getInterpolation(t,e,i,r,s){return Ve.getInterpolation(t,this.a,this.b,this.c,e,i,r,s)}containsPoint(t){return Ve.containsPoint(t,this.a,this.b,this.c)}isFrontFacing(t){return Ve.isFrontFacing(this.a,this.b,this.c,t)}intersectsBox(t){return t.intersectsTriangle(this)}closestPointToPoint(t,e){const i=this.a,r=this.b,s=this.c;let o,a;li.subVectors(r,i),ui.subVectors(s,i),ps.subVectors(t,i);const l=li.dot(ps),u=ui.dot(ps);if(l<=0&&u<=0)return e.copy(i);ms.subVectors(t,r);const h=li.dot(ms),p=ui.dot(ms);if(h>=0&&p<=h)return e.copy(r);const m=l*p-h*u;if(m<=0&&l>=0&&h<=0)return o=l/(l-h),e.copy(i).addScaledVector(li,o);_s.subVectors(t,s);const _=li.dot(_s),x=ui.dot(_s);if(x>=0&&_<=x)return e.copy(s);const y=_*u-l*x;if(y<=0&&u>=0&&x<=0)return a=u/(u-x),e.copy(i).addScaledVector(ui,a);const g=h*x-_*p;if(g<=0&&p-h>=0&&_-x>=0)return Eo.subVectors(s,r),a=(p-h)/(p-h+(_-x)),e.copy(r).addScaledVector(Eo,a);const d=1/(g+y+m);return o=y*d,a=m*d,e.copy(i).addScaledVector(li,o).addScaledVector(ui,a)}equals(t){return t.a.equals(this.a)&&t.b.equals(this.b)&&t.c.equals(this.c)}}const Kc={aliceblue:15792383,antiquewhite:16444375,aqua:65535,aquamarine:8388564,azure:15794175,beige:16119260,bisque:16770244,black:0,blanchedalmond:16772045,blue:255,blueviolet:9055202,brown:10824234,burlywood:14596231,cadetblue:6266528,chartreuse:8388352,chocolate:13789470,coral:16744272,cornflowerblue:6591981,cornsilk:16775388,crimson:14423100,cyan:65535,darkblue:139,darkcyan:35723,darkgoldenrod:12092939,darkgray:11119017,darkgreen:25600,darkgrey:11119017,darkkhaki:12433259,darkmagenta:9109643,darkolivegreen:5597999,darkorange:16747520,darkorchid:10040012,darkred:9109504,darksalmon:15308410,darkseagreen:9419919,darkslateblue:4734347,darkslategray:3100495,darkslategrey:3100495,darkturquoise:52945,darkviolet:9699539,deeppink:16716947,deepskyblue:49151,dimgray:6908265,dimgrey:6908265,dodgerblue:2003199,firebrick:11674146,floralwhite:16775920,forestgreen:2263842,fuchsia:16711935,gainsboro:14474460,ghostwhite:16316671,gold:16766720,goldenrod:14329120,gray:8421504,green:32768,greenyellow:11403055,grey:8421504,honeydew:15794160,hotpink:16738740,indianred:13458524,indigo:4915330,ivory:16777200,khaki:15787660,lavender:15132410,lavenderblush:16773365,lawngreen:8190976,lemonchiffon:16775885,lightblue:11393254,lightcoral:15761536,lightcyan:14745599,lightgoldenrodyellow:16448210,lightgray:13882323,lightgreen:9498256,lightgrey:13882323,lightpink:16758465,lightsalmon:16752762,lightseagreen:2142890,lightskyblue:8900346,lightslategray:7833753,lightslategrey:7833753,lightsteelblue:11584734,lightyellow:16777184,lime:65280,limegreen:3329330,linen:16445670,magenta:16711935,maroon:8388608,mediumaquamarine:6737322,mediumblue:205,mediumorchid:12211667,mediumpurple:9662683,mediumseagreen:3978097,mediumslateblue:8087790,mediumspringgreen:64154,mediumturquoise:4772300,mediumvioletred:13047173,midnightblue:1644912,mintcream:16121850,mistyrose:16770273,moccasin:16770229,navajowhite:16768685,navy:128,oldlace:16643558,olive:8421376,olivedrab:7048739,orange:16753920,orangered:16729344,orchid:14315734,palegoldenrod:15657130,palegreen:10025880,paleturquoise:11529966,palevioletred:14381203,papayawhip:16773077,peachpuff:16767673,peru:13468991,pink:16761035,plum:14524637,powderblue:11591910,purple:8388736,rebeccapurple:6697881,red:16711680,rosybrown:12357519,royalblue:4286945,saddlebrown:9127187,salmon:16416882,sandybrown:16032864,seagreen:3050327,seashell:16774638,sienna:10506797,silver:12632256,skyblue:8900331,slateblue:6970061,slategray:7372944,slategrey:7372944,snow:16775930,springgreen:65407,steelblue:4620980,tan:13808780,teal:32896,thistle:14204888,tomato:16737095,turquoise:4251856,violet:15631086,wheat:16113331,white:16777215,whitesmoke:16119285,yellow:16776960,yellowgreen:10145074},En={h:0,s:0,l:0},cr={h:0,s:0,l:0};function ys(n,t,e){return e<0&&(e+=1),e>1&&(e-=1),e<1/6?n+(t-n)*6*e:e<1/2?t:e<2/3?n+(t-n)*6*(2/3-e):n}class Wt{constructor(t,e,i){return this.isColor=!0,this.r=1,this.g=1,this.b=1,this.set(t,e,i)}set(t,e,i){if(e===void 0&&i===void 0){const r=t;r&&r.isColor?this.copy(r):typeof r=="number"?this.setHex(r):typeof r=="string"&&this.setStyle(r)}else this.setRGB(t,e,i);return this}setScalar(t){return this.r=t,this.g=t,this.b=t,this}setHex(t,e=Fe){return t=Math.floor(t),this.r=(t>>16&255)/255,this.g=(t>>8&255)/255,this.b=(t&255)/255,Gt.toWorkingColorSpace(this,e),this}setRGB(t,e,i,r=Gt.workingColorSpace){return this.r=t,this.g=e,this.b=i,Gt.toWorkingColorSpace(this,r),this}setHSL(t,e,i,r=Gt.workingColorSpace){if(t=Gu(t,1),e=kt(e,0,1),i=kt(i,0,1),e===0)this.r=this.g=this.b=i;else{const s=i<=.5?i*(1+e):i+e-i*e,o=2*i-s;this.r=ys(o,s,t+1/3),this.g=ys(o,s,t),this.b=ys(o,s,t-1/3)}return Gt.toWorkingColorSpace(this,r),this}setStyle(t,e=Fe){function i(s){s!==void 0&&parseFloat(s)<1&&console.warn("THREE.Color: Alpha component of "+t+" will be ignored.")}let r;if(r=/^(\w+)\(([^\)]*)\)/.exec(t)){let s;const o=r[1],a=r[2];switch(o){case"rgb":case"rgba":if(s=/^\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(a))return i(s[4]),this.setRGB(Math.min(255,parseInt(s[1],10))/255,Math.min(255,parseInt(s[2],10))/255,Math.min(255,parseInt(s[3],10))/255,e);if(s=/^\s*(\d+)\%\s*,\s*(\d+)\%\s*,\s*(\d+)\%\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(a))return i(s[4]),this.setRGB(Math.min(100,parseInt(s[1],10))/100,Math.min(100,parseInt(s[2],10))/100,Math.min(100,parseInt(s[3],10))/100,e);break;case"hsl":case"hsla":if(s=/^\s*(\d*\.?\d+)\s*,\s*(\d*\.?\d+)\%\s*,\s*(\d*\.?\d+)\%\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(a))return i(s[4]),this.setHSL(parseFloat(s[1])/360,parseFloat(s[2])/100,parseFloat(s[3])/100,e);break;default:console.warn("THREE.Color: Unknown color model "+t)}}else if(r=/^\#([A-Fa-f\d]+)$/.exec(t)){const s=r[1],o=s.length;if(o===3)return this.setRGB(parseInt(s.charAt(0),16)/15,parseInt(s.charAt(1),16)/15,parseInt(s.charAt(2),16)/15,e);if(o===6)return this.setHex(parseInt(s,16),e);console.warn("THREE.Color: Invalid hex color "+t)}else if(t&&t.length>0)return this.setColorName(t,e);return this}setColorName(t,e=Fe){const i=Kc[t.toLowerCase()];return i!==void 0?this.setHex(i,e):console.warn("THREE.Color: Unknown color "+t),this}clone(){return new this.constructor(this.r,this.g,this.b)}copy(t){return this.r=t.r,this.g=t.g,this.b=t.b,this}copySRGBToLinear(t){return this.r=dn(t.r),this.g=dn(t.g),this.b=dn(t.b),this}copyLinearToSRGB(t){return this.r=gi(t.r),this.g=gi(t.g),this.b=gi(t.b),this}convertSRGBToLinear(){return this.copySRGBToLinear(this),this}convertLinearToSRGB(){return this.copyLinearToSRGB(this),this}getHex(t=Fe){return Gt.fromWorkingColorSpace(_e.copy(this),t),Math.round(kt(_e.r*255,0,255))*65536+Math.round(kt(_e.g*255,0,255))*256+Math.round(kt(_e.b*255,0,255))}getHexString(t=Fe){return("000000"+this.getHex(t).toString(16)).slice(-6)}getHSL(t,e=Gt.workingColorSpace){Gt.fromWorkingColorSpace(_e.copy(this),e);const i=_e.r,r=_e.g,s=_e.b,o=Math.max(i,r,s),a=Math.min(i,r,s);let l,u;const h=(a+o)/2;if(a===o)l=0,u=0;else{const p=o-a;switch(u=h<=.5?p/(o+a):p/(2-o-a),o){case i:l=(r-s)/p+(r<s?6:0);break;case r:l=(s-i)/p+2;break;case s:l=(i-r)/p+4;break}l/=6}return t.h=l,t.s=u,t.l=h,t}getRGB(t,e=Gt.workingColorSpace){return Gt.fromWorkingColorSpace(_e.copy(this),e),t.r=_e.r,t.g=_e.g,t.b=_e.b,t}getStyle(t=Fe){Gt.fromWorkingColorSpace(_e.copy(this),t);const e=_e.r,i=_e.g,r=_e.b;return t!==Fe?`color(${t} ${e.toFixed(3)} ${i.toFixed(3)} ${r.toFixed(3)})`:`rgb(${Math.round(e*255)},${Math.round(i*255)},${Math.round(r*255)})`}offsetHSL(t,e,i){return this.getHSL(En),this.setHSL(En.h+t,En.s+e,En.l+i)}add(t){return this.r+=t.r,this.g+=t.g,this.b+=t.b,this}addColors(t,e){return this.r=t.r+e.r,this.g=t.g+e.g,this.b=t.b+e.b,this}addScalar(t){return this.r+=t,this.g+=t,this.b+=t,this}sub(t){return this.r=Math.max(0,this.r-t.r),this.g=Math.max(0,this.g-t.g),this.b=Math.max(0,this.b-t.b),this}multiply(t){return this.r*=t.r,this.g*=t.g,this.b*=t.b,this}multiplyScalar(t){return this.r*=t,this.g*=t,this.b*=t,this}lerp(t,e){return this.r+=(t.r-this.r)*e,this.g+=(t.g-this.g)*e,this.b+=(t.b-this.b)*e,this}lerpColors(t,e,i){return this.r=t.r+(e.r-t.r)*i,this.g=t.g+(e.g-t.g)*i,this.b=t.b+(e.b-t.b)*i,this}lerpHSL(t,e){this.getHSL(En),t.getHSL(cr);const i=is(En.h,cr.h,e),r=is(En.s,cr.s,e),s=is(En.l,cr.l,e);return this.setHSL(i,r,s),this}setFromVector3(t){return this.r=t.x,this.g=t.y,this.b=t.z,this}applyMatrix3(t){const e=this.r,i=this.g,r=this.b,s=t.elements;return this.r=s[0]*e+s[3]*i+s[6]*r,this.g=s[1]*e+s[4]*i+s[7]*r,this.b=s[2]*e+s[5]*i+s[8]*r,this}equals(t){return t.r===this.r&&t.g===this.g&&t.b===this.b}fromArray(t,e=0){return this.r=t[e],this.g=t[e+1],this.b=t[e+2],this}toArray(t=[],e=0){return t[e]=this.r,t[e+1]=this.g,t[e+2]=this.b,t}fromBufferAttribute(t,e){return this.r=t.getX(e),this.g=t.getY(e),this.b=t.getZ(e),this}toJSON(){return this.getHex()}*[Symbol.iterator](){yield this.r,yield this.g,yield this.b}}const _e=new Wt;Wt.NAMES=Kc;let cf=0;class Jn extends Ti{constructor(){super(),this.isMaterial=!0,Object.defineProperty(this,"id",{value:cf++}),this.uuid=$i(),this.name="",this.type="Material",this.blending=_i,this.side=Pn,this.vertexColors=!1,this.opacity=1,this.transparent=!1,this.alphaHash=!1,this.blendSrc=zs,this.blendDst=ks,this.blendEquation=Wn,this.blendSrcAlpha=null,this.blendDstAlpha=null,this.blendEquationAlpha=null,this.blendColor=new Wt(0,0,0),this.blendAlpha=0,this.depthFunc=xi,this.depthTest=!0,this.depthWrite=!0,this.stencilWriteMask=255,this.stencilFunc=co,this.stencilRef=0,this.stencilFuncMask=255,this.stencilFail=ei,this.stencilZFail=ei,this.stencilZPass=ei,this.stencilWrite=!1,this.clippingPlanes=null,this.clipIntersection=!1,this.clipShadows=!1,this.shadowSide=null,this.colorWrite=!0,this.precision=null,this.polygonOffset=!1,this.polygonOffsetFactor=0,this.polygonOffsetUnits=0,this.dithering=!1,this.alphaToCoverage=!1,this.premultipliedAlpha=!1,this.forceSinglePass=!1,this.allowOverride=!0,this.visible=!0,this.toneMapped=!0,this.userData={},this.version=0,this._alphaTest=0}get alphaTest(){return this._alphaTest}set alphaTest(t){this._alphaTest>0!=t>0&&this.version++,this._alphaTest=t}onBeforeRender(){}onBeforeCompile(){}customProgramCacheKey(){return this.onBeforeCompile.toString()}setValues(t){if(t!==void 0)for(const e in t){const i=t[e];if(i===void 0){console.warn(`THREE.Material: parameter '${e}' has value of undefined.`);continue}const r=this[e];if(r===void 0){console.warn(`THREE.Material: '${e}' is not a property of THREE.${this.type}.`);continue}r&&r.isColor?r.set(i):r&&r.isVector3&&i&&i.isVector3?r.copy(i):this[e]=i}}toJSON(t){const e=t===void 0||typeof t=="string";e&&(t={textures:{},images:{}});const i={metadata:{version:4.6,type:"Material",generator:"Material.toJSON"}};i.uuid=this.uuid,i.type=this.type,this.name!==""&&(i.name=this.name),this.color&&this.color.isColor&&(i.color=this.color.getHex()),this.roughness!==void 0&&(i.roughness=this.roughness),this.metalness!==void 0&&(i.metalness=this.metalness),this.sheen!==void 0&&(i.sheen=this.sheen),this.sheenColor&&this.sheenColor.isColor&&(i.sheenColor=this.sheenColor.getHex()),this.sheenRoughness!==void 0&&(i.sheenRoughness=this.sheenRoughness),this.emissive&&this.emissive.isColor&&(i.emissive=this.emissive.getHex()),this.emissiveIntensity!==void 0&&this.emissiveIntensity!==1&&(i.emissiveIntensity=this.emissiveIntensity),this.specular&&this.specular.isColor&&(i.specular=this.specular.getHex()),this.specularIntensity!==void 0&&(i.specularIntensity=this.specularIntensity),this.specularColor&&this.specularColor.isColor&&(i.specularColor=this.specularColor.getHex()),this.shininess!==void 0&&(i.shininess=this.shininess),this.clearcoat!==void 0&&(i.clearcoat=this.clearcoat),this.clearcoatRoughness!==void 0&&(i.clearcoatRoughness=this.clearcoatRoughness),this.clearcoatMap&&this.clearcoatMap.isTexture&&(i.clearcoatMap=this.clearcoatMap.toJSON(t).uuid),this.clearcoatRoughnessMap&&this.clearcoatRoughnessMap.isTexture&&(i.clearcoatRoughnessMap=this.clearcoatRoughnessMap.toJSON(t).uuid),this.clearcoatNormalMap&&this.clearcoatNormalMap.isTexture&&(i.clearcoatNormalMap=this.clearcoatNormalMap.toJSON(t).uuid,i.clearcoatNormalScale=this.clearcoatNormalScale.toArray()),this.dispersion!==void 0&&(i.dispersion=this.dispersion),this.iridescence!==void 0&&(i.iridescence=this.iridescence),this.iridescenceIOR!==void 0&&(i.iridescenceIOR=this.iridescenceIOR),this.iridescenceThicknessRange!==void 0&&(i.iridescenceThicknessRange=this.iridescenceThicknessRange),this.iridescenceMap&&this.iridescenceMap.isTexture&&(i.iridescenceMap=this.iridescenceMap.toJSON(t).uuid),this.iridescenceThicknessMap&&this.iridescenceThicknessMap.isTexture&&(i.iridescenceThicknessMap=this.iridescenceThicknessMap.toJSON(t).uuid),this.anisotropy!==void 0&&(i.anisotropy=this.anisotropy),this.anisotropyRotation!==void 0&&(i.anisotropyRotation=this.anisotropyRotation),this.anisotropyMap&&this.anisotropyMap.isTexture&&(i.anisotropyMap=this.anisotropyMap.toJSON(t).uuid),this.map&&this.map.isTexture&&(i.map=this.map.toJSON(t).uuid),this.matcap&&this.matcap.isTexture&&(i.matcap=this.matcap.toJSON(t).uuid),this.alphaMap&&this.alphaMap.isTexture&&(i.alphaMap=this.alphaMap.toJSON(t).uuid),this.lightMap&&this.lightMap.isTexture&&(i.lightMap=this.lightMap.toJSON(t).uuid,i.lightMapIntensity=this.lightMapIntensity),this.aoMap&&this.aoMap.isTexture&&(i.aoMap=this.aoMap.toJSON(t).uuid,i.aoMapIntensity=this.aoMapIntensity),this.bumpMap&&this.bumpMap.isTexture&&(i.bumpMap=this.bumpMap.toJSON(t).uuid,i.bumpScale=this.bumpScale),this.normalMap&&this.normalMap.isTexture&&(i.normalMap=this.normalMap.toJSON(t).uuid,i.normalMapType=this.normalMapType,i.normalScale=this.normalScale.toArray()),this.displacementMap&&this.displacementMap.isTexture&&(i.displacementMap=this.displacementMap.toJSON(t).uuid,i.displacementScale=this.displacementScale,i.displacementBias=this.displacementBias),this.roughnessMap&&this.roughnessMap.isTexture&&(i.roughnessMap=this.roughnessMap.toJSON(t).uuid),this.metalnessMap&&this.metalnessMap.isTexture&&(i.metalnessMap=this.metalnessMap.toJSON(t).uuid),this.emissiveMap&&this.emissiveMap.isTexture&&(i.emissiveMap=this.emissiveMap.toJSON(t).uuid),this.specularMap&&this.specularMap.isTexture&&(i.specularMap=this.specularMap.toJSON(t).uuid),this.specularIntensityMap&&this.specularIntensityMap.isTexture&&(i.specularIntensityMap=this.specularIntensityMap.toJSON(t).uuid),this.specularColorMap&&this.specularColorMap.isTexture&&(i.specularColorMap=this.specularColorMap.toJSON(t).uuid),this.envMap&&this.envMap.isTexture&&(i.envMap=this.envMap.toJSON(t).uuid,this.combine!==void 0&&(i.combine=this.combine)),this.envMapRotation!==void 0&&(i.envMapRotation=this.envMapRotation.toArray()),this.envMapIntensity!==void 0&&(i.envMapIntensity=this.envMapIntensity),this.reflectivity!==void 0&&(i.reflectivity=this.reflectivity),this.refractionRatio!==void 0&&(i.refractionRatio=this.refractionRatio),this.gradientMap&&this.gradientMap.isTexture&&(i.gradientMap=this.gradientMap.toJSON(t).uuid),this.transmission!==void 0&&(i.transmission=this.transmission),this.transmissionMap&&this.transmissionMap.isTexture&&(i.transmissionMap=this.transmissionMap.toJSON(t).uuid),this.thickness!==void 0&&(i.thickness=this.thickness),this.thicknessMap&&this.thicknessMap.isTexture&&(i.thicknessMap=this.thicknessMap.toJSON(t).uuid),this.attenuationDistance!==void 0&&this.attenuationDistance!==1/0&&(i.attenuationDistance=this.attenuationDistance),this.attenuationColor!==void 0&&(i.attenuationColor=this.attenuationColor.getHex()),this.size!==void 0&&(i.size=this.size),this.shadowSide!==null&&(i.shadowSide=this.shadowSide),this.sizeAttenuation!==void 0&&(i.sizeAttenuation=this.sizeAttenuation),this.blending!==_i&&(i.blending=this.blending),this.side!==Pn&&(i.side=this.side),this.vertexColors===!0&&(i.vertexColors=!0),this.opacity<1&&(i.opacity=this.opacity),this.transparent===!0&&(i.transparent=!0),this.blendSrc!==zs&&(i.blendSrc=this.blendSrc),this.blendDst!==ks&&(i.blendDst=this.blendDst),this.blendEquation!==Wn&&(i.blendEquation=this.blendEquation),this.blendSrcAlpha!==null&&(i.blendSrcAlpha=this.blendSrcAlpha),this.blendDstAlpha!==null&&(i.blendDstAlpha=this.blendDstAlpha),this.blendEquationAlpha!==null&&(i.blendEquationAlpha=this.blendEquationAlpha),this.blendColor&&this.blendColor.isColor&&(i.blendColor=this.blendColor.getHex()),this.blendAlpha!==0&&(i.blendAlpha=this.blendAlpha),this.depthFunc!==xi&&(i.depthFunc=this.depthFunc),this.depthTest===!1&&(i.depthTest=this.depthTest),this.depthWrite===!1&&(i.depthWrite=this.depthWrite),this.colorWrite===!1&&(i.colorWrite=this.colorWrite),this.stencilWriteMask!==255&&(i.stencilWriteMask=this.stencilWriteMask),this.stencilFunc!==co&&(i.stencilFunc=this.stencilFunc),this.stencilRef!==0&&(i.stencilRef=this.stencilRef),this.stencilFuncMask!==255&&(i.stencilFuncMask=this.stencilFuncMask),this.stencilFail!==ei&&(i.stencilFail=this.stencilFail),this.stencilZFail!==ei&&(i.stencilZFail=this.stencilZFail),this.stencilZPass!==ei&&(i.stencilZPass=this.stencilZPass),this.stencilWrite===!0&&(i.stencilWrite=this.stencilWrite),this.rotation!==void 0&&this.rotation!==0&&(i.rotation=this.rotation),this.polygonOffset===!0&&(i.polygonOffset=!0),this.polygonOffsetFactor!==0&&(i.polygonOffsetFactor=this.polygonOffsetFactor),this.polygonOffsetUnits!==0&&(i.polygonOffsetUnits=this.polygonOffsetUnits),this.linewidth!==void 0&&this.linewidth!==1&&(i.linewidth=this.linewidth),this.dashSize!==void 0&&(i.dashSize=this.dashSize),this.gapSize!==void 0&&(i.gapSize=this.gapSize),this.scale!==void 0&&(i.scale=this.scale),this.dithering===!0&&(i.dithering=!0),this.alphaTest>0&&(i.alphaTest=this.alphaTest),this.alphaHash===!0&&(i.alphaHash=!0),this.alphaToCoverage===!0&&(i.alphaToCoverage=!0),this.premultipliedAlpha===!0&&(i.premultipliedAlpha=!0),this.forceSinglePass===!0&&(i.forceSinglePass=!0),this.wireframe===!0&&(i.wireframe=!0),this.wireframeLinewidth>1&&(i.wireframeLinewidth=this.wireframeLinewidth),this.wireframeLinecap!=="round"&&(i.wireframeLinecap=this.wireframeLinecap),this.wireframeLinejoin!=="round"&&(i.wireframeLinejoin=this.wireframeLinejoin),this.flatShading===!0&&(i.flatShading=!0),this.visible===!1&&(i.visible=!1),this.toneMapped===!1&&(i.toneMapped=!1),this.fog===!1&&(i.fog=!1),Object.keys(this.userData).length>0&&(i.userData=this.userData);function r(s){const o=[];for(const a in s){const l=s[a];delete l.metadata,o.push(l)}return o}if(e){const s=r(t.textures),o=r(t.images);s.length>0&&(i.textures=s),o.length>0&&(i.images=o)}return i}clone(){return new this.constructor().copy(this)}copy(t){this.name=t.name,this.blending=t.blending,this.side=t.side,this.vertexColors=t.vertexColors,this.opacity=t.opacity,this.transparent=t.transparent,this.blendSrc=t.blendSrc,this.blendDst=t.blendDst,this.blendEquation=t.blendEquation,this.blendSrcAlpha=t.blendSrcAlpha,this.blendDstAlpha=t.blendDstAlpha,this.blendEquationAlpha=t.blendEquationAlpha,this.blendColor.copy(t.blendColor),this.blendAlpha=t.blendAlpha,this.depthFunc=t.depthFunc,this.depthTest=t.depthTest,this.depthWrite=t.depthWrite,this.stencilWriteMask=t.stencilWriteMask,this.stencilFunc=t.stencilFunc,this.stencilRef=t.stencilRef,this.stencilFuncMask=t.stencilFuncMask,this.stencilFail=t.stencilFail,this.stencilZFail=t.stencilZFail,this.stencilZPass=t.stencilZPass,this.stencilWrite=t.stencilWrite;const e=t.clippingPlanes;let i=null;if(e!==null){const r=e.length;i=new Array(r);for(let s=0;s!==r;++s)i[s]=e[s].clone()}return this.clippingPlanes=i,this.clipIntersection=t.clipIntersection,this.clipShadows=t.clipShadows,this.shadowSide=t.shadowSide,this.colorWrite=t.colorWrite,this.precision=t.precision,this.polygonOffset=t.polygonOffset,this.polygonOffsetFactor=t.polygonOffsetFactor,this.polygonOffsetUnits=t.polygonOffsetUnits,this.dithering=t.dithering,this.alphaTest=t.alphaTest,this.alphaHash=t.alphaHash,this.alphaToCoverage=t.alphaToCoverage,this.premultipliedAlpha=t.premultipliedAlpha,this.forceSinglePass=t.forceSinglePass,this.visible=t.visible,this.toneMapped=t.toneMapped,this.userData=JSON.parse(JSON.stringify(t.userData)),this}dispose(){this.dispatchEvent({type:"dispose"})}set needsUpdate(t){t===!0&&this.version++}}class un extends Jn{constructor(t){super(),this.isMeshBasicMaterial=!0,this.type="MeshBasicMaterial",this.color=new Wt(16777215),this.map=null,this.lightMap=null,this.lightMapIntensity=1,this.aoMap=null,this.aoMapIntensity=1,this.specularMap=null,this.alphaMap=null,this.envMap=null,this.envMapRotation=new Ye,this.combine=Fc,this.reflectivity=1,this.refractionRatio=.98,this.wireframe=!1,this.wireframeLinewidth=1,this.wireframeLinecap="round",this.wireframeLinejoin="round",this.fog=!0,this.setValues(t)}copy(t){return super.copy(t),this.color.copy(t.color),this.map=t.map,this.lightMap=t.lightMap,this.lightMapIntensity=t.lightMapIntensity,this.aoMap=t.aoMap,this.aoMapIntensity=t.aoMapIntensity,this.specularMap=t.specularMap,this.alphaMap=t.alphaMap,this.envMap=t.envMap,this.envMapRotation.copy(t.envMapRotation),this.combine=t.combine,this.reflectivity=t.reflectivity,this.refractionRatio=t.refractionRatio,this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this.wireframeLinecap=t.wireframeLinecap,this.wireframeLinejoin=t.wireframeLinejoin,this.fog=t.fog,this}}const oe=new H,lr=new Xt;let lf=0;class $e{constructor(t,e,i=!1){if(Array.isArray(t))throw new TypeError("THREE.BufferAttribute: array should be a Typed Array.");this.isBufferAttribute=!0,Object.defineProperty(this,"id",{value:lf++}),this.name="",this.array=t,this.itemSize=e,this.count=t!==void 0?t.length/e:0,this.normalized=i,this.usage=lo,this.updateRanges=[],this.gpuType=cn,this.version=0}onUploadCallback(){}set needsUpdate(t){t===!0&&this.version++}setUsage(t){return this.usage=t,this}addUpdateRange(t,e){this.updateRanges.push({start:t,count:e})}clearUpdateRanges(){this.updateRanges.length=0}copy(t){return this.name=t.name,this.array=new t.array.constructor(t.array),this.itemSize=t.itemSize,this.count=t.count,this.normalized=t.normalized,this.usage=t.usage,this.gpuType=t.gpuType,this}copyAt(t,e,i){t*=this.itemSize,i*=e.itemSize;for(let r=0,s=this.itemSize;r<s;r++)this.array[t+r]=e.array[i+r];return this}copyArray(t){return this.array.set(t),this}applyMatrix3(t){if(this.itemSize===2)for(let e=0,i=this.count;e<i;e++)lr.fromBufferAttribute(this,e),lr.applyMatrix3(t),this.setXY(e,lr.x,lr.y);else if(this.itemSize===3)for(let e=0,i=this.count;e<i;e++)oe.fromBufferAttribute(this,e),oe.applyMatrix3(t),this.setXYZ(e,oe.x,oe.y,oe.z);return this}applyMatrix4(t){for(let e=0,i=this.count;e<i;e++)oe.fromBufferAttribute(this,e),oe.applyMatrix4(t),this.setXYZ(e,oe.x,oe.y,oe.z);return this}applyNormalMatrix(t){for(let e=0,i=this.count;e<i;e++)oe.fromBufferAttribute(this,e),oe.applyNormalMatrix(t),this.setXYZ(e,oe.x,oe.y,oe.z);return this}transformDirection(t){for(let e=0,i=this.count;e<i;e++)oe.fromBufferAttribute(this,e),oe.transformDirection(t),this.setXYZ(e,oe.x,oe.y,oe.z);return this}set(t,e=0){return this.array.set(t,e),this}getComponent(t,e){let i=this.array[t*this.itemSize+e];return this.normalized&&(i=Pi(i,this.array)),i}setComponent(t,e,i){return this.normalized&&(i=we(i,this.array)),this.array[t*this.itemSize+e]=i,this}getX(t){let e=this.array[t*this.itemSize];return this.normalized&&(e=Pi(e,this.array)),e}setX(t,e){return this.normalized&&(e=we(e,this.array)),this.array[t*this.itemSize]=e,this}getY(t){let e=this.array[t*this.itemSize+1];return this.normalized&&(e=Pi(e,this.array)),e}setY(t,e){return this.normalized&&(e=we(e,this.array)),this.array[t*this.itemSize+1]=e,this}getZ(t){let e=this.array[t*this.itemSize+2];return this.normalized&&(e=Pi(e,this.array)),e}setZ(t,e){return this.normalized&&(e=we(e,this.array)),this.array[t*this.itemSize+2]=e,this}getW(t){let e=this.array[t*this.itemSize+3];return this.normalized&&(e=Pi(e,this.array)),e}setW(t,e){return this.normalized&&(e=we(e,this.array)),this.array[t*this.itemSize+3]=e,this}setXY(t,e,i){return t*=this.itemSize,this.normalized&&(e=we(e,this.array),i=we(i,this.array)),this.array[t+0]=e,this.array[t+1]=i,this}setXYZ(t,e,i,r){return t*=this.itemSize,this.normalized&&(e=we(e,this.array),i=we(i,this.array),r=we(r,this.array)),this.array[t+0]=e,this.array[t+1]=i,this.array[t+2]=r,this}setXYZW(t,e,i,r,s){return t*=this.itemSize,this.normalized&&(e=we(e,this.array),i=we(i,this.array),r=we(r,this.array),s=we(s,this.array)),this.array[t+0]=e,this.array[t+1]=i,this.array[t+2]=r,this.array[t+3]=s,this}onUpload(t){return this.onUploadCallback=t,this}clone(){return new this.constructor(this.array,this.itemSize).copy(this)}toJSON(){const t={itemSize:this.itemSize,type:this.array.constructor.name,array:Array.from(this.array),normalized:this.normalized};return this.name!==""&&(t.name=this.name),this.usage!==lo&&(t.usage=this.usage),t}}class Zc extends $e{constructor(t,e,i){super(new Uint16Array(t),e,i)}}class Jc extends $e{constructor(t,e,i){super(new Uint32Array(t),e,i)}}class ye extends $e{constructor(t,e,i){super(new Float32Array(t),e,i)}}let uf=0;const Ue=new te,Ss=new de,fi=new H,Pe=new ji,Ui=new ji,fe=new H;class Le extends Ti{constructor(){super(),this.isBufferGeometry=!0,Object.defineProperty(this,"id",{value:uf++}),this.uuid=$i(),this.name="",this.type="BufferGeometry",this.index=null,this.indirect=null,this.attributes={},this.morphAttributes={},this.morphTargetsRelative=!1,this.groups=[],this.boundingBox=null,this.boundingSphere=null,this.drawRange={start:0,count:1/0},this.userData={}}getIndex(){return this.index}setIndex(t){return Array.isArray(t)?this.index=new(qc(t)?Jc:Zc)(t,1):this.index=t,this}setIndirect(t){return this.indirect=t,this}getIndirect(){return this.indirect}getAttribute(t){return this.attributes[t]}setAttribute(t,e){return this.attributes[t]=e,this}deleteAttribute(t){return delete this.attributes[t],this}hasAttribute(t){return this.attributes[t]!==void 0}addGroup(t,e,i=0){this.groups.push({start:t,count:e,materialIndex:i})}clearGroups(){this.groups=[]}setDrawRange(t,e){this.drawRange.start=t,this.drawRange.count=e}applyMatrix4(t){const e=this.attributes.position;e!==void 0&&(e.applyMatrix4(t),e.needsUpdate=!0);const i=this.attributes.normal;if(i!==void 0){const s=new Ft().getNormalMatrix(t);i.applyNormalMatrix(s),i.needsUpdate=!0}const r=this.attributes.tangent;return r!==void 0&&(r.transformDirection(t),r.needsUpdate=!0),this.boundingBox!==null&&this.computeBoundingBox(),this.boundingSphere!==null&&this.computeBoundingSphere(),this}applyQuaternion(t){return Ue.makeRotationFromQuaternion(t),this.applyMatrix4(Ue),this}rotateX(t){return Ue.makeRotationX(t),this.applyMatrix4(Ue),this}rotateY(t){return Ue.makeRotationY(t),this.applyMatrix4(Ue),this}rotateZ(t){return Ue.makeRotationZ(t),this.applyMatrix4(Ue),this}translate(t,e,i){return Ue.makeTranslation(t,e,i),this.applyMatrix4(Ue),this}scale(t,e,i){return Ue.makeScale(t,e,i),this.applyMatrix4(Ue),this}lookAt(t){return Ss.lookAt(t),Ss.updateMatrix(),this.applyMatrix4(Ss.matrix),this}center(){return this.computeBoundingBox(),this.boundingBox.getCenter(fi).negate(),this.translate(fi.x,fi.y,fi.z),this}setFromPoints(t){const e=this.getAttribute("position");if(e===void 0){const i=[];for(let r=0,s=t.length;r<s;r++){const o=t[r];i.push(o.x,o.y,o.z||0)}this.setAttribute("position",new ye(i,3))}else{const i=Math.min(t.length,e.count);for(let r=0;r<i;r++){const s=t[r];e.setXYZ(r,s.x,s.y,s.z||0)}t.length>e.count&&console.warn("THREE.BufferGeometry: Buffer size too small for points data. Use .dispose() and create a new geometry."),e.needsUpdate=!0}return this}computeBoundingBox(){this.boundingBox===null&&(this.boundingBox=new ji);const t=this.attributes.position,e=this.morphAttributes.position;if(t&&t.isGLBufferAttribute){console.error("THREE.BufferGeometry.computeBoundingBox(): GLBufferAttribute requires a manual bounding box.",this),this.boundingBox.set(new H(-1/0,-1/0,-1/0),new H(1/0,1/0,1/0));return}if(t!==void 0){if(this.boundingBox.setFromBufferAttribute(t),e)for(let i=0,r=e.length;i<r;i++){const s=e[i];Pe.setFromBufferAttribute(s),this.morphTargetsRelative?(fe.addVectors(this.boundingBox.min,Pe.min),this.boundingBox.expandByPoint(fe),fe.addVectors(this.boundingBox.max,Pe.max),this.boundingBox.expandByPoint(fe)):(this.boundingBox.expandByPoint(Pe.min),this.boundingBox.expandByPoint(Pe.max))}}else this.boundingBox.makeEmpty();(isNaN(this.boundingBox.min.x)||isNaN(this.boundingBox.min.y)||isNaN(this.boundingBox.min.z))&&console.error('THREE.BufferGeometry.computeBoundingBox(): Computed min/max have NaN values. The "position" attribute is likely to have NaN values.',this)}computeBoundingSphere(){this.boundingSphere===null&&(this.boundingSphere=new Wr);const t=this.attributes.position,e=this.morphAttributes.position;if(t&&t.isGLBufferAttribute){console.error("THREE.BufferGeometry.computeBoundingSphere(): GLBufferAttribute requires a manual bounding sphere.",this),this.boundingSphere.set(new H,1/0);return}if(t){const i=this.boundingSphere.center;if(Pe.setFromBufferAttribute(t),e)for(let s=0,o=e.length;s<o;s++){const a=e[s];Ui.setFromBufferAttribute(a),this.morphTargetsRelative?(fe.addVectors(Pe.min,Ui.min),Pe.expandByPoint(fe),fe.addVectors(Pe.max,Ui.max),Pe.expandByPoint(fe)):(Pe.expandByPoint(Ui.min),Pe.expandByPoint(Ui.max))}Pe.getCenter(i);let r=0;for(let s=0,o=t.count;s<o;s++)fe.fromBufferAttribute(t,s),r=Math.max(r,i.distanceToSquared(fe));if(e)for(let s=0,o=e.length;s<o;s++){const a=e[s],l=this.morphTargetsRelative;for(let u=0,h=a.count;u<h;u++)fe.fromBufferAttribute(a,u),l&&(fi.fromBufferAttribute(t,u),fe.add(fi)),r=Math.max(r,i.distanceToSquared(fe))}this.boundingSphere.radius=Math.sqrt(r),isNaN(this.boundingSphere.radius)&&console.error('THREE.BufferGeometry.computeBoundingSphere(): Computed radius is NaN. The "position" attribute is likely to have NaN values.',this)}}computeTangents(){const t=this.index,e=this.attributes;if(t===null||e.position===void 0||e.normal===void 0||e.uv===void 0){console.error("THREE.BufferGeometry: .computeTangents() failed. Missing required attributes (index, position, normal or uv)");return}const i=e.position,r=e.normal,s=e.uv;this.hasAttribute("tangent")===!1&&this.setAttribute("tangent",new $e(new Float32Array(4*i.count),4));const o=this.getAttribute("tangent"),a=[],l=[];for(let N=0;N<i.count;N++)a[N]=new H,l[N]=new H;const u=new H,h=new H,p=new H,m=new Xt,_=new Xt,x=new Xt,y=new H,g=new H;function d(N,A,T){u.fromBufferAttribute(i,N),h.fromBufferAttribute(i,A),p.fromBufferAttribute(i,T),m.fromBufferAttribute(s,N),_.fromBufferAttribute(s,A),x.fromBufferAttribute(s,T),h.sub(u),p.sub(u),_.sub(m),x.sub(m);const O=1/(_.x*x.y-x.x*_.y);isFinite(O)&&(y.copy(h).multiplyScalar(x.y).addScaledVector(p,-_.y).multiplyScalar(O),g.copy(p).multiplyScalar(_.x).addScaledVector(h,-x.x).multiplyScalar(O),a[N].add(y),a[A].add(y),a[T].add(y),l[N].add(g),l[A].add(g),l[T].add(g))}let P=this.groups;P.length===0&&(P=[{start:0,count:t.count}]);for(let N=0,A=P.length;N<A;++N){const T=P[N],O=T.start,$=T.count;for(let W=O,K=O+$;W<K;W+=3)d(t.getX(W+0),t.getX(W+1),t.getX(W+2))}const b=new H,w=new H,D=new H,L=new H;function I(N){D.fromBufferAttribute(r,N),L.copy(D);const A=a[N];b.copy(A),b.sub(D.multiplyScalar(D.dot(A))).normalize(),w.crossVectors(L,A);const O=w.dot(l[N])<0?-1:1;o.setXYZW(N,b.x,b.y,b.z,O)}for(let N=0,A=P.length;N<A;++N){const T=P[N],O=T.start,$=T.count;for(let W=O,K=O+$;W<K;W+=3)I(t.getX(W+0)),I(t.getX(W+1)),I(t.getX(W+2))}}computeVertexNormals(){const t=this.index,e=this.getAttribute("position");if(e!==void 0){let i=this.getAttribute("normal");if(i===void 0)i=new $e(new Float32Array(e.count*3),3),this.setAttribute("normal",i);else for(let m=0,_=i.count;m<_;m++)i.setXYZ(m,0,0,0);const r=new H,s=new H,o=new H,a=new H,l=new H,u=new H,h=new H,p=new H;if(t)for(let m=0,_=t.count;m<_;m+=3){const x=t.getX(m+0),y=t.getX(m+1),g=t.getX(m+2);r.fromBufferAttribute(e,x),s.fromBufferAttribute(e,y),o.fromBufferAttribute(e,g),h.subVectors(o,s),p.subVectors(r,s),h.cross(p),a.fromBufferAttribute(i,x),l.fromBufferAttribute(i,y),u.fromBufferAttribute(i,g),a.add(h),l.add(h),u.add(h),i.setXYZ(x,a.x,a.y,a.z),i.setXYZ(y,l.x,l.y,l.z),i.setXYZ(g,u.x,u.y,u.z)}else for(let m=0,_=e.count;m<_;m+=3)r.fromBufferAttribute(e,m+0),s.fromBufferAttribute(e,m+1),o.fromBufferAttribute(e,m+2),h.subVectors(o,s),p.subVectors(r,s),h.cross(p),i.setXYZ(m+0,h.x,h.y,h.z),i.setXYZ(m+1,h.x,h.y,h.z),i.setXYZ(m+2,h.x,h.y,h.z);this.normalizeNormals(),i.needsUpdate=!0}}normalizeNormals(){const t=this.attributes.normal;for(let e=0,i=t.count;e<i;e++)fe.fromBufferAttribute(t,e),fe.normalize(),t.setXYZ(e,fe.x,fe.y,fe.z)}toNonIndexed(){function t(a,l){const u=a.array,h=a.itemSize,p=a.normalized,m=new u.constructor(l.length*h);let _=0,x=0;for(let y=0,g=l.length;y<g;y++){a.isInterleavedBufferAttribute?_=l[y]*a.data.stride+a.offset:_=l[y]*h;for(let d=0;d<h;d++)m[x++]=u[_++]}return new $e(m,h,p)}if(this.index===null)return console.warn("THREE.BufferGeometry.toNonIndexed(): BufferGeometry is already non-indexed."),this;const e=new Le,i=this.index.array,r=this.attributes;for(const a in r){const l=r[a],u=t(l,i);e.setAttribute(a,u)}const s=this.morphAttributes;for(const a in s){const l=[],u=s[a];for(let h=0,p=u.length;h<p;h++){const m=u[h],_=t(m,i);l.push(_)}e.morphAttributes[a]=l}e.morphTargetsRelative=this.morphTargetsRelative;const o=this.groups;for(let a=0,l=o.length;a<l;a++){const u=o[a];e.addGroup(u.start,u.count,u.materialIndex)}return e}toJSON(){const t={metadata:{version:4.6,type:"BufferGeometry",generator:"BufferGeometry.toJSON"}};if(t.uuid=this.uuid,t.type=this.type,this.name!==""&&(t.name=this.name),Object.keys(this.userData).length>0&&(t.userData=this.userData),this.parameters!==void 0){const l=this.parameters;for(const u in l)l[u]!==void 0&&(t[u]=l[u]);return t}t.data={attributes:{}};const e=this.index;e!==null&&(t.data.index={type:e.array.constructor.name,array:Array.prototype.slice.call(e.array)});const i=this.attributes;for(const l in i){const u=i[l];t.data.attributes[l]=u.toJSON(t.data)}const r={};let s=!1;for(const l in this.morphAttributes){const u=this.morphAttributes[l],h=[];for(let p=0,m=u.length;p<m;p++){const _=u[p];h.push(_.toJSON(t.data))}h.length>0&&(r[l]=h,s=!0)}s&&(t.data.morphAttributes=r,t.data.morphTargetsRelative=this.morphTargetsRelative);const o=this.groups;o.length>0&&(t.data.groups=JSON.parse(JSON.stringify(o)));const a=this.boundingSphere;return a!==null&&(t.data.boundingSphere={center:a.center.toArray(),radius:a.radius}),t}clone(){return new this.constructor().copy(this)}copy(t){this.index=null,this.attributes={},this.morphAttributes={},this.groups=[],this.boundingBox=null,this.boundingSphere=null;const e={};this.name=t.name;const i=t.index;i!==null&&this.setIndex(i.clone());const r=t.attributes;for(const u in r){const h=r[u];this.setAttribute(u,h.clone(e))}const s=t.morphAttributes;for(const u in s){const h=[],p=s[u];for(let m=0,_=p.length;m<_;m++)h.push(p[m].clone(e));this.morphAttributes[u]=h}this.morphTargetsRelative=t.morphTargetsRelative;const o=t.groups;for(let u=0,h=o.length;u<h;u++){const p=o[u];this.addGroup(p.start,p.count,p.materialIndex)}const a=t.boundingBox;a!==null&&(this.boundingBox=a.clone());const l=t.boundingSphere;return l!==null&&(this.boundingSphere=l.clone()),this.drawRange.start=t.drawRange.start,this.drawRange.count=t.drawRange.count,this.userData=t.userData,this}dispose(){this.dispatchEvent({type:"dispose"})}}const To=new te,Bn=new Yc,ur=new Wr,wo=new H,fr=new H,hr=new H,dr=new H,Ms=new H,pr=new H,Ao=new H,mr=new H;class ce extends de{constructor(t=new Le,e=new un){super(),this.isMesh=!0,this.type="Mesh",this.geometry=t,this.material=e,this.morphTargetDictionary=void 0,this.morphTargetInfluences=void 0,this.updateMorphTargets()}copy(t,e){return super.copy(t,e),t.morphTargetInfluences!==void 0&&(this.morphTargetInfluences=t.morphTargetInfluences.slice()),t.morphTargetDictionary!==void 0&&(this.morphTargetDictionary=Object.assign({},t.morphTargetDictionary)),this.material=Array.isArray(t.material)?t.material.slice():t.material,this.geometry=t.geometry,this}updateMorphTargets(){const e=this.geometry.morphAttributes,i=Object.keys(e);if(i.length>0){const r=e[i[0]];if(r!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let s=0,o=r.length;s<o;s++){const a=r[s].name||String(s);this.morphTargetInfluences.push(0),this.morphTargetDictionary[a]=s}}}}getVertexPosition(t,e){const i=this.geometry,r=i.attributes.position,s=i.morphAttributes.position,o=i.morphTargetsRelative;e.fromBufferAttribute(r,t);const a=this.morphTargetInfluences;if(s&&a){pr.set(0,0,0);for(let l=0,u=s.length;l<u;l++){const h=a[l],p=s[l];h!==0&&(Ms.fromBufferAttribute(p,t),o?pr.addScaledVector(Ms,h):pr.addScaledVector(Ms.sub(e),h))}e.add(pr)}return e}raycast(t,e){const i=this.geometry,r=this.material,s=this.matrixWorld;r!==void 0&&(i.boundingSphere===null&&i.computeBoundingSphere(),ur.copy(i.boundingSphere),ur.applyMatrix4(s),Bn.copy(t.ray).recast(t.near),!(ur.containsPoint(Bn.origin)===!1&&(Bn.intersectSphere(ur,wo)===null||Bn.origin.distanceToSquared(wo)>(t.far-t.near)**2))&&(To.copy(s).invert(),Bn.copy(t.ray).applyMatrix4(To),!(i.boundingBox!==null&&Bn.intersectsBox(i.boundingBox)===!1)&&this._computeIntersections(t,e,Bn)))}_computeIntersections(t,e,i){let r;const s=this.geometry,o=this.material,a=s.index,l=s.attributes.position,u=s.attributes.uv,h=s.attributes.uv1,p=s.attributes.normal,m=s.groups,_=s.drawRange;if(a!==null)if(Array.isArray(o))for(let x=0,y=m.length;x<y;x++){const g=m[x],d=o[g.materialIndex],P=Math.max(g.start,_.start),b=Math.min(a.count,Math.min(g.start+g.count,_.start+_.count));for(let w=P,D=b;w<D;w+=3){const L=a.getX(w),I=a.getX(w+1),N=a.getX(w+2);r=_r(this,d,t,i,u,h,p,L,I,N),r&&(r.faceIndex=Math.floor(w/3),r.face.materialIndex=g.materialIndex,e.push(r))}}else{const x=Math.max(0,_.start),y=Math.min(a.count,_.start+_.count);for(let g=x,d=y;g<d;g+=3){const P=a.getX(g),b=a.getX(g+1),w=a.getX(g+2);r=_r(this,o,t,i,u,h,p,P,b,w),r&&(r.faceIndex=Math.floor(g/3),e.push(r))}}else if(l!==void 0)if(Array.isArray(o))for(let x=0,y=m.length;x<y;x++){const g=m[x],d=o[g.materialIndex],P=Math.max(g.start,_.start),b=Math.min(l.count,Math.min(g.start+g.count,_.start+_.count));for(let w=P,D=b;w<D;w+=3){const L=w,I=w+1,N=w+2;r=_r(this,d,t,i,u,h,p,L,I,N),r&&(r.faceIndex=Math.floor(w/3),r.face.materialIndex=g.materialIndex,e.push(r))}}else{const x=Math.max(0,_.start),y=Math.min(l.count,_.start+_.count);for(let g=x,d=y;g<d;g+=3){const P=g,b=g+1,w=g+2;r=_r(this,o,t,i,u,h,p,P,b,w),r&&(r.faceIndex=Math.floor(g/3),e.push(r))}}}}function ff(n,t,e,i,r,s,o,a){let l;if(t.side===Ae?l=i.intersectTriangle(o,s,r,!0,a):l=i.intersectTriangle(r,s,o,t.side===Pn,a),l===null)return null;mr.copy(a),mr.applyMatrix4(n.matrixWorld);const u=e.ray.origin.distanceTo(mr);return u<e.near||u>e.far?null:{distance:u,point:mr.clone(),object:n}}function _r(n,t,e,i,r,s,o,a,l,u){n.getVertexPosition(a,fr),n.getVertexPosition(l,hr),n.getVertexPosition(u,dr);const h=ff(n,t,e,i,fr,hr,dr,Ao);if(h){const p=new H;Ve.getBarycoord(Ao,fr,hr,dr,p),r&&(h.uv=Ve.getInterpolatedAttribute(r,a,l,u,p,new Xt)),s&&(h.uv1=Ve.getInterpolatedAttribute(s,a,l,u,p,new Xt)),o&&(h.normal=Ve.getInterpolatedAttribute(o,a,l,u,p,new H),h.normal.dot(i.direction)>0&&h.normal.multiplyScalar(-1));const m={a,b:l,c:u,normal:new H,materialIndex:0};Ve.getNormal(fr,hr,dr,m.normal),h.face=m,h.barycoord=p}return h}class Kn extends Le{constructor(t=1,e=1,i=1,r=1,s=1,o=1){super(),this.type="BoxGeometry",this.parameters={width:t,height:e,depth:i,widthSegments:r,heightSegments:s,depthSegments:o};const a=this;r=Math.floor(r),s=Math.floor(s),o=Math.floor(o);const l=[],u=[],h=[],p=[];let m=0,_=0;x("z","y","x",-1,-1,i,e,t,o,s,0),x("z","y","x",1,-1,i,e,-t,o,s,1),x("x","z","y",1,1,t,i,e,r,o,2),x("x","z","y",1,-1,t,i,-e,r,o,3),x("x","y","z",1,-1,t,e,i,r,s,4),x("x","y","z",-1,-1,t,e,-i,r,s,5),this.setIndex(l),this.setAttribute("position",new ye(u,3)),this.setAttribute("normal",new ye(h,3)),this.setAttribute("uv",new ye(p,2));function x(y,g,d,P,b,w,D,L,I,N,A){const T=w/I,O=D/N,$=w/2,W=D/2,K=L/2,it=I+1,Z=N+1;let at=0,Y=0;const ft=new H;for(let gt=0;gt<Z;gt++){const ht=gt*O-W;for(let bt=0;bt<it;bt++){const qt=bt*T-$;ft[y]=qt*P,ft[g]=ht*b,ft[d]=K,u.push(ft.x,ft.y,ft.z),ft[y]=0,ft[g]=0,ft[d]=L>0?1:-1,h.push(ft.x,ft.y,ft.z),p.push(bt/I),p.push(1-gt/N),at+=1}}for(let gt=0;gt<N;gt++)for(let ht=0;ht<I;ht++){const bt=m+ht+it*gt,qt=m+ht+it*(gt+1),Q=m+(ht+1)+it*(gt+1),ct=m+(ht+1)+it*gt;l.push(bt,qt,ct),l.push(qt,Q,ct),Y+=6}a.addGroup(_,Y,A),_+=Y,m+=at}}copy(t){return super.copy(t),this.parameters=Object.assign({},t.parameters),this}static fromJSON(t){return new Kn(t.width,t.height,t.depth,t.widthSegments,t.heightSegments,t.depthSegments)}}function Mi(n){const t={};for(const e in n){t[e]={};for(const i in n[e]){const r=n[e][i];r&&(r.isColor||r.isMatrix3||r.isMatrix4||r.isVector2||r.isVector3||r.isVector4||r.isTexture||r.isQuaternion)?r.isRenderTargetTexture?(console.warn("UniformsUtils: Textures of render targets cannot be cloned via cloneUniforms() or mergeUniforms()."),t[e][i]=null):t[e][i]=r.clone():Array.isArray(r)?t[e][i]=r.slice():t[e][i]=r}}return t}function Me(n){const t={};for(let e=0;e<n.length;e++){const i=Mi(n[e]);for(const r in i)t[r]=i[r]}return t}function hf(n){const t=[];for(let e=0;e<n.length;e++)t.push(n[e].clone());return t}function Qc(n){const t=n.getRenderTarget();return t===null?n.outputColorSpace:t.isXRRenderTarget===!0?t.texture.colorSpace:Gt.workingColorSpace}const df={clone:Mi,merge:Me};var pf=`void main() {
	gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
}`,mf=`void main() {
	gl_FragColor = vec4( 1.0, 0.0, 0.0, 1.0 );
}`;class Ln extends Jn{constructor(t){super(),this.isShaderMaterial=!0,this.type="ShaderMaterial",this.defines={},this.uniforms={},this.uniformsGroups=[],this.vertexShader=pf,this.fragmentShader=mf,this.linewidth=1,this.wireframe=!1,this.wireframeLinewidth=1,this.fog=!1,this.lights=!1,this.clipping=!1,this.forceSinglePass=!0,this.extensions={clipCullDistance:!1,multiDraw:!1},this.defaultAttributeValues={color:[1,1,1],uv:[0,0],uv1:[0,0]},this.index0AttributeName=void 0,this.uniformsNeedUpdate=!1,this.glslVersion=null,t!==void 0&&this.setValues(t)}copy(t){return super.copy(t),this.fragmentShader=t.fragmentShader,this.vertexShader=t.vertexShader,this.uniforms=Mi(t.uniforms),this.uniformsGroups=hf(t.uniformsGroups),this.defines=Object.assign({},t.defines),this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this.fog=t.fog,this.lights=t.lights,this.clipping=t.clipping,this.extensions=Object.assign({},t.extensions),this.glslVersion=t.glslVersion,this}toJSON(t){const e=super.toJSON(t);e.glslVersion=this.glslVersion,e.uniforms={};for(const r in this.uniforms){const o=this.uniforms[r].value;o&&o.isTexture?e.uniforms[r]={type:"t",value:o.toJSON(t).uuid}:o&&o.isColor?e.uniforms[r]={type:"c",value:o.getHex()}:o&&o.isVector2?e.uniforms[r]={type:"v2",value:o.toArray()}:o&&o.isVector3?e.uniforms[r]={type:"v3",value:o.toArray()}:o&&o.isVector4?e.uniforms[r]={type:"v4",value:o.toArray()}:o&&o.isMatrix3?e.uniforms[r]={type:"m3",value:o.toArray()}:o&&o.isMatrix4?e.uniforms[r]={type:"m4",value:o.toArray()}:e.uniforms[r]={value:o}}Object.keys(this.defines).length>0&&(e.defines=this.defines),e.vertexShader=this.vertexShader,e.fragmentShader=this.fragmentShader,e.lights=this.lights,e.clipping=this.clipping;const i={};for(const r in this.extensions)this.extensions[r]===!0&&(i[r]=!0);return Object.keys(i).length>0&&(e.extensions=i),e}}class tl extends de{constructor(){super(),this.isCamera=!0,this.type="Camera",this.matrixWorldInverse=new te,this.projectionMatrix=new te,this.projectionMatrixInverse=new te,this.coordinateSystem=ln}copy(t,e){return super.copy(t,e),this.matrixWorldInverse.copy(t.matrixWorldInverse),this.projectionMatrix.copy(t.projectionMatrix),this.projectionMatrixInverse.copy(t.projectionMatrixInverse),this.coordinateSystem=t.coordinateSystem,this}getWorldDirection(t){return super.getWorldDirection(t).negate()}updateMatrixWorld(t){super.updateMatrixWorld(t),this.matrixWorldInverse.copy(this.matrixWorld).invert()}updateWorldMatrix(t,e){super.updateWorldMatrix(t,e),this.matrixWorldInverse.copy(this.matrixWorld).invert()}clone(){return new this.constructor().copy(this)}}const Tn=new H,bo=new Xt,Co=new Xt;class Ne extends tl{constructor(t=50,e=1,i=.1,r=2e3){super(),this.isPerspectiveCamera=!0,this.type="PerspectiveCamera",this.fov=t,this.zoom=1,this.near=i,this.far=r,this.focus=10,this.aspect=e,this.view=null,this.filmGauge=35,this.filmOffset=0,this.updateProjectionMatrix()}copy(t,e){return super.copy(t,e),this.fov=t.fov,this.zoom=t.zoom,this.near=t.near,this.far=t.far,this.focus=t.focus,this.aspect=t.aspect,this.view=t.view===null?null:Object.assign({},t.view),this.filmGauge=t.filmGauge,this.filmOffset=t.filmOffset,this}setFocalLength(t){const e=.5*this.getFilmHeight()/t;this.fov=Ta*2*Math.atan(e),this.updateProjectionMatrix()}getFocalLength(){const t=Math.tan(ns*.5*this.fov);return .5*this.getFilmHeight()/t}getEffectiveFOV(){return Ta*2*Math.atan(Math.tan(ns*.5*this.fov)/this.zoom)}getFilmWidth(){return this.filmGauge*Math.min(this.aspect,1)}getFilmHeight(){return this.filmGauge/Math.max(this.aspect,1)}getViewBounds(t,e,i){Tn.set(-1,-1,.5).applyMatrix4(this.projectionMatrixInverse),e.set(Tn.x,Tn.y).multiplyScalar(-t/Tn.z),Tn.set(1,1,.5).applyMatrix4(this.projectionMatrixInverse),i.set(Tn.x,Tn.y).multiplyScalar(-t/Tn.z)}getViewSize(t,e){return this.getViewBounds(t,bo,Co),e.subVectors(Co,bo)}setViewOffset(t,e,i,r,s,o){this.aspect=t/e,this.view===null&&(this.view={enabled:!0,fullWidth:1,fullHeight:1,offsetX:0,offsetY:0,width:1,height:1}),this.view.enabled=!0,this.view.fullWidth=t,this.view.fullHeight=e,this.view.offsetX=i,this.view.offsetY=r,this.view.width=s,this.view.height=o,this.updateProjectionMatrix()}clearViewOffset(){this.view!==null&&(this.view.enabled=!1),this.updateProjectionMatrix()}updateProjectionMatrix(){const t=this.near;let e=t*Math.tan(ns*.5*this.fov)/this.zoom,i=2*e,r=this.aspect*i,s=-.5*r;const o=this.view;if(this.view!==null&&this.view.enabled){const l=o.fullWidth,u=o.fullHeight;s+=o.offsetX*r/l,e-=o.offsetY*i/u,r*=o.width/l,i*=o.height/u}const a=this.filmOffset;a!==0&&(s+=t*a/this.getFilmWidth()),this.projectionMatrix.makePerspective(s,s+r,e,e-i,t,this.far,this.coordinateSystem),this.projectionMatrixInverse.copy(this.projectionMatrix).invert()}toJSON(t){const e=super.toJSON(t);return e.object.fov=this.fov,e.object.zoom=this.zoom,e.object.near=this.near,e.object.far=this.far,e.object.focus=this.focus,e.object.aspect=this.aspect,this.view!==null&&(e.object.view=Object.assign({},this.view)),e.object.filmGauge=this.filmGauge,e.object.filmOffset=this.filmOffset,e}}const hi=-90,di=1;class _f extends de{constructor(t,e,i){super(),this.type="CubeCamera",this.renderTarget=i,this.coordinateSystem=null,this.activeMipmapLevel=0;const r=new Ne(hi,di,t,e);r.layers=this.layers,this.add(r);const s=new Ne(hi,di,t,e);s.layers=this.layers,this.add(s);const o=new Ne(hi,di,t,e);o.layers=this.layers,this.add(o);const a=new Ne(hi,di,t,e);a.layers=this.layers,this.add(a);const l=new Ne(hi,di,t,e);l.layers=this.layers,this.add(l);const u=new Ne(hi,di,t,e);u.layers=this.layers,this.add(u)}updateCoordinateSystem(){const t=this.coordinateSystem,e=this.children.concat(),[i,r,s,o,a,l]=e;for(const u of e)this.remove(u);if(t===ln)i.up.set(0,1,0),i.lookAt(1,0,0),r.up.set(0,1,0),r.lookAt(-1,0,0),s.up.set(0,0,-1),s.lookAt(0,1,0),o.up.set(0,0,1),o.lookAt(0,-1,0),a.up.set(0,1,0),a.lookAt(0,0,1),l.up.set(0,1,0),l.lookAt(0,0,-1);else if(t===Or)i.up.set(0,-1,0),i.lookAt(-1,0,0),r.up.set(0,-1,0),r.lookAt(1,0,0),s.up.set(0,0,1),s.lookAt(0,1,0),o.up.set(0,0,-1),o.lookAt(0,-1,0),a.up.set(0,-1,0),a.lookAt(0,0,1),l.up.set(0,-1,0),l.lookAt(0,0,-1);else throw new Error("THREE.CubeCamera.updateCoordinateSystem(): Invalid coordinate system: "+t);for(const u of e)this.add(u),u.updateMatrixWorld()}update(t,e){this.parent===null&&this.updateMatrixWorld();const{renderTarget:i,activeMipmapLevel:r}=this;this.coordinateSystem!==t.coordinateSystem&&(this.coordinateSystem=t.coordinateSystem,this.updateCoordinateSystem());const[s,o,a,l,u,h]=this.children,p=t.getRenderTarget(),m=t.getActiveCubeFace(),_=t.getActiveMipmapLevel(),x=t.xr.enabled;t.xr.enabled=!1;const y=i.texture.generateMipmaps;i.texture.generateMipmaps=!1,t.setRenderTarget(i,0,r),t.render(e,s),t.setRenderTarget(i,1,r),t.render(e,o),t.setRenderTarget(i,2,r),t.render(e,a),t.setRenderTarget(i,3,r),t.render(e,l),t.setRenderTarget(i,4,r),t.render(e,u),i.texture.generateMipmaps=y,t.setRenderTarget(i,5,r),t.render(e,h),t.setRenderTarget(p,m,_),t.xr.enabled=x,i.texture.needsPMREMUpdate=!0}}class el extends be{constructor(t=[],e=vi,i,r,s,o,a,l,u,h){super(t,e,i,r,s,o,a,l,u,h),this.isCubeTexture=!0,this.flipY=!1}get images(){return this.image}set images(t){this.image=t}}class gf extends jn{constructor(t=1,e={}){super(t,t,e),this.isWebGLCubeRenderTarget=!0;const i={width:t,height:t,depth:1},r=[i,i,i,i,i,i];this.texture=new el(r,e.mapping,e.wrapS,e.wrapT,e.magFilter,e.minFilter,e.format,e.type,e.anisotropy,e.colorSpace),this.texture.isRenderTargetTexture=!0,this.texture.generateMipmaps=e.generateMipmaps!==void 0?e.generateMipmaps:!1,this.texture.minFilter=e.minFilter!==void 0?e.minFilter:Ke}fromEquirectangularTexture(t,e){this.texture.type=e.type,this.texture.colorSpace=e.colorSpace,this.texture.generateMipmaps=e.generateMipmaps,this.texture.minFilter=e.minFilter,this.texture.magFilter=e.magFilter;const i={uniforms:{tEquirect:{value:null}},vertexShader:`

				varying vec3 vWorldDirection;

				vec3 transformDirection( in vec3 dir, in mat4 matrix ) {

					return normalize( ( matrix * vec4( dir, 0.0 ) ).xyz );

				}

				void main() {

					vWorldDirection = transformDirection( position, modelMatrix );

					#include <begin_vertex>
					#include <project_vertex>

				}
			`,fragmentShader:`

				uniform sampler2D tEquirect;

				varying vec3 vWorldDirection;

				#include <common>

				void main() {

					vec3 direction = normalize( vWorldDirection );

					vec2 sampleUV = equirectUv( direction );

					gl_FragColor = texture2D( tEquirect, sampleUV );

				}
			`},r=new Kn(5,5,5),s=new Ln({name:"CubemapFromEquirect",uniforms:Mi(i.uniforms),vertexShader:i.vertexShader,fragmentShader:i.fragmentShader,side:Ae,blending:Cn});s.uniforms.tEquirect.value=e;const o=new ce(r,s),a=e.minFilter;return e.minFilter===$n&&(e.minFilter=Ke),new _f(1,10,this).update(t,o),e.minFilter=a,o.geometry.dispose(),o.material.dispose(),this}clear(t,e=!0,i=!0,r=!0){const s=t.getRenderTarget();for(let o=0;o<6;o++)t.setRenderTarget(this,o),t.clear(e,i,r);t.setRenderTarget(s)}}class bn extends de{constructor(){super(),this.isGroup=!0,this.type="Group"}}const xf={type:"move"};class Es{constructor(){this._targetRay=null,this._grip=null,this._hand=null}getHandSpace(){return this._hand===null&&(this._hand=new bn,this._hand.matrixAutoUpdate=!1,this._hand.visible=!1,this._hand.joints={},this._hand.inputState={pinching:!1}),this._hand}getTargetRaySpace(){return this._targetRay===null&&(this._targetRay=new bn,this._targetRay.matrixAutoUpdate=!1,this._targetRay.visible=!1,this._targetRay.hasLinearVelocity=!1,this._targetRay.linearVelocity=new H,this._targetRay.hasAngularVelocity=!1,this._targetRay.angularVelocity=new H),this._targetRay}getGripSpace(){return this._grip===null&&(this._grip=new bn,this._grip.matrixAutoUpdate=!1,this._grip.visible=!1,this._grip.hasLinearVelocity=!1,this._grip.linearVelocity=new H,this._grip.hasAngularVelocity=!1,this._grip.angularVelocity=new H),this._grip}dispatchEvent(t){return this._targetRay!==null&&this._targetRay.dispatchEvent(t),this._grip!==null&&this._grip.dispatchEvent(t),this._hand!==null&&this._hand.dispatchEvent(t),this}connect(t){if(t&&t.hand){const e=this._hand;if(e)for(const i of t.hand.values())this._getHandJoint(e,i)}return this.dispatchEvent({type:"connected",data:t}),this}disconnect(t){return this.dispatchEvent({type:"disconnected",data:t}),this._targetRay!==null&&(this._targetRay.visible=!1),this._grip!==null&&(this._grip.visible=!1),this._hand!==null&&(this._hand.visible=!1),this}update(t,e,i){let r=null,s=null,o=null;const a=this._targetRay,l=this._grip,u=this._hand;if(t&&e.session.visibilityState!=="visible-blurred"){if(u&&t.hand){o=!0;for(const y of t.hand.values()){const g=e.getJointPose(y,i),d=this._getHandJoint(u,y);g!==null&&(d.matrix.fromArray(g.transform.matrix),d.matrix.decompose(d.position,d.rotation,d.scale),d.matrixWorldNeedsUpdate=!0,d.jointRadius=g.radius),d.visible=g!==null}const h=u.joints["index-finger-tip"],p=u.joints["thumb-tip"],m=h.position.distanceTo(p.position),_=.02,x=.005;u.inputState.pinching&&m>_+x?(u.inputState.pinching=!1,this.dispatchEvent({type:"pinchend",handedness:t.handedness,target:this})):!u.inputState.pinching&&m<=_-x&&(u.inputState.pinching=!0,this.dispatchEvent({type:"pinchstart",handedness:t.handedness,target:this}))}else l!==null&&t.gripSpace&&(s=e.getPose(t.gripSpace,i),s!==null&&(l.matrix.fromArray(s.transform.matrix),l.matrix.decompose(l.position,l.rotation,l.scale),l.matrixWorldNeedsUpdate=!0,s.linearVelocity?(l.hasLinearVelocity=!0,l.linearVelocity.copy(s.linearVelocity)):l.hasLinearVelocity=!1,s.angularVelocity?(l.hasAngularVelocity=!0,l.angularVelocity.copy(s.angularVelocity)):l.hasAngularVelocity=!1));a!==null&&(r=e.getPose(t.targetRaySpace,i),r===null&&s!==null&&(r=s),r!==null&&(a.matrix.fromArray(r.transform.matrix),a.matrix.decompose(a.position,a.rotation,a.scale),a.matrixWorldNeedsUpdate=!0,r.linearVelocity?(a.hasLinearVelocity=!0,a.linearVelocity.copy(r.linearVelocity)):a.hasLinearVelocity=!1,r.angularVelocity?(a.hasAngularVelocity=!0,a.angularVelocity.copy(r.angularVelocity)):a.hasAngularVelocity=!1,this.dispatchEvent(xf)))}return a!==null&&(a.visible=r!==null),l!==null&&(l.visible=s!==null),u!==null&&(u.visible=o!==null),this}_getHandJoint(t,e){if(t.joints[e.jointName]===void 0){const i=new bn;i.matrixAutoUpdate=!1,i.visible=!1,t.joints[e.jointName]=i,t.add(i)}return t.joints[e.jointName]}}class vf extends de{constructor(){super(),this.isScene=!0,this.type="Scene",this.background=null,this.environment=null,this.fog=null,this.backgroundBlurriness=0,this.backgroundIntensity=1,this.backgroundRotation=new Ye,this.environmentIntensity=1,this.environmentRotation=new Ye,this.overrideMaterial=null,typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("observe",{detail:this}))}copy(t,e){return super.copy(t,e),t.background!==null&&(this.background=t.background.clone()),t.environment!==null&&(this.environment=t.environment.clone()),t.fog!==null&&(this.fog=t.fog.clone()),this.backgroundBlurriness=t.backgroundBlurriness,this.backgroundIntensity=t.backgroundIntensity,this.backgroundRotation.copy(t.backgroundRotation),this.environmentIntensity=t.environmentIntensity,this.environmentRotation.copy(t.environmentRotation),t.overrideMaterial!==null&&(this.overrideMaterial=t.overrideMaterial.clone()),this.matrixAutoUpdate=t.matrixAutoUpdate,this}toJSON(t){const e=super.toJSON(t);return this.fog!==null&&(e.object.fog=this.fog.toJSON()),this.backgroundBlurriness>0&&(e.object.backgroundBlurriness=this.backgroundBlurriness),this.backgroundIntensity!==1&&(e.object.backgroundIntensity=this.backgroundIntensity),e.object.backgroundRotation=this.backgroundRotation.toArray(),this.environmentIntensity!==1&&(e.object.environmentIntensity=this.environmentIntensity),e.object.environmentRotation=this.environmentRotation.toArray(),e}}const Ts=new H,yf=new H,Sf=new Ft;class Vn{constructor(t=new H(1,0,0),e=0){this.isPlane=!0,this.normal=t,this.constant=e}set(t,e){return this.normal.copy(t),this.constant=e,this}setComponents(t,e,i,r){return this.normal.set(t,e,i),this.constant=r,this}setFromNormalAndCoplanarPoint(t,e){return this.normal.copy(t),this.constant=-e.dot(this.normal),this}setFromCoplanarPoints(t,e,i){const r=Ts.subVectors(i,e).cross(yf.subVectors(t,e)).normalize();return this.setFromNormalAndCoplanarPoint(r,t),this}copy(t){return this.normal.copy(t.normal),this.constant=t.constant,this}normalize(){const t=1/this.normal.length();return this.normal.multiplyScalar(t),this.constant*=t,this}negate(){return this.constant*=-1,this.normal.negate(),this}distanceToPoint(t){return this.normal.dot(t)+this.constant}distanceToSphere(t){return this.distanceToPoint(t.center)-t.radius}projectPoint(t,e){return e.copy(t).addScaledVector(this.normal,-this.distanceToPoint(t))}intersectLine(t,e){const i=t.delta(Ts),r=this.normal.dot(i);if(r===0)return this.distanceToPoint(t.start)===0?e.copy(t.start):null;const s=-(t.start.dot(this.normal)+this.constant)/r;return s<0||s>1?null:e.copy(t.start).addScaledVector(i,s)}intersectsLine(t){const e=this.distanceToPoint(t.start),i=this.distanceToPoint(t.end);return e<0&&i>0||i<0&&e>0}intersectsBox(t){return t.intersectsPlane(this)}intersectsSphere(t){return t.intersectsPlane(this)}coplanarPoint(t){return t.copy(this.normal).multiplyScalar(-this.constant)}applyMatrix4(t,e){const i=e||Sf.getNormalMatrix(t),r=this.coplanarPoint(Ts).applyMatrix4(t),s=this.normal.applyMatrix3(i).normalize();return this.constant=-r.dot(s),this}translate(t){return this.constant-=t.dot(this.normal),this}equals(t){return t.normal.equals(this.normal)&&t.constant===this.constant}clone(){return new this.constructor().copy(this)}}const zn=new Wr,gr=new H;class ka{constructor(t=new Vn,e=new Vn,i=new Vn,r=new Vn,s=new Vn,o=new Vn){this.planes=[t,e,i,r,s,o]}set(t,e,i,r,s,o){const a=this.planes;return a[0].copy(t),a[1].copy(e),a[2].copy(i),a[3].copy(r),a[4].copy(s),a[5].copy(o),this}copy(t){const e=this.planes;for(let i=0;i<6;i++)e[i].copy(t.planes[i]);return this}setFromProjectionMatrix(t,e=ln){const i=this.planes,r=t.elements,s=r[0],o=r[1],a=r[2],l=r[3],u=r[4],h=r[5],p=r[6],m=r[7],_=r[8],x=r[9],y=r[10],g=r[11],d=r[12],P=r[13],b=r[14],w=r[15];if(i[0].setComponents(l-s,m-u,g-_,w-d).normalize(),i[1].setComponents(l+s,m+u,g+_,w+d).normalize(),i[2].setComponents(l+o,m+h,g+x,w+P).normalize(),i[3].setComponents(l-o,m-h,g-x,w-P).normalize(),i[4].setComponents(l-a,m-p,g-y,w-b).normalize(),e===ln)i[5].setComponents(l+a,m+p,g+y,w+b).normalize();else if(e===Or)i[5].setComponents(a,p,y,b).normalize();else throw new Error("THREE.Frustum.setFromProjectionMatrix(): Invalid coordinate system: "+e);return this}intersectsObject(t){if(t.boundingSphere!==void 0)t.boundingSphere===null&&t.computeBoundingSphere(),zn.copy(t.boundingSphere).applyMatrix4(t.matrixWorld);else{const e=t.geometry;e.boundingSphere===null&&e.computeBoundingSphere(),zn.copy(e.boundingSphere).applyMatrix4(t.matrixWorld)}return this.intersectsSphere(zn)}intersectsSprite(t){return zn.center.set(0,0,0),zn.radius=.7071067811865476,zn.applyMatrix4(t.matrixWorld),this.intersectsSphere(zn)}intersectsSphere(t){const e=this.planes,i=t.center,r=-t.radius;for(let s=0;s<6;s++)if(e[s].distanceToPoint(i)<r)return!1;return!0}intersectsBox(t){const e=this.planes;for(let i=0;i<6;i++){const r=e[i];if(gr.x=r.normal.x>0?t.max.x:t.min.x,gr.y=r.normal.y>0?t.max.y:t.min.y,gr.z=r.normal.z>0?t.max.z:t.min.z,r.distanceToPoint(gr)<0)return!1}return!0}containsPoint(t){const e=this.planes;for(let i=0;i<6;i++)if(e[i].distanceToPoint(t)<0)return!1;return!0}clone(){return new this.constructor().copy(this)}}class nl extends Jn{constructor(t){super(),this.isLineBasicMaterial=!0,this.type="LineBasicMaterial",this.color=new Wt(16777215),this.map=null,this.linewidth=1,this.linecap="round",this.linejoin="round",this.fog=!0,this.setValues(t)}copy(t){return super.copy(t),this.color.copy(t.color),this.map=t.map,this.linewidth=t.linewidth,this.linecap=t.linecap,this.linejoin=t.linejoin,this.fog=t.fog,this}}const zr=new H,kr=new H,Ro=new te,Fi=new Yc,xr=new Wr,ws=new H,Po=new H;class Mf extends de{constructor(t=new Le,e=new nl){super(),this.isLine=!0,this.type="Line",this.geometry=t,this.material=e,this.morphTargetDictionary=void 0,this.morphTargetInfluences=void 0,this.updateMorphTargets()}copy(t,e){return super.copy(t,e),this.material=Array.isArray(t.material)?t.material.slice():t.material,this.geometry=t.geometry,this}computeLineDistances(){const t=this.geometry;if(t.index===null){const e=t.attributes.position,i=[0];for(let r=1,s=e.count;r<s;r++)zr.fromBufferAttribute(e,r-1),kr.fromBufferAttribute(e,r),i[r]=i[r-1],i[r]+=zr.distanceTo(kr);t.setAttribute("lineDistance",new ye(i,1))}else console.warn("THREE.Line.computeLineDistances(): Computation only possible with non-indexed BufferGeometry.");return this}raycast(t,e){const i=this.geometry,r=this.matrixWorld,s=t.params.Line.threshold,o=i.drawRange;if(i.boundingSphere===null&&i.computeBoundingSphere(),xr.copy(i.boundingSphere),xr.applyMatrix4(r),xr.radius+=s,t.ray.intersectsSphere(xr)===!1)return;Ro.copy(r).invert(),Fi.copy(t.ray).applyMatrix4(Ro);const a=s/((this.scale.x+this.scale.y+this.scale.z)/3),l=a*a,u=this.isLineSegments?2:1,h=i.index,m=i.attributes.position;if(h!==null){const _=Math.max(0,o.start),x=Math.min(h.count,o.start+o.count);for(let y=_,g=x-1;y<g;y+=u){const d=h.getX(y),P=h.getX(y+1),b=vr(this,t,Fi,l,d,P,y);b&&e.push(b)}if(this.isLineLoop){const y=h.getX(x-1),g=h.getX(_),d=vr(this,t,Fi,l,y,g,x-1);d&&e.push(d)}}else{const _=Math.max(0,o.start),x=Math.min(m.count,o.start+o.count);for(let y=_,g=x-1;y<g;y+=u){const d=vr(this,t,Fi,l,y,y+1,y);d&&e.push(d)}if(this.isLineLoop){const y=vr(this,t,Fi,l,x-1,_,x-1);y&&e.push(y)}}}updateMorphTargets(){const e=this.geometry.morphAttributes,i=Object.keys(e);if(i.length>0){const r=e[i[0]];if(r!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let s=0,o=r.length;s<o;s++){const a=r[s].name||String(s);this.morphTargetInfluences.push(0),this.morphTargetDictionary[a]=s}}}}}function vr(n,t,e,i,r,s,o){const a=n.geometry.attributes.position;if(zr.fromBufferAttribute(a,r),kr.fromBufferAttribute(a,s),e.distanceSqToSegment(zr,kr,ws,Po)>i)return;ws.applyMatrix4(n.matrixWorld);const u=t.ray.origin.distanceTo(ws);if(!(u<t.near||u>t.far))return{distance:u,point:Po.clone().applyMatrix4(n.matrixWorld),index:o,face:null,faceIndex:null,barycoord:null,object:n}}class il extends be{constructor(t,e,i=Yn,r,s,o,a=qe,l=qe,u,h=Vi){if(h!==Vi&&h!==Gi)throw new Error("DepthTexture format must be either THREE.DepthFormat or THREE.DepthStencilFormat");super(null,r,s,o,a,l,h,i,u),this.isDepthTexture=!0,this.image={width:t,height:e},this.flipY=!1,this.generateMipmaps=!1,this.compareFunction=null}copy(t){return super.copy(t),this.source=new za(Object.assign({},t.image)),this.compareFunction=t.compareFunction,this}toJSON(t){const e=super.toJSON(t);return this.compareFunction!==null&&(e.compareFunction=this.compareFunction),e}}class Xr extends Le{constructor(t=1,e=1,i=1,r=32,s=1,o=!1,a=0,l=Math.PI*2){super(),this.type="CylinderGeometry",this.parameters={radiusTop:t,radiusBottom:e,height:i,radialSegments:r,heightSegments:s,openEnded:o,thetaStart:a,thetaLength:l};const u=this;r=Math.floor(r),s=Math.floor(s);const h=[],p=[],m=[],_=[];let x=0;const y=[],g=i/2;let d=0;P(),o===!1&&(t>0&&b(!0),e>0&&b(!1)),this.setIndex(h),this.setAttribute("position",new ye(p,3)),this.setAttribute("normal",new ye(m,3)),this.setAttribute("uv",new ye(_,2));function P(){const w=new H,D=new H;let L=0;const I=(e-t)/i;for(let N=0;N<=s;N++){const A=[],T=N/s,O=T*(e-t)+t;for(let $=0;$<=r;$++){const W=$/r,K=W*l+a,it=Math.sin(K),Z=Math.cos(K);D.x=O*it,D.y=-T*i+g,D.z=O*Z,p.push(D.x,D.y,D.z),w.set(it,I,Z).normalize(),m.push(w.x,w.y,w.z),_.push(W,1-T),A.push(x++)}y.push(A)}for(let N=0;N<r;N++)for(let A=0;A<s;A++){const T=y[A][N],O=y[A+1][N],$=y[A+1][N+1],W=y[A][N+1];(t>0||A!==0)&&(h.push(T,O,W),L+=3),(e>0||A!==s-1)&&(h.push(O,$,W),L+=3)}u.addGroup(d,L,0),d+=L}function b(w){const D=x,L=new Xt,I=new H;let N=0;const A=w===!0?t:e,T=w===!0?1:-1;for(let $=1;$<=r;$++)p.push(0,g*T,0),m.push(0,T,0),_.push(.5,.5),x++;const O=x;for(let $=0;$<=r;$++){const K=$/r*l+a,it=Math.cos(K),Z=Math.sin(K);I.x=A*Z,I.y=g*T,I.z=A*it,p.push(I.x,I.y,I.z),m.push(0,T,0),L.x=it*.5+.5,L.y=Z*.5*T+.5,_.push(L.x,L.y),x++}for(let $=0;$<r;$++){const W=D+$,K=O+$;w===!0?h.push(K,K+1,W):h.push(K+1,K,W),N+=3}u.addGroup(d,N,w===!0?1:2),d+=N}}copy(t){return super.copy(t),this.parameters=Object.assign({},t.parameters),this}static fromJSON(t){return new Xr(t.radiusTop,t.radiusBottom,t.height,t.radialSegments,t.heightSegments,t.openEnded,t.thetaStart,t.thetaLength)}}class Ki extends Le{constructor(t=1,e=1,i=1,r=1){super(),this.type="PlaneGeometry",this.parameters={width:t,height:e,widthSegments:i,heightSegments:r};const s=t/2,o=e/2,a=Math.floor(i),l=Math.floor(r),u=a+1,h=l+1,p=t/a,m=e/l,_=[],x=[],y=[],g=[];for(let d=0;d<h;d++){const P=d*m-o;for(let b=0;b<u;b++){const w=b*p-s;x.push(w,-P,0),y.push(0,0,1),g.push(b/a),g.push(1-d/l)}}for(let d=0;d<l;d++)for(let P=0;P<a;P++){const b=P+u*d,w=P+u*(d+1),D=P+1+u*(d+1),L=P+1+u*d;_.push(b,w,L),_.push(w,D,L)}this.setIndex(_),this.setAttribute("position",new ye(x,3)),this.setAttribute("normal",new ye(y,3)),this.setAttribute("uv",new ye(g,2))}copy(t){return super.copy(t),this.parameters=Object.assign({},t.parameters),this}static fromJSON(t){return new Ki(t.width,t.height,t.widthSegments,t.heightSegments)}}class Hr extends Le{constructor(t=1,e=32,i=16,r=0,s=Math.PI*2,o=0,a=Math.PI){super(),this.type="SphereGeometry",this.parameters={radius:t,widthSegments:e,heightSegments:i,phiStart:r,phiLength:s,thetaStart:o,thetaLength:a},e=Math.max(3,Math.floor(e)),i=Math.max(2,Math.floor(i));const l=Math.min(o+a,Math.PI);let u=0;const h=[],p=new H,m=new H,_=[],x=[],y=[],g=[];for(let d=0;d<=i;d++){const P=[],b=d/i;let w=0;d===0&&o===0?w=.5/e:d===i&&l===Math.PI&&(w=-.5/e);for(let D=0;D<=e;D++){const L=D/e;p.x=-t*Math.cos(r+L*s)*Math.sin(o+b*a),p.y=t*Math.cos(o+b*a),p.z=t*Math.sin(r+L*s)*Math.sin(o+b*a),x.push(p.x,p.y,p.z),m.copy(p).normalize(),y.push(m.x,m.y,m.z),g.push(L+w,1-b),P.push(u++)}h.push(P)}for(let d=0;d<i;d++)for(let P=0;P<e;P++){const b=h[d][P+1],w=h[d][P],D=h[d+1][P],L=h[d+1][P+1];(d!==0||o>0)&&_.push(b,w,L),(d!==i-1||l<Math.PI)&&_.push(w,D,L)}this.setIndex(_),this.setAttribute("position",new ye(x,3)),this.setAttribute("normal",new ye(y,3)),this.setAttribute("uv",new ye(g,2))}copy(t){return super.copy(t),this.parameters=Object.assign({},t.parameters),this}static fromJSON(t){return new Hr(t.radius,t.widthSegments,t.heightSegments,t.phiStart,t.phiLength,t.thetaStart,t.thetaLength)}}class Lo extends Jn{constructor(t){super(),this.isMeshStandardMaterial=!0,this.type="MeshStandardMaterial",this.defines={STANDARD:""},this.color=new Wt(16777215),this.roughness=1,this.metalness=0,this.map=null,this.lightMap=null,this.lightMapIntensity=1,this.aoMap=null,this.aoMapIntensity=1,this.emissive=new Wt(0),this.emissiveIntensity=1,this.emissiveMap=null,this.bumpMap=null,this.bumpScale=1,this.normalMap=null,this.normalMapType=Ba,this.normalScale=new Xt(1,1),this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.roughnessMap=null,this.metalnessMap=null,this.alphaMap=null,this.envMap=null,this.envMapRotation=new Ye,this.envMapIntensity=1,this.wireframe=!1,this.wireframeLinewidth=1,this.wireframeLinecap="round",this.wireframeLinejoin="round",this.flatShading=!1,this.fog=!0,this.setValues(t)}copy(t){return super.copy(t),this.defines={STANDARD:""},this.color.copy(t.color),this.roughness=t.roughness,this.metalness=t.metalness,this.map=t.map,this.lightMap=t.lightMap,this.lightMapIntensity=t.lightMapIntensity,this.aoMap=t.aoMap,this.aoMapIntensity=t.aoMapIntensity,this.emissive.copy(t.emissive),this.emissiveMap=t.emissiveMap,this.emissiveIntensity=t.emissiveIntensity,this.bumpMap=t.bumpMap,this.bumpScale=t.bumpScale,this.normalMap=t.normalMap,this.normalMapType=t.normalMapType,this.normalScale.copy(t.normalScale),this.displacementMap=t.displacementMap,this.displacementScale=t.displacementScale,this.displacementBias=t.displacementBias,this.roughnessMap=t.roughnessMap,this.metalnessMap=t.metalnessMap,this.alphaMap=t.alphaMap,this.envMap=t.envMap,this.envMapRotation.copy(t.envMapRotation),this.envMapIntensity=t.envMapIntensity,this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this.wireframeLinecap=t.wireframeLinecap,this.wireframeLinejoin=t.wireframeLinejoin,this.flatShading=t.flatShading,this.fog=t.fog,this}}class Do extends Jn{constructor(t){super(),this.isMeshNormalMaterial=!0,this.type="MeshNormalMaterial",this.bumpMap=null,this.bumpScale=1,this.normalMap=null,this.normalMapType=Ba,this.normalScale=new Xt(1,1),this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.wireframe=!1,this.wireframeLinewidth=1,this.flatShading=!1,this.setValues(t)}copy(t){return super.copy(t),this.bumpMap=t.bumpMap,this.bumpScale=t.bumpScale,this.normalMap=t.normalMap,this.normalMapType=t.normalMapType,this.normalScale.copy(t.normalScale),this.displacementMap=t.displacementMap,this.displacementScale=t.displacementScale,this.displacementBias=t.displacementBias,this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this.flatShading=t.flatShading,this}}class Ef extends Jn{constructor(t){super(),this.isMeshDepthMaterial=!0,this.type="MeshDepthMaterial",this.depthPacking=Iu,this.map=null,this.alphaMap=null,this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.wireframe=!1,this.wireframeLinewidth=1,this.setValues(t)}copy(t){return super.copy(t),this.depthPacking=t.depthPacking,this.map=t.map,this.alphaMap=t.alphaMap,this.displacementMap=t.displacementMap,this.displacementScale=t.displacementScale,this.displacementBias=t.displacementBias,this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this}}class Tf extends Jn{constructor(t){super(),this.isMeshDistanceMaterial=!0,this.type="MeshDistanceMaterial",this.map=null,this.alphaMap=null,this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.setValues(t)}copy(t){return super.copy(t),this.map=t.map,this.alphaMap=t.alphaMap,this.displacementMap=t.displacementMap,this.displacementScale=t.displacementScale,this.displacementBias=t.displacementBias,this}}class wf extends de{constructor(t,e=1){super(),this.isLight=!0,this.type="Light",this.color=new Wt(t),this.intensity=e}dispose(){}copy(t,e){return super.copy(t,e),this.color.copy(t.color),this.intensity=t.intensity,this}toJSON(t){const e=super.toJSON(t);return e.object.color=this.color.getHex(),e.object.intensity=this.intensity,this.groundColor!==void 0&&(e.object.groundColor=this.groundColor.getHex()),this.distance!==void 0&&(e.object.distance=this.distance),this.angle!==void 0&&(e.object.angle=this.angle),this.decay!==void 0&&(e.object.decay=this.decay),this.penumbra!==void 0&&(e.object.penumbra=this.penumbra),this.shadow!==void 0&&(e.object.shadow=this.shadow.toJSON()),this.target!==void 0&&(e.object.target=this.target.uuid),e}}const As=new te,Io=new H,Uo=new H;class Af{constructor(t){this.camera=t,this.intensity=1,this.bias=0,this.normalBias=0,this.radius=1,this.blurSamples=8,this.mapSize=new Xt(512,512),this.mapType=Je,this.map=null,this.mapPass=null,this.matrix=new te,this.autoUpdate=!0,this.needsUpdate=!1,this._frustum=new ka,this._frameExtents=new Xt(1,1),this._viewportCount=1,this._viewports=[new se(0,0,1,1)]}getViewportCount(){return this._viewportCount}getFrustum(){return this._frustum}updateMatrices(t){const e=this.camera,i=this.matrix;Io.setFromMatrixPosition(t.matrixWorld),e.position.copy(Io),Uo.setFromMatrixPosition(t.target.matrixWorld),e.lookAt(Uo),e.updateMatrixWorld(),As.multiplyMatrices(e.projectionMatrix,e.matrixWorldInverse),this._frustum.setFromProjectionMatrix(As),i.set(.5,0,0,.5,0,.5,0,.5,0,0,.5,.5,0,0,0,1),i.multiply(As)}getViewport(t){return this._viewports[t]}getFrameExtents(){return this._frameExtents}dispose(){this.map&&this.map.dispose(),this.mapPass&&this.mapPass.dispose()}copy(t){return this.camera=t.camera.clone(),this.intensity=t.intensity,this.bias=t.bias,this.radius=t.radius,this.autoUpdate=t.autoUpdate,this.needsUpdate=t.needsUpdate,this.normalBias=t.normalBias,this.blurSamples=t.blurSamples,this.mapSize.copy(t.mapSize),this}clone(){return new this.constructor().copy(this)}toJSON(){const t={};return this.intensity!==1&&(t.intensity=this.intensity),this.bias!==0&&(t.bias=this.bias),this.normalBias!==0&&(t.normalBias=this.normalBias),this.radius!==1&&(t.radius=this.radius),(this.mapSize.x!==512||this.mapSize.y!==512)&&(t.mapSize=this.mapSize.toArray()),t.camera=this.camera.toJSON(!1).object,delete t.camera.matrix,t}}class rl extends tl{constructor(t=-1,e=1,i=1,r=-1,s=.1,o=2e3){super(),this.isOrthographicCamera=!0,this.type="OrthographicCamera",this.zoom=1,this.view=null,this.left=t,this.right=e,this.top=i,this.bottom=r,this.near=s,this.far=o,this.updateProjectionMatrix()}copy(t,e){return super.copy(t,e),this.left=t.left,this.right=t.right,this.top=t.top,this.bottom=t.bottom,this.near=t.near,this.far=t.far,this.zoom=t.zoom,this.view=t.view===null?null:Object.assign({},t.view),this}setViewOffset(t,e,i,r,s,o){this.view===null&&(this.view={enabled:!0,fullWidth:1,fullHeight:1,offsetX:0,offsetY:0,width:1,height:1}),this.view.enabled=!0,this.view.fullWidth=t,this.view.fullHeight=e,this.view.offsetX=i,this.view.offsetY=r,this.view.width=s,this.view.height=o,this.updateProjectionMatrix()}clearViewOffset(){this.view!==null&&(this.view.enabled=!1),this.updateProjectionMatrix()}updateProjectionMatrix(){const t=(this.right-this.left)/(2*this.zoom),e=(this.top-this.bottom)/(2*this.zoom),i=(this.right+this.left)/2,r=(this.top+this.bottom)/2;let s=i-t,o=i+t,a=r+e,l=r-e;if(this.view!==null&&this.view.enabled){const u=(this.right-this.left)/this.view.fullWidth/this.zoom,h=(this.top-this.bottom)/this.view.fullHeight/this.zoom;s+=u*this.view.offsetX,o=s+u*this.view.width,a-=h*this.view.offsetY,l=a-h*this.view.height}this.projectionMatrix.makeOrthographic(s,o,a,l,this.near,this.far,this.coordinateSystem),this.projectionMatrixInverse.copy(this.projectionMatrix).invert()}toJSON(t){const e=super.toJSON(t);return e.object.zoom=this.zoom,e.object.left=this.left,e.object.right=this.right,e.object.top=this.top,e.object.bottom=this.bottom,e.object.near=this.near,e.object.far=this.far,this.view!==null&&(e.object.view=Object.assign({},this.view)),e}}class bf extends Af{constructor(){super(new rl(-5,5,5,-5,.5,500)),this.isDirectionalLightShadow=!0}}class Cf extends wf{constructor(t,e){super(t,e),this.isDirectionalLight=!0,this.type="DirectionalLight",this.position.copy(de.DEFAULT_UP),this.updateMatrix(),this.target=new de,this.shadow=new bf}dispose(){this.shadow.dispose()}copy(t){return super.copy(t),this.target=t.target.clone(),this.shadow=t.shadow.clone(),this}}class Rf extends Ne{constructor(t=[]){super(),this.isArrayCamera=!0,this.isMultiViewCamera=!1,this.cameras=t}}function Fo(n,t,e,i){const r=Pf(i);switch(e){case kc:return n*t;case Vc:return n*t/r.components*r.byteLength;case Fa:return n*t/r.components*r.byteLength;case Gc:return n*t*2/r.components*r.byteLength;case Na:return n*t*2/r.components*r.byteLength;case Hc:return n*t*3/r.components*r.byteLength;case We:return n*t*4/r.components*r.byteLength;case Oa:return n*t*4/r.components*r.byteLength;case Cr:case Rr:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*8;case Pr:case Lr:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*16;case Qs:case ea:return Math.max(n,16)*Math.max(t,8)/4;case Js:case ta:return Math.max(n,8)*Math.max(t,8)/2;case na:case ia:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*8;case ra:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*16;case sa:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*16;case aa:return Math.floor((n+4)/5)*Math.floor((t+3)/4)*16;case oa:return Math.floor((n+4)/5)*Math.floor((t+4)/5)*16;case ca:return Math.floor((n+5)/6)*Math.floor((t+4)/5)*16;case la:return Math.floor((n+5)/6)*Math.floor((t+5)/6)*16;case ua:return Math.floor((n+7)/8)*Math.floor((t+4)/5)*16;case fa:return Math.floor((n+7)/8)*Math.floor((t+5)/6)*16;case ha:return Math.floor((n+7)/8)*Math.floor((t+7)/8)*16;case da:return Math.floor((n+9)/10)*Math.floor((t+4)/5)*16;case pa:return Math.floor((n+9)/10)*Math.floor((t+5)/6)*16;case ma:return Math.floor((n+9)/10)*Math.floor((t+7)/8)*16;case _a:return Math.floor((n+9)/10)*Math.floor((t+9)/10)*16;case ga:return Math.floor((n+11)/12)*Math.floor((t+9)/10)*16;case xa:return Math.floor((n+11)/12)*Math.floor((t+11)/12)*16;case Dr:case va:case ya:return Math.ceil(n/4)*Math.ceil(t/4)*16;case Wc:case Sa:return Math.ceil(n/4)*Math.ceil(t/4)*8;case Ma:case Ea:return Math.ceil(n/4)*Math.ceil(t/4)*16}throw new Error(`Unable to determine texture byte length for ${e} format.`)}function Pf(n){switch(n){case Je:case Oc:return{byteLength:1,components:1};case ki:case Bc:case qi:return{byteLength:2,components:1};case Ia:case Ua:return{byteLength:2,components:4};case Yn:case Da:case cn:return{byteLength:4,components:1};case zc:return{byteLength:4,components:3}}throw new Error(`Unknown texture type ${n}.`)}typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("register",{detail:{revision:La}}));typeof window<"u"&&(window.__THREE__?console.warn("WARNING: Multiple instances of Three.js being imported."):window.__THREE__=La);/**
 * @license
 * Copyright 2010-2025 Three.js Authors
 * SPDX-License-Identifier: MIT
 */function sl(){let n=null,t=!1,e=null,i=null;function r(s,o){e(s,o),i=n.requestAnimationFrame(r)}return{start:function(){t!==!0&&e!==null&&(i=n.requestAnimationFrame(r),t=!0)},stop:function(){n.cancelAnimationFrame(i),t=!1},setAnimationLoop:function(s){e=s},setContext:function(s){n=s}}}function Lf(n){const t=new WeakMap;function e(a,l){const u=a.array,h=a.usage,p=u.byteLength,m=n.createBuffer();n.bindBuffer(l,m),n.bufferData(l,u,h),a.onUploadCallback();let _;if(u instanceof Float32Array)_=n.FLOAT;else if(u instanceof Uint16Array)a.isFloat16BufferAttribute?_=n.HALF_FLOAT:_=n.UNSIGNED_SHORT;else if(u instanceof Int16Array)_=n.SHORT;else if(u instanceof Uint32Array)_=n.UNSIGNED_INT;else if(u instanceof Int32Array)_=n.INT;else if(u instanceof Int8Array)_=n.BYTE;else if(u instanceof Uint8Array)_=n.UNSIGNED_BYTE;else if(u instanceof Uint8ClampedArray)_=n.UNSIGNED_BYTE;else throw new Error("THREE.WebGLAttributes: Unsupported buffer data format: "+u);return{buffer:m,type:_,bytesPerElement:u.BYTES_PER_ELEMENT,version:a.version,size:p}}function i(a,l,u){const h=l.array,p=l.updateRanges;if(n.bindBuffer(u,a),p.length===0)n.bufferSubData(u,0,h);else{p.sort((_,x)=>_.start-x.start);let m=0;for(let _=1;_<p.length;_++){const x=p[m],y=p[_];y.start<=x.start+x.count+1?x.count=Math.max(x.count,y.start+y.count-x.start):(++m,p[m]=y)}p.length=m+1;for(let _=0,x=p.length;_<x;_++){const y=p[_];n.bufferSubData(u,y.start*h.BYTES_PER_ELEMENT,h,y.start,y.count)}l.clearUpdateRanges()}l.onUploadCallback()}function r(a){return a.isInterleavedBufferAttribute&&(a=a.data),t.get(a)}function s(a){a.isInterleavedBufferAttribute&&(a=a.data);const l=t.get(a);l&&(n.deleteBuffer(l.buffer),t.delete(a))}function o(a,l){if(a.isInterleavedBufferAttribute&&(a=a.data),a.isGLBufferAttribute){const h=t.get(a);(!h||h.version<a.version)&&t.set(a,{buffer:a.buffer,type:a.type,bytesPerElement:a.elementSize,version:a.version});return}const u=t.get(a);if(u===void 0)t.set(a,e(a,l));else if(u.version<a.version){if(u.size!==a.array.byteLength)throw new Error("THREE.WebGLAttributes: The size of the buffer attribute's array buffer does not match the original size. Resizing buffer attributes is not supported.");i(u.buffer,a,l),u.version=a.version}}return{get:r,remove:s,update:o}}var Df=`#ifdef USE_ALPHAHASH
	if ( diffuseColor.a < getAlphaHashThreshold( vPosition ) ) discard;
#endif`,If=`#ifdef USE_ALPHAHASH
	const float ALPHA_HASH_SCALE = 0.05;
	float hash2D( vec2 value ) {
		return fract( 1.0e4 * sin( 17.0 * value.x + 0.1 * value.y ) * ( 0.1 + abs( sin( 13.0 * value.y + value.x ) ) ) );
	}
	float hash3D( vec3 value ) {
		return hash2D( vec2( hash2D( value.xy ), value.z ) );
	}
	float getAlphaHashThreshold( vec3 position ) {
		float maxDeriv = max(
			length( dFdx( position.xyz ) ),
			length( dFdy( position.xyz ) )
		);
		float pixScale = 1.0 / ( ALPHA_HASH_SCALE * maxDeriv );
		vec2 pixScales = vec2(
			exp2( floor( log2( pixScale ) ) ),
			exp2( ceil( log2( pixScale ) ) )
		);
		vec2 alpha = vec2(
			hash3D( floor( pixScales.x * position.xyz ) ),
			hash3D( floor( pixScales.y * position.xyz ) )
		);
		float lerpFactor = fract( log2( pixScale ) );
		float x = ( 1.0 - lerpFactor ) * alpha.x + lerpFactor * alpha.y;
		float a = min( lerpFactor, 1.0 - lerpFactor );
		vec3 cases = vec3(
			x * x / ( 2.0 * a * ( 1.0 - a ) ),
			( x - 0.5 * a ) / ( 1.0 - a ),
			1.0 - ( ( 1.0 - x ) * ( 1.0 - x ) / ( 2.0 * a * ( 1.0 - a ) ) )
		);
		float threshold = ( x < ( 1.0 - a ) )
			? ( ( x < a ) ? cases.x : cases.y )
			: cases.z;
		return clamp( threshold , 1.0e-6, 1.0 );
	}
#endif`,Uf=`#ifdef USE_ALPHAMAP
	diffuseColor.a *= texture2D( alphaMap, vAlphaMapUv ).g;
#endif`,Ff=`#ifdef USE_ALPHAMAP
	uniform sampler2D alphaMap;
#endif`,Nf=`#ifdef USE_ALPHATEST
	#ifdef ALPHA_TO_COVERAGE
	diffuseColor.a = smoothstep( alphaTest, alphaTest + fwidth( diffuseColor.a ), diffuseColor.a );
	if ( diffuseColor.a == 0.0 ) discard;
	#else
	if ( diffuseColor.a < alphaTest ) discard;
	#endif
#endif`,Of=`#ifdef USE_ALPHATEST
	uniform float alphaTest;
#endif`,Bf=`#ifdef USE_AOMAP
	float ambientOcclusion = ( texture2D( aoMap, vAoMapUv ).r - 1.0 ) * aoMapIntensity + 1.0;
	reflectedLight.indirectDiffuse *= ambientOcclusion;
	#if defined( USE_CLEARCOAT ) 
		clearcoatSpecularIndirect *= ambientOcclusion;
	#endif
	#if defined( USE_SHEEN ) 
		sheenSpecularIndirect *= ambientOcclusion;
	#endif
	#if defined( USE_ENVMAP ) && defined( STANDARD )
		float dotNV = saturate( dot( geometryNormal, geometryViewDir ) );
		reflectedLight.indirectSpecular *= computeSpecularOcclusion( dotNV, ambientOcclusion, material.roughness );
	#endif
#endif`,zf=`#ifdef USE_AOMAP
	uniform sampler2D aoMap;
	uniform float aoMapIntensity;
#endif`,kf=`#ifdef USE_BATCHING
	#if ! defined( GL_ANGLE_multi_draw )
	#define gl_DrawID _gl_DrawID
	uniform int _gl_DrawID;
	#endif
	uniform highp sampler2D batchingTexture;
	uniform highp usampler2D batchingIdTexture;
	mat4 getBatchingMatrix( const in float i ) {
		int size = textureSize( batchingTexture, 0 ).x;
		int j = int( i ) * 4;
		int x = j % size;
		int y = j / size;
		vec4 v1 = texelFetch( batchingTexture, ivec2( x, y ), 0 );
		vec4 v2 = texelFetch( batchingTexture, ivec2( x + 1, y ), 0 );
		vec4 v3 = texelFetch( batchingTexture, ivec2( x + 2, y ), 0 );
		vec4 v4 = texelFetch( batchingTexture, ivec2( x + 3, y ), 0 );
		return mat4( v1, v2, v3, v4 );
	}
	float getIndirectIndex( const in int i ) {
		int size = textureSize( batchingIdTexture, 0 ).x;
		int x = i % size;
		int y = i / size;
		return float( texelFetch( batchingIdTexture, ivec2( x, y ), 0 ).r );
	}
#endif
#ifdef USE_BATCHING_COLOR
	uniform sampler2D batchingColorTexture;
	vec3 getBatchingColor( const in float i ) {
		int size = textureSize( batchingColorTexture, 0 ).x;
		int j = int( i );
		int x = j % size;
		int y = j / size;
		return texelFetch( batchingColorTexture, ivec2( x, y ), 0 ).rgb;
	}
#endif`,Hf=`#ifdef USE_BATCHING
	mat4 batchingMatrix = getBatchingMatrix( getIndirectIndex( gl_DrawID ) );
#endif`,Vf=`vec3 transformed = vec3( position );
#ifdef USE_ALPHAHASH
	vPosition = vec3( position );
#endif`,Gf=`vec3 objectNormal = vec3( normal );
#ifdef USE_TANGENT
	vec3 objectTangent = vec3( tangent.xyz );
#endif`,Wf=`float G_BlinnPhong_Implicit( ) {
	return 0.25;
}
float D_BlinnPhong( const in float shininess, const in float dotNH ) {
	return RECIPROCAL_PI * ( shininess * 0.5 + 1.0 ) * pow( dotNH, shininess );
}
vec3 BRDF_BlinnPhong( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, const in vec3 specularColor, const in float shininess ) {
	vec3 halfDir = normalize( lightDir + viewDir );
	float dotNH = saturate( dot( normal, halfDir ) );
	float dotVH = saturate( dot( viewDir, halfDir ) );
	vec3 F = F_Schlick( specularColor, 1.0, dotVH );
	float G = G_BlinnPhong_Implicit( );
	float D = D_BlinnPhong( shininess, dotNH );
	return F * ( G * D );
} // validated`,Xf=`#ifdef USE_IRIDESCENCE
	const mat3 XYZ_TO_REC709 = mat3(
		 3.2404542, -0.9692660,  0.0556434,
		-1.5371385,  1.8760108, -0.2040259,
		-0.4985314,  0.0415560,  1.0572252
	);
	vec3 Fresnel0ToIor( vec3 fresnel0 ) {
		vec3 sqrtF0 = sqrt( fresnel0 );
		return ( vec3( 1.0 ) + sqrtF0 ) / ( vec3( 1.0 ) - sqrtF0 );
	}
	vec3 IorToFresnel0( vec3 transmittedIor, float incidentIor ) {
		return pow2( ( transmittedIor - vec3( incidentIor ) ) / ( transmittedIor + vec3( incidentIor ) ) );
	}
	float IorToFresnel0( float transmittedIor, float incidentIor ) {
		return pow2( ( transmittedIor - incidentIor ) / ( transmittedIor + incidentIor ));
	}
	vec3 evalSensitivity( float OPD, vec3 shift ) {
		float phase = 2.0 * PI * OPD * 1.0e-9;
		vec3 val = vec3( 5.4856e-13, 4.4201e-13, 5.2481e-13 );
		vec3 pos = vec3( 1.6810e+06, 1.7953e+06, 2.2084e+06 );
		vec3 var = vec3( 4.3278e+09, 9.3046e+09, 6.6121e+09 );
		vec3 xyz = val * sqrt( 2.0 * PI * var ) * cos( pos * phase + shift ) * exp( - pow2( phase ) * var );
		xyz.x += 9.7470e-14 * sqrt( 2.0 * PI * 4.5282e+09 ) * cos( 2.2399e+06 * phase + shift[ 0 ] ) * exp( - 4.5282e+09 * pow2( phase ) );
		xyz /= 1.0685e-7;
		vec3 rgb = XYZ_TO_REC709 * xyz;
		return rgb;
	}
	vec3 evalIridescence( float outsideIOR, float eta2, float cosTheta1, float thinFilmThickness, vec3 baseF0 ) {
		vec3 I;
		float iridescenceIOR = mix( outsideIOR, eta2, smoothstep( 0.0, 0.03, thinFilmThickness ) );
		float sinTheta2Sq = pow2( outsideIOR / iridescenceIOR ) * ( 1.0 - pow2( cosTheta1 ) );
		float cosTheta2Sq = 1.0 - sinTheta2Sq;
		if ( cosTheta2Sq < 0.0 ) {
			return vec3( 1.0 );
		}
		float cosTheta2 = sqrt( cosTheta2Sq );
		float R0 = IorToFresnel0( iridescenceIOR, outsideIOR );
		float R12 = F_Schlick( R0, 1.0, cosTheta1 );
		float T121 = 1.0 - R12;
		float phi12 = 0.0;
		if ( iridescenceIOR < outsideIOR ) phi12 = PI;
		float phi21 = PI - phi12;
		vec3 baseIOR = Fresnel0ToIor( clamp( baseF0, 0.0, 0.9999 ) );		vec3 R1 = IorToFresnel0( baseIOR, iridescenceIOR );
		vec3 R23 = F_Schlick( R1, 1.0, cosTheta2 );
		vec3 phi23 = vec3( 0.0 );
		if ( baseIOR[ 0 ] < iridescenceIOR ) phi23[ 0 ] = PI;
		if ( baseIOR[ 1 ] < iridescenceIOR ) phi23[ 1 ] = PI;
		if ( baseIOR[ 2 ] < iridescenceIOR ) phi23[ 2 ] = PI;
		float OPD = 2.0 * iridescenceIOR * thinFilmThickness * cosTheta2;
		vec3 phi = vec3( phi21 ) + phi23;
		vec3 R123 = clamp( R12 * R23, 1e-5, 0.9999 );
		vec3 r123 = sqrt( R123 );
		vec3 Rs = pow2( T121 ) * R23 / ( vec3( 1.0 ) - R123 );
		vec3 C0 = R12 + Rs;
		I = C0;
		vec3 Cm = Rs - T121;
		for ( int m = 1; m <= 2; ++ m ) {
			Cm *= r123;
			vec3 Sm = 2.0 * evalSensitivity( float( m ) * OPD, float( m ) * phi );
			I += Cm * Sm;
		}
		return max( I, vec3( 0.0 ) );
	}
#endif`,qf=`#ifdef USE_BUMPMAP
	uniform sampler2D bumpMap;
	uniform float bumpScale;
	vec2 dHdxy_fwd() {
		vec2 dSTdx = dFdx( vBumpMapUv );
		vec2 dSTdy = dFdy( vBumpMapUv );
		float Hll = bumpScale * texture2D( bumpMap, vBumpMapUv ).x;
		float dBx = bumpScale * texture2D( bumpMap, vBumpMapUv + dSTdx ).x - Hll;
		float dBy = bumpScale * texture2D( bumpMap, vBumpMapUv + dSTdy ).x - Hll;
		return vec2( dBx, dBy );
	}
	vec3 perturbNormalArb( vec3 surf_pos, vec3 surf_norm, vec2 dHdxy, float faceDirection ) {
		vec3 vSigmaX = normalize( dFdx( surf_pos.xyz ) );
		vec3 vSigmaY = normalize( dFdy( surf_pos.xyz ) );
		vec3 vN = surf_norm;
		vec3 R1 = cross( vSigmaY, vN );
		vec3 R2 = cross( vN, vSigmaX );
		float fDet = dot( vSigmaX, R1 ) * faceDirection;
		vec3 vGrad = sign( fDet ) * ( dHdxy.x * R1 + dHdxy.y * R2 );
		return normalize( abs( fDet ) * surf_norm - vGrad );
	}
#endif`,$f=`#if NUM_CLIPPING_PLANES > 0
	vec4 plane;
	#ifdef ALPHA_TO_COVERAGE
		float distanceToPlane, distanceGradient;
		float clipOpacity = 1.0;
		#pragma unroll_loop_start
		for ( int i = 0; i < UNION_CLIPPING_PLANES; i ++ ) {
			plane = clippingPlanes[ i ];
			distanceToPlane = - dot( vClipPosition, plane.xyz ) + plane.w;
			distanceGradient = fwidth( distanceToPlane ) / 2.0;
			clipOpacity *= smoothstep( - distanceGradient, distanceGradient, distanceToPlane );
			if ( clipOpacity == 0.0 ) discard;
		}
		#pragma unroll_loop_end
		#if UNION_CLIPPING_PLANES < NUM_CLIPPING_PLANES
			float unionClipOpacity = 1.0;
			#pragma unroll_loop_start
			for ( int i = UNION_CLIPPING_PLANES; i < NUM_CLIPPING_PLANES; i ++ ) {
				plane = clippingPlanes[ i ];
				distanceToPlane = - dot( vClipPosition, plane.xyz ) + plane.w;
				distanceGradient = fwidth( distanceToPlane ) / 2.0;
				unionClipOpacity *= 1.0 - smoothstep( - distanceGradient, distanceGradient, distanceToPlane );
			}
			#pragma unroll_loop_end
			clipOpacity *= 1.0 - unionClipOpacity;
		#endif
		diffuseColor.a *= clipOpacity;
		if ( diffuseColor.a == 0.0 ) discard;
	#else
		#pragma unroll_loop_start
		for ( int i = 0; i < UNION_CLIPPING_PLANES; i ++ ) {
			plane = clippingPlanes[ i ];
			if ( dot( vClipPosition, plane.xyz ) > plane.w ) discard;
		}
		#pragma unroll_loop_end
		#if UNION_CLIPPING_PLANES < NUM_CLIPPING_PLANES
			bool clipped = true;
			#pragma unroll_loop_start
			for ( int i = UNION_CLIPPING_PLANES; i < NUM_CLIPPING_PLANES; i ++ ) {
				plane = clippingPlanes[ i ];
				clipped = ( dot( vClipPosition, plane.xyz ) > plane.w ) && clipped;
			}
			#pragma unroll_loop_end
			if ( clipped ) discard;
		#endif
	#endif
#endif`,Yf=`#if NUM_CLIPPING_PLANES > 0
	varying vec3 vClipPosition;
	uniform vec4 clippingPlanes[ NUM_CLIPPING_PLANES ];
#endif`,jf=`#if NUM_CLIPPING_PLANES > 0
	varying vec3 vClipPosition;
#endif`,Kf=`#if NUM_CLIPPING_PLANES > 0
	vClipPosition = - mvPosition.xyz;
#endif`,Zf=`#if defined( USE_COLOR_ALPHA )
	diffuseColor *= vColor;
#elif defined( USE_COLOR )
	diffuseColor.rgb *= vColor;
#endif`,Jf=`#if defined( USE_COLOR_ALPHA )
	varying vec4 vColor;
#elif defined( USE_COLOR )
	varying vec3 vColor;
#endif`,Qf=`#if defined( USE_COLOR_ALPHA )
	varying vec4 vColor;
#elif defined( USE_COLOR ) || defined( USE_INSTANCING_COLOR ) || defined( USE_BATCHING_COLOR )
	varying vec3 vColor;
#endif`,th=`#if defined( USE_COLOR_ALPHA )
	vColor = vec4( 1.0 );
#elif defined( USE_COLOR ) || defined( USE_INSTANCING_COLOR ) || defined( USE_BATCHING_COLOR )
	vColor = vec3( 1.0 );
#endif
#ifdef USE_COLOR
	vColor *= color;
#endif
#ifdef USE_INSTANCING_COLOR
	vColor.xyz *= instanceColor.xyz;
#endif
#ifdef USE_BATCHING_COLOR
	vec3 batchingColor = getBatchingColor( getIndirectIndex( gl_DrawID ) );
	vColor.xyz *= batchingColor.xyz;
#endif`,eh=`#define PI 3.141592653589793
#define PI2 6.283185307179586
#define PI_HALF 1.5707963267948966
#define RECIPROCAL_PI 0.3183098861837907
#define RECIPROCAL_PI2 0.15915494309189535
#define EPSILON 1e-6
#ifndef saturate
#define saturate( a ) clamp( a, 0.0, 1.0 )
#endif
#define whiteComplement( a ) ( 1.0 - saturate( a ) )
float pow2( const in float x ) { return x*x; }
vec3 pow2( const in vec3 x ) { return x*x; }
float pow3( const in float x ) { return x*x*x; }
float pow4( const in float x ) { float x2 = x*x; return x2*x2; }
float max3( const in vec3 v ) { return max( max( v.x, v.y ), v.z ); }
float average( const in vec3 v ) { return dot( v, vec3( 0.3333333 ) ); }
highp float rand( const in vec2 uv ) {
	const highp float a = 12.9898, b = 78.233, c = 43758.5453;
	highp float dt = dot( uv.xy, vec2( a,b ) ), sn = mod( dt, PI );
	return fract( sin( sn ) * c );
}
#ifdef HIGH_PRECISION
	float precisionSafeLength( vec3 v ) { return length( v ); }
#else
	float precisionSafeLength( vec3 v ) {
		float maxComponent = max3( abs( v ) );
		return length( v / maxComponent ) * maxComponent;
	}
#endif
struct IncidentLight {
	vec3 color;
	vec3 direction;
	bool visible;
};
struct ReflectedLight {
	vec3 directDiffuse;
	vec3 directSpecular;
	vec3 indirectDiffuse;
	vec3 indirectSpecular;
};
#ifdef USE_ALPHAHASH
	varying vec3 vPosition;
#endif
vec3 transformDirection( in vec3 dir, in mat4 matrix ) {
	return normalize( ( matrix * vec4( dir, 0.0 ) ).xyz );
}
vec3 inverseTransformDirection( in vec3 dir, in mat4 matrix ) {
	return normalize( ( vec4( dir, 0.0 ) * matrix ).xyz );
}
mat3 transposeMat3( const in mat3 m ) {
	mat3 tmp;
	tmp[ 0 ] = vec3( m[ 0 ].x, m[ 1 ].x, m[ 2 ].x );
	tmp[ 1 ] = vec3( m[ 0 ].y, m[ 1 ].y, m[ 2 ].y );
	tmp[ 2 ] = vec3( m[ 0 ].z, m[ 1 ].z, m[ 2 ].z );
	return tmp;
}
bool isPerspectiveMatrix( mat4 m ) {
	return m[ 2 ][ 3 ] == - 1.0;
}
vec2 equirectUv( in vec3 dir ) {
	float u = atan( dir.z, dir.x ) * RECIPROCAL_PI2 + 0.5;
	float v = asin( clamp( dir.y, - 1.0, 1.0 ) ) * RECIPROCAL_PI + 0.5;
	return vec2( u, v );
}
vec3 BRDF_Lambert( const in vec3 diffuseColor ) {
	return RECIPROCAL_PI * diffuseColor;
}
vec3 F_Schlick( const in vec3 f0, const in float f90, const in float dotVH ) {
	float fresnel = exp2( ( - 5.55473 * dotVH - 6.98316 ) * dotVH );
	return f0 * ( 1.0 - fresnel ) + ( f90 * fresnel );
}
float F_Schlick( const in float f0, const in float f90, const in float dotVH ) {
	float fresnel = exp2( ( - 5.55473 * dotVH - 6.98316 ) * dotVH );
	return f0 * ( 1.0 - fresnel ) + ( f90 * fresnel );
} // validated`,nh=`#ifdef ENVMAP_TYPE_CUBE_UV
	#define cubeUV_minMipLevel 4.0
	#define cubeUV_minTileSize 16.0
	float getFace( vec3 direction ) {
		vec3 absDirection = abs( direction );
		float face = - 1.0;
		if ( absDirection.x > absDirection.z ) {
			if ( absDirection.x > absDirection.y )
				face = direction.x > 0.0 ? 0.0 : 3.0;
			else
				face = direction.y > 0.0 ? 1.0 : 4.0;
		} else {
			if ( absDirection.z > absDirection.y )
				face = direction.z > 0.0 ? 2.0 : 5.0;
			else
				face = direction.y > 0.0 ? 1.0 : 4.0;
		}
		return face;
	}
	vec2 getUV( vec3 direction, float face ) {
		vec2 uv;
		if ( face == 0.0 ) {
			uv = vec2( direction.z, direction.y ) / abs( direction.x );
		} else if ( face == 1.0 ) {
			uv = vec2( - direction.x, - direction.z ) / abs( direction.y );
		} else if ( face == 2.0 ) {
			uv = vec2( - direction.x, direction.y ) / abs( direction.z );
		} else if ( face == 3.0 ) {
			uv = vec2( - direction.z, direction.y ) / abs( direction.x );
		} else if ( face == 4.0 ) {
			uv = vec2( - direction.x, direction.z ) / abs( direction.y );
		} else {
			uv = vec2( direction.x, direction.y ) / abs( direction.z );
		}
		return 0.5 * ( uv + 1.0 );
	}
	vec3 bilinearCubeUV( sampler2D envMap, vec3 direction, float mipInt ) {
		float face = getFace( direction );
		float filterInt = max( cubeUV_minMipLevel - mipInt, 0.0 );
		mipInt = max( mipInt, cubeUV_minMipLevel );
		float faceSize = exp2( mipInt );
		highp vec2 uv = getUV( direction, face ) * ( faceSize - 2.0 ) + 1.0;
		if ( face > 2.0 ) {
			uv.y += faceSize;
			face -= 3.0;
		}
		uv.x += face * faceSize;
		uv.x += filterInt * 3.0 * cubeUV_minTileSize;
		uv.y += 4.0 * ( exp2( CUBEUV_MAX_MIP ) - faceSize );
		uv.x *= CUBEUV_TEXEL_WIDTH;
		uv.y *= CUBEUV_TEXEL_HEIGHT;
		#ifdef texture2DGradEXT
			return texture2DGradEXT( envMap, uv, vec2( 0.0 ), vec2( 0.0 ) ).rgb;
		#else
			return texture2D( envMap, uv ).rgb;
		#endif
	}
	#define cubeUV_r0 1.0
	#define cubeUV_m0 - 2.0
	#define cubeUV_r1 0.8
	#define cubeUV_m1 - 1.0
	#define cubeUV_r4 0.4
	#define cubeUV_m4 2.0
	#define cubeUV_r5 0.305
	#define cubeUV_m5 3.0
	#define cubeUV_r6 0.21
	#define cubeUV_m6 4.0
	float roughnessToMip( float roughness ) {
		float mip = 0.0;
		if ( roughness >= cubeUV_r1 ) {
			mip = ( cubeUV_r0 - roughness ) * ( cubeUV_m1 - cubeUV_m0 ) / ( cubeUV_r0 - cubeUV_r1 ) + cubeUV_m0;
		} else if ( roughness >= cubeUV_r4 ) {
			mip = ( cubeUV_r1 - roughness ) * ( cubeUV_m4 - cubeUV_m1 ) / ( cubeUV_r1 - cubeUV_r4 ) + cubeUV_m1;
		} else if ( roughness >= cubeUV_r5 ) {
			mip = ( cubeUV_r4 - roughness ) * ( cubeUV_m5 - cubeUV_m4 ) / ( cubeUV_r4 - cubeUV_r5 ) + cubeUV_m4;
		} else if ( roughness >= cubeUV_r6 ) {
			mip = ( cubeUV_r5 - roughness ) * ( cubeUV_m6 - cubeUV_m5 ) / ( cubeUV_r5 - cubeUV_r6 ) + cubeUV_m5;
		} else {
			mip = - 2.0 * log2( 1.16 * roughness );		}
		return mip;
	}
	vec4 textureCubeUV( sampler2D envMap, vec3 sampleDir, float roughness ) {
		float mip = clamp( roughnessToMip( roughness ), cubeUV_m0, CUBEUV_MAX_MIP );
		float mipF = fract( mip );
		float mipInt = floor( mip );
		vec3 color0 = bilinearCubeUV( envMap, sampleDir, mipInt );
		if ( mipF == 0.0 ) {
			return vec4( color0, 1.0 );
		} else {
			vec3 color1 = bilinearCubeUV( envMap, sampleDir, mipInt + 1.0 );
			return vec4( mix( color0, color1, mipF ), 1.0 );
		}
	}
#endif`,ih=`vec3 transformedNormal = objectNormal;
#ifdef USE_TANGENT
	vec3 transformedTangent = objectTangent;
#endif
#ifdef USE_BATCHING
	mat3 bm = mat3( batchingMatrix );
	transformedNormal /= vec3( dot( bm[ 0 ], bm[ 0 ] ), dot( bm[ 1 ], bm[ 1 ] ), dot( bm[ 2 ], bm[ 2 ] ) );
	transformedNormal = bm * transformedNormal;
	#ifdef USE_TANGENT
		transformedTangent = bm * transformedTangent;
	#endif
#endif
#ifdef USE_INSTANCING
	mat3 im = mat3( instanceMatrix );
	transformedNormal /= vec3( dot( im[ 0 ], im[ 0 ] ), dot( im[ 1 ], im[ 1 ] ), dot( im[ 2 ], im[ 2 ] ) );
	transformedNormal = im * transformedNormal;
	#ifdef USE_TANGENT
		transformedTangent = im * transformedTangent;
	#endif
#endif
transformedNormal = normalMatrix * transformedNormal;
#ifdef FLIP_SIDED
	transformedNormal = - transformedNormal;
#endif
#ifdef USE_TANGENT
	transformedTangent = ( modelViewMatrix * vec4( transformedTangent, 0.0 ) ).xyz;
	#ifdef FLIP_SIDED
		transformedTangent = - transformedTangent;
	#endif
#endif`,rh=`#ifdef USE_DISPLACEMENTMAP
	uniform sampler2D displacementMap;
	uniform float displacementScale;
	uniform float displacementBias;
#endif`,sh=`#ifdef USE_DISPLACEMENTMAP
	transformed += normalize( objectNormal ) * ( texture2D( displacementMap, vDisplacementMapUv ).x * displacementScale + displacementBias );
#endif`,ah=`#ifdef USE_EMISSIVEMAP
	vec4 emissiveColor = texture2D( emissiveMap, vEmissiveMapUv );
	#ifdef DECODE_VIDEO_TEXTURE_EMISSIVE
		emissiveColor = sRGBTransferEOTF( emissiveColor );
	#endif
	totalEmissiveRadiance *= emissiveColor.rgb;
#endif`,oh=`#ifdef USE_EMISSIVEMAP
	uniform sampler2D emissiveMap;
#endif`,ch="gl_FragColor = linearToOutputTexel( gl_FragColor );",lh=`vec4 LinearTransferOETF( in vec4 value ) {
	return value;
}
vec4 sRGBTransferEOTF( in vec4 value ) {
	return vec4( mix( pow( value.rgb * 0.9478672986 + vec3( 0.0521327014 ), vec3( 2.4 ) ), value.rgb * 0.0773993808, vec3( lessThanEqual( value.rgb, vec3( 0.04045 ) ) ) ), value.a );
}
vec4 sRGBTransferOETF( in vec4 value ) {
	return vec4( mix( pow( value.rgb, vec3( 0.41666 ) ) * 1.055 - vec3( 0.055 ), value.rgb * 12.92, vec3( lessThanEqual( value.rgb, vec3( 0.0031308 ) ) ) ), value.a );
}`,uh=`#ifdef USE_ENVMAP
	#ifdef ENV_WORLDPOS
		vec3 cameraToFrag;
		if ( isOrthographic ) {
			cameraToFrag = normalize( vec3( - viewMatrix[ 0 ][ 2 ], - viewMatrix[ 1 ][ 2 ], - viewMatrix[ 2 ][ 2 ] ) );
		} else {
			cameraToFrag = normalize( vWorldPosition - cameraPosition );
		}
		vec3 worldNormal = inverseTransformDirection( normal, viewMatrix );
		#ifdef ENVMAP_MODE_REFLECTION
			vec3 reflectVec = reflect( cameraToFrag, worldNormal );
		#else
			vec3 reflectVec = refract( cameraToFrag, worldNormal, refractionRatio );
		#endif
	#else
		vec3 reflectVec = vReflect;
	#endif
	#ifdef ENVMAP_TYPE_CUBE
		vec4 envColor = textureCube( envMap, envMapRotation * vec3( flipEnvMap * reflectVec.x, reflectVec.yz ) );
	#else
		vec4 envColor = vec4( 0.0 );
	#endif
	#ifdef ENVMAP_BLENDING_MULTIPLY
		outgoingLight = mix( outgoingLight, outgoingLight * envColor.xyz, specularStrength * reflectivity );
	#elif defined( ENVMAP_BLENDING_MIX )
		outgoingLight = mix( outgoingLight, envColor.xyz, specularStrength * reflectivity );
	#elif defined( ENVMAP_BLENDING_ADD )
		outgoingLight += envColor.xyz * specularStrength * reflectivity;
	#endif
#endif`,fh=`#ifdef USE_ENVMAP
	uniform float envMapIntensity;
	uniform float flipEnvMap;
	uniform mat3 envMapRotation;
	#ifdef ENVMAP_TYPE_CUBE
		uniform samplerCube envMap;
	#else
		uniform sampler2D envMap;
	#endif
	
#endif`,hh=`#ifdef USE_ENVMAP
	uniform float reflectivity;
	#if defined( USE_BUMPMAP ) || defined( USE_NORMALMAP ) || defined( PHONG ) || defined( LAMBERT )
		#define ENV_WORLDPOS
	#endif
	#ifdef ENV_WORLDPOS
		varying vec3 vWorldPosition;
		uniform float refractionRatio;
	#else
		varying vec3 vReflect;
	#endif
#endif`,dh=`#ifdef USE_ENVMAP
	#if defined( USE_BUMPMAP ) || defined( USE_NORMALMAP ) || defined( PHONG ) || defined( LAMBERT )
		#define ENV_WORLDPOS
	#endif
	#ifdef ENV_WORLDPOS
		
		varying vec3 vWorldPosition;
	#else
		varying vec3 vReflect;
		uniform float refractionRatio;
	#endif
#endif`,ph=`#ifdef USE_ENVMAP
	#ifdef ENV_WORLDPOS
		vWorldPosition = worldPosition.xyz;
	#else
		vec3 cameraToVertex;
		if ( isOrthographic ) {
			cameraToVertex = normalize( vec3( - viewMatrix[ 0 ][ 2 ], - viewMatrix[ 1 ][ 2 ], - viewMatrix[ 2 ][ 2 ] ) );
		} else {
			cameraToVertex = normalize( worldPosition.xyz - cameraPosition );
		}
		vec3 worldNormal = inverseTransformDirection( transformedNormal, viewMatrix );
		#ifdef ENVMAP_MODE_REFLECTION
			vReflect = reflect( cameraToVertex, worldNormal );
		#else
			vReflect = refract( cameraToVertex, worldNormal, refractionRatio );
		#endif
	#endif
#endif`,mh=`#ifdef USE_FOG
	vFogDepth = - mvPosition.z;
#endif`,_h=`#ifdef USE_FOG
	varying float vFogDepth;
#endif`,gh=`#ifdef USE_FOG
	#ifdef FOG_EXP2
		float fogFactor = 1.0 - exp( - fogDensity * fogDensity * vFogDepth * vFogDepth );
	#else
		float fogFactor = smoothstep( fogNear, fogFar, vFogDepth );
	#endif
	gl_FragColor.rgb = mix( gl_FragColor.rgb, fogColor, fogFactor );
#endif`,xh=`#ifdef USE_FOG
	uniform vec3 fogColor;
	varying float vFogDepth;
	#ifdef FOG_EXP2
		uniform float fogDensity;
	#else
		uniform float fogNear;
		uniform float fogFar;
	#endif
#endif`,vh=`#ifdef USE_GRADIENTMAP
	uniform sampler2D gradientMap;
#endif
vec3 getGradientIrradiance( vec3 normal, vec3 lightDirection ) {
	float dotNL = dot( normal, lightDirection );
	vec2 coord = vec2( dotNL * 0.5 + 0.5, 0.0 );
	#ifdef USE_GRADIENTMAP
		return vec3( texture2D( gradientMap, coord ).r );
	#else
		vec2 fw = fwidth( coord ) * 0.5;
		return mix( vec3( 0.7 ), vec3( 1.0 ), smoothstep( 0.7 - fw.x, 0.7 + fw.x, coord.x ) );
	#endif
}`,yh=`#ifdef USE_LIGHTMAP
	uniform sampler2D lightMap;
	uniform float lightMapIntensity;
#endif`,Sh=`LambertMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.specularStrength = specularStrength;`,Mh=`varying vec3 vViewPosition;
struct LambertMaterial {
	vec3 diffuseColor;
	float specularStrength;
};
void RE_Direct_Lambert( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in LambertMaterial material, inout ReflectedLight reflectedLight ) {
	float dotNL = saturate( dot( geometryNormal, directLight.direction ) );
	vec3 irradiance = dotNL * directLight.color;
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
void RE_IndirectDiffuse_Lambert( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in LambertMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
#define RE_Direct				RE_Direct_Lambert
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Lambert`,Eh=`uniform bool receiveShadow;
uniform vec3 ambientLightColor;
#if defined( USE_LIGHT_PROBES )
	uniform vec3 lightProbe[ 9 ];
#endif
vec3 shGetIrradianceAt( in vec3 normal, in vec3 shCoefficients[ 9 ] ) {
	float x = normal.x, y = normal.y, z = normal.z;
	vec3 result = shCoefficients[ 0 ] * 0.886227;
	result += shCoefficients[ 1 ] * 2.0 * 0.511664 * y;
	result += shCoefficients[ 2 ] * 2.0 * 0.511664 * z;
	result += shCoefficients[ 3 ] * 2.0 * 0.511664 * x;
	result += shCoefficients[ 4 ] * 2.0 * 0.429043 * x * y;
	result += shCoefficients[ 5 ] * 2.0 * 0.429043 * y * z;
	result += shCoefficients[ 6 ] * ( 0.743125 * z * z - 0.247708 );
	result += shCoefficients[ 7 ] * 2.0 * 0.429043 * x * z;
	result += shCoefficients[ 8 ] * 0.429043 * ( x * x - y * y );
	return result;
}
vec3 getLightProbeIrradiance( const in vec3 lightProbe[ 9 ], const in vec3 normal ) {
	vec3 worldNormal = inverseTransformDirection( normal, viewMatrix );
	vec3 irradiance = shGetIrradianceAt( worldNormal, lightProbe );
	return irradiance;
}
vec3 getAmbientLightIrradiance( const in vec3 ambientLightColor ) {
	vec3 irradiance = ambientLightColor;
	return irradiance;
}
float getDistanceAttenuation( const in float lightDistance, const in float cutoffDistance, const in float decayExponent ) {
	float distanceFalloff = 1.0 / max( pow( lightDistance, decayExponent ), 0.01 );
	if ( cutoffDistance > 0.0 ) {
		distanceFalloff *= pow2( saturate( 1.0 - pow4( lightDistance / cutoffDistance ) ) );
	}
	return distanceFalloff;
}
float getSpotAttenuation( const in float coneCosine, const in float penumbraCosine, const in float angleCosine ) {
	return smoothstep( coneCosine, penumbraCosine, angleCosine );
}
#if NUM_DIR_LIGHTS > 0
	struct DirectionalLight {
		vec3 direction;
		vec3 color;
	};
	uniform DirectionalLight directionalLights[ NUM_DIR_LIGHTS ];
	void getDirectionalLightInfo( const in DirectionalLight directionalLight, out IncidentLight light ) {
		light.color = directionalLight.color;
		light.direction = directionalLight.direction;
		light.visible = true;
	}
#endif
#if NUM_POINT_LIGHTS > 0
	struct PointLight {
		vec3 position;
		vec3 color;
		float distance;
		float decay;
	};
	uniform PointLight pointLights[ NUM_POINT_LIGHTS ];
	void getPointLightInfo( const in PointLight pointLight, const in vec3 geometryPosition, out IncidentLight light ) {
		vec3 lVector = pointLight.position - geometryPosition;
		light.direction = normalize( lVector );
		float lightDistance = length( lVector );
		light.color = pointLight.color;
		light.color *= getDistanceAttenuation( lightDistance, pointLight.distance, pointLight.decay );
		light.visible = ( light.color != vec3( 0.0 ) );
	}
#endif
#if NUM_SPOT_LIGHTS > 0
	struct SpotLight {
		vec3 position;
		vec3 direction;
		vec3 color;
		float distance;
		float decay;
		float coneCos;
		float penumbraCos;
	};
	uniform SpotLight spotLights[ NUM_SPOT_LIGHTS ];
	void getSpotLightInfo( const in SpotLight spotLight, const in vec3 geometryPosition, out IncidentLight light ) {
		vec3 lVector = spotLight.position - geometryPosition;
		light.direction = normalize( lVector );
		float angleCos = dot( light.direction, spotLight.direction );
		float spotAttenuation = getSpotAttenuation( spotLight.coneCos, spotLight.penumbraCos, angleCos );
		if ( spotAttenuation > 0.0 ) {
			float lightDistance = length( lVector );
			light.color = spotLight.color * spotAttenuation;
			light.color *= getDistanceAttenuation( lightDistance, spotLight.distance, spotLight.decay );
			light.visible = ( light.color != vec3( 0.0 ) );
		} else {
			light.color = vec3( 0.0 );
			light.visible = false;
		}
	}
#endif
#if NUM_RECT_AREA_LIGHTS > 0
	struct RectAreaLight {
		vec3 color;
		vec3 position;
		vec3 halfWidth;
		vec3 halfHeight;
	};
	uniform sampler2D ltc_1;	uniform sampler2D ltc_2;
	uniform RectAreaLight rectAreaLights[ NUM_RECT_AREA_LIGHTS ];
#endif
#if NUM_HEMI_LIGHTS > 0
	struct HemisphereLight {
		vec3 direction;
		vec3 skyColor;
		vec3 groundColor;
	};
	uniform HemisphereLight hemisphereLights[ NUM_HEMI_LIGHTS ];
	vec3 getHemisphereLightIrradiance( const in HemisphereLight hemiLight, const in vec3 normal ) {
		float dotNL = dot( normal, hemiLight.direction );
		float hemiDiffuseWeight = 0.5 * dotNL + 0.5;
		vec3 irradiance = mix( hemiLight.groundColor, hemiLight.skyColor, hemiDiffuseWeight );
		return irradiance;
	}
#endif`,Th=`#ifdef USE_ENVMAP
	vec3 getIBLIrradiance( const in vec3 normal ) {
		#ifdef ENVMAP_TYPE_CUBE_UV
			vec3 worldNormal = inverseTransformDirection( normal, viewMatrix );
			vec4 envMapColor = textureCubeUV( envMap, envMapRotation * worldNormal, 1.0 );
			return PI * envMapColor.rgb * envMapIntensity;
		#else
			return vec3( 0.0 );
		#endif
	}
	vec3 getIBLRadiance( const in vec3 viewDir, const in vec3 normal, const in float roughness ) {
		#ifdef ENVMAP_TYPE_CUBE_UV
			vec3 reflectVec = reflect( - viewDir, normal );
			reflectVec = normalize( mix( reflectVec, normal, roughness * roughness) );
			reflectVec = inverseTransformDirection( reflectVec, viewMatrix );
			vec4 envMapColor = textureCubeUV( envMap, envMapRotation * reflectVec, roughness );
			return envMapColor.rgb * envMapIntensity;
		#else
			return vec3( 0.0 );
		#endif
	}
	#ifdef USE_ANISOTROPY
		vec3 getIBLAnisotropyRadiance( const in vec3 viewDir, const in vec3 normal, const in float roughness, const in vec3 bitangent, const in float anisotropy ) {
			#ifdef ENVMAP_TYPE_CUBE_UV
				vec3 bentNormal = cross( bitangent, viewDir );
				bentNormal = normalize( cross( bentNormal, bitangent ) );
				bentNormal = normalize( mix( bentNormal, normal, pow2( pow2( 1.0 - anisotropy * ( 1.0 - roughness ) ) ) ) );
				return getIBLRadiance( viewDir, bentNormal, roughness );
			#else
				return vec3( 0.0 );
			#endif
		}
	#endif
#endif`,wh=`ToonMaterial material;
material.diffuseColor = diffuseColor.rgb;`,Ah=`varying vec3 vViewPosition;
struct ToonMaterial {
	vec3 diffuseColor;
};
void RE_Direct_Toon( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in ToonMaterial material, inout ReflectedLight reflectedLight ) {
	vec3 irradiance = getGradientIrradiance( geometryNormal, directLight.direction ) * directLight.color;
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
void RE_IndirectDiffuse_Toon( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in ToonMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
#define RE_Direct				RE_Direct_Toon
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Toon`,bh=`BlinnPhongMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.specularColor = specular;
material.specularShininess = shininess;
material.specularStrength = specularStrength;`,Ch=`varying vec3 vViewPosition;
struct BlinnPhongMaterial {
	vec3 diffuseColor;
	vec3 specularColor;
	float specularShininess;
	float specularStrength;
};
void RE_Direct_BlinnPhong( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in BlinnPhongMaterial material, inout ReflectedLight reflectedLight ) {
	float dotNL = saturate( dot( geometryNormal, directLight.direction ) );
	vec3 irradiance = dotNL * directLight.color;
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
	reflectedLight.directSpecular += irradiance * BRDF_BlinnPhong( directLight.direction, geometryViewDir, geometryNormal, material.specularColor, material.specularShininess ) * material.specularStrength;
}
void RE_IndirectDiffuse_BlinnPhong( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in BlinnPhongMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
#define RE_Direct				RE_Direct_BlinnPhong
#define RE_IndirectDiffuse		RE_IndirectDiffuse_BlinnPhong`,Rh=`PhysicalMaterial material;
material.diffuseColor = diffuseColor.rgb * ( 1.0 - metalnessFactor );
vec3 dxy = max( abs( dFdx( nonPerturbedNormal ) ), abs( dFdy( nonPerturbedNormal ) ) );
float geometryRoughness = max( max( dxy.x, dxy.y ), dxy.z );
material.roughness = max( roughnessFactor, 0.0525 );material.roughness += geometryRoughness;
material.roughness = min( material.roughness, 1.0 );
#ifdef IOR
	material.ior = ior;
	#ifdef USE_SPECULAR
		float specularIntensityFactor = specularIntensity;
		vec3 specularColorFactor = specularColor;
		#ifdef USE_SPECULAR_COLORMAP
			specularColorFactor *= texture2D( specularColorMap, vSpecularColorMapUv ).rgb;
		#endif
		#ifdef USE_SPECULAR_INTENSITYMAP
			specularIntensityFactor *= texture2D( specularIntensityMap, vSpecularIntensityMapUv ).a;
		#endif
		material.specularF90 = mix( specularIntensityFactor, 1.0, metalnessFactor );
	#else
		float specularIntensityFactor = 1.0;
		vec3 specularColorFactor = vec3( 1.0 );
		material.specularF90 = 1.0;
	#endif
	material.specularColor = mix( min( pow2( ( material.ior - 1.0 ) / ( material.ior + 1.0 ) ) * specularColorFactor, vec3( 1.0 ) ) * specularIntensityFactor, diffuseColor.rgb, metalnessFactor );
#else
	material.specularColor = mix( vec3( 0.04 ), diffuseColor.rgb, metalnessFactor );
	material.specularF90 = 1.0;
#endif
#ifdef USE_CLEARCOAT
	material.clearcoat = clearcoat;
	material.clearcoatRoughness = clearcoatRoughness;
	material.clearcoatF0 = vec3( 0.04 );
	material.clearcoatF90 = 1.0;
	#ifdef USE_CLEARCOATMAP
		material.clearcoat *= texture2D( clearcoatMap, vClearcoatMapUv ).x;
	#endif
	#ifdef USE_CLEARCOAT_ROUGHNESSMAP
		material.clearcoatRoughness *= texture2D( clearcoatRoughnessMap, vClearcoatRoughnessMapUv ).y;
	#endif
	material.clearcoat = saturate( material.clearcoat );	material.clearcoatRoughness = max( material.clearcoatRoughness, 0.0525 );
	material.clearcoatRoughness += geometryRoughness;
	material.clearcoatRoughness = min( material.clearcoatRoughness, 1.0 );
#endif
#ifdef USE_DISPERSION
	material.dispersion = dispersion;
#endif
#ifdef USE_IRIDESCENCE
	material.iridescence = iridescence;
	material.iridescenceIOR = iridescenceIOR;
	#ifdef USE_IRIDESCENCEMAP
		material.iridescence *= texture2D( iridescenceMap, vIridescenceMapUv ).r;
	#endif
	#ifdef USE_IRIDESCENCE_THICKNESSMAP
		material.iridescenceThickness = (iridescenceThicknessMaximum - iridescenceThicknessMinimum) * texture2D( iridescenceThicknessMap, vIridescenceThicknessMapUv ).g + iridescenceThicknessMinimum;
	#else
		material.iridescenceThickness = iridescenceThicknessMaximum;
	#endif
#endif
#ifdef USE_SHEEN
	material.sheenColor = sheenColor;
	#ifdef USE_SHEEN_COLORMAP
		material.sheenColor *= texture2D( sheenColorMap, vSheenColorMapUv ).rgb;
	#endif
	material.sheenRoughness = clamp( sheenRoughness, 0.07, 1.0 );
	#ifdef USE_SHEEN_ROUGHNESSMAP
		material.sheenRoughness *= texture2D( sheenRoughnessMap, vSheenRoughnessMapUv ).a;
	#endif
#endif
#ifdef USE_ANISOTROPY
	#ifdef USE_ANISOTROPYMAP
		mat2 anisotropyMat = mat2( anisotropyVector.x, anisotropyVector.y, - anisotropyVector.y, anisotropyVector.x );
		vec3 anisotropyPolar = texture2D( anisotropyMap, vAnisotropyMapUv ).rgb;
		vec2 anisotropyV = anisotropyMat * normalize( 2.0 * anisotropyPolar.rg - vec2( 1.0 ) ) * anisotropyPolar.b;
	#else
		vec2 anisotropyV = anisotropyVector;
	#endif
	material.anisotropy = length( anisotropyV );
	if( material.anisotropy == 0.0 ) {
		anisotropyV = vec2( 1.0, 0.0 );
	} else {
		anisotropyV /= material.anisotropy;
		material.anisotropy = saturate( material.anisotropy );
	}
	material.alphaT = mix( pow2( material.roughness ), 1.0, pow2( material.anisotropy ) );
	material.anisotropyT = tbn[ 0 ] * anisotropyV.x + tbn[ 1 ] * anisotropyV.y;
	material.anisotropyB = tbn[ 1 ] * anisotropyV.x - tbn[ 0 ] * anisotropyV.y;
#endif`,Ph=`struct PhysicalMaterial {
	vec3 diffuseColor;
	float roughness;
	vec3 specularColor;
	float specularF90;
	float dispersion;
	#ifdef USE_CLEARCOAT
		float clearcoat;
		float clearcoatRoughness;
		vec3 clearcoatF0;
		float clearcoatF90;
	#endif
	#ifdef USE_IRIDESCENCE
		float iridescence;
		float iridescenceIOR;
		float iridescenceThickness;
		vec3 iridescenceFresnel;
		vec3 iridescenceF0;
	#endif
	#ifdef USE_SHEEN
		vec3 sheenColor;
		float sheenRoughness;
	#endif
	#ifdef IOR
		float ior;
	#endif
	#ifdef USE_TRANSMISSION
		float transmission;
		float transmissionAlpha;
		float thickness;
		float attenuationDistance;
		vec3 attenuationColor;
	#endif
	#ifdef USE_ANISOTROPY
		float anisotropy;
		float alphaT;
		vec3 anisotropyT;
		vec3 anisotropyB;
	#endif
};
vec3 clearcoatSpecularDirect = vec3( 0.0 );
vec3 clearcoatSpecularIndirect = vec3( 0.0 );
vec3 sheenSpecularDirect = vec3( 0.0 );
vec3 sheenSpecularIndirect = vec3(0.0 );
vec3 Schlick_to_F0( const in vec3 f, const in float f90, const in float dotVH ) {
    float x = clamp( 1.0 - dotVH, 0.0, 1.0 );
    float x2 = x * x;
    float x5 = clamp( x * x2 * x2, 0.0, 0.9999 );
    return ( f - vec3( f90 ) * x5 ) / ( 1.0 - x5 );
}
float V_GGX_SmithCorrelated( const in float alpha, const in float dotNL, const in float dotNV ) {
	float a2 = pow2( alpha );
	float gv = dotNL * sqrt( a2 + ( 1.0 - a2 ) * pow2( dotNV ) );
	float gl = dotNV * sqrt( a2 + ( 1.0 - a2 ) * pow2( dotNL ) );
	return 0.5 / max( gv + gl, EPSILON );
}
float D_GGX( const in float alpha, const in float dotNH ) {
	float a2 = pow2( alpha );
	float denom = pow2( dotNH ) * ( a2 - 1.0 ) + 1.0;
	return RECIPROCAL_PI * a2 / pow2( denom );
}
#ifdef USE_ANISOTROPY
	float V_GGX_SmithCorrelated_Anisotropic( const in float alphaT, const in float alphaB, const in float dotTV, const in float dotBV, const in float dotTL, const in float dotBL, const in float dotNV, const in float dotNL ) {
		float gv = dotNL * length( vec3( alphaT * dotTV, alphaB * dotBV, dotNV ) );
		float gl = dotNV * length( vec3( alphaT * dotTL, alphaB * dotBL, dotNL ) );
		float v = 0.5 / ( gv + gl );
		return saturate(v);
	}
	float D_GGX_Anisotropic( const in float alphaT, const in float alphaB, const in float dotNH, const in float dotTH, const in float dotBH ) {
		float a2 = alphaT * alphaB;
		highp vec3 v = vec3( alphaB * dotTH, alphaT * dotBH, a2 * dotNH );
		highp float v2 = dot( v, v );
		float w2 = a2 / v2;
		return RECIPROCAL_PI * a2 * pow2 ( w2 );
	}
#endif
#ifdef USE_CLEARCOAT
	vec3 BRDF_GGX_Clearcoat( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, const in PhysicalMaterial material) {
		vec3 f0 = material.clearcoatF0;
		float f90 = material.clearcoatF90;
		float roughness = material.clearcoatRoughness;
		float alpha = pow2( roughness );
		vec3 halfDir = normalize( lightDir + viewDir );
		float dotNL = saturate( dot( normal, lightDir ) );
		float dotNV = saturate( dot( normal, viewDir ) );
		float dotNH = saturate( dot( normal, halfDir ) );
		float dotVH = saturate( dot( viewDir, halfDir ) );
		vec3 F = F_Schlick( f0, f90, dotVH );
		float V = V_GGX_SmithCorrelated( alpha, dotNL, dotNV );
		float D = D_GGX( alpha, dotNH );
		return F * ( V * D );
	}
#endif
vec3 BRDF_GGX( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, const in PhysicalMaterial material ) {
	vec3 f0 = material.specularColor;
	float f90 = material.specularF90;
	float roughness = material.roughness;
	float alpha = pow2( roughness );
	vec3 halfDir = normalize( lightDir + viewDir );
	float dotNL = saturate( dot( normal, lightDir ) );
	float dotNV = saturate( dot( normal, viewDir ) );
	float dotNH = saturate( dot( normal, halfDir ) );
	float dotVH = saturate( dot( viewDir, halfDir ) );
	vec3 F = F_Schlick( f0, f90, dotVH );
	#ifdef USE_IRIDESCENCE
		F = mix( F, material.iridescenceFresnel, material.iridescence );
	#endif
	#ifdef USE_ANISOTROPY
		float dotTL = dot( material.anisotropyT, lightDir );
		float dotTV = dot( material.anisotropyT, viewDir );
		float dotTH = dot( material.anisotropyT, halfDir );
		float dotBL = dot( material.anisotropyB, lightDir );
		float dotBV = dot( material.anisotropyB, viewDir );
		float dotBH = dot( material.anisotropyB, halfDir );
		float V = V_GGX_SmithCorrelated_Anisotropic( material.alphaT, alpha, dotTV, dotBV, dotTL, dotBL, dotNV, dotNL );
		float D = D_GGX_Anisotropic( material.alphaT, alpha, dotNH, dotTH, dotBH );
	#else
		float V = V_GGX_SmithCorrelated( alpha, dotNL, dotNV );
		float D = D_GGX( alpha, dotNH );
	#endif
	return F * ( V * D );
}
vec2 LTC_Uv( const in vec3 N, const in vec3 V, const in float roughness ) {
	const float LUT_SIZE = 64.0;
	const float LUT_SCALE = ( LUT_SIZE - 1.0 ) / LUT_SIZE;
	const float LUT_BIAS = 0.5 / LUT_SIZE;
	float dotNV = saturate( dot( N, V ) );
	vec2 uv = vec2( roughness, sqrt( 1.0 - dotNV ) );
	uv = uv * LUT_SCALE + LUT_BIAS;
	return uv;
}
float LTC_ClippedSphereFormFactor( const in vec3 f ) {
	float l = length( f );
	return max( ( l * l + f.z ) / ( l + 1.0 ), 0.0 );
}
vec3 LTC_EdgeVectorFormFactor( const in vec3 v1, const in vec3 v2 ) {
	float x = dot( v1, v2 );
	float y = abs( x );
	float a = 0.8543985 + ( 0.4965155 + 0.0145206 * y ) * y;
	float b = 3.4175940 + ( 4.1616724 + y ) * y;
	float v = a / b;
	float theta_sintheta = ( x > 0.0 ) ? v : 0.5 * inversesqrt( max( 1.0 - x * x, 1e-7 ) ) - v;
	return cross( v1, v2 ) * theta_sintheta;
}
vec3 LTC_Evaluate( const in vec3 N, const in vec3 V, const in vec3 P, const in mat3 mInv, const in vec3 rectCoords[ 4 ] ) {
	vec3 v1 = rectCoords[ 1 ] - rectCoords[ 0 ];
	vec3 v2 = rectCoords[ 3 ] - rectCoords[ 0 ];
	vec3 lightNormal = cross( v1, v2 );
	if( dot( lightNormal, P - rectCoords[ 0 ] ) < 0.0 ) return vec3( 0.0 );
	vec3 T1, T2;
	T1 = normalize( V - N * dot( V, N ) );
	T2 = - cross( N, T1 );
	mat3 mat = mInv * transposeMat3( mat3( T1, T2, N ) );
	vec3 coords[ 4 ];
	coords[ 0 ] = mat * ( rectCoords[ 0 ] - P );
	coords[ 1 ] = mat * ( rectCoords[ 1 ] - P );
	coords[ 2 ] = mat * ( rectCoords[ 2 ] - P );
	coords[ 3 ] = mat * ( rectCoords[ 3 ] - P );
	coords[ 0 ] = normalize( coords[ 0 ] );
	coords[ 1 ] = normalize( coords[ 1 ] );
	coords[ 2 ] = normalize( coords[ 2 ] );
	coords[ 3 ] = normalize( coords[ 3 ] );
	vec3 vectorFormFactor = vec3( 0.0 );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 0 ], coords[ 1 ] );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 1 ], coords[ 2 ] );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 2 ], coords[ 3 ] );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 3 ], coords[ 0 ] );
	float result = LTC_ClippedSphereFormFactor( vectorFormFactor );
	return vec3( result );
}
#if defined( USE_SHEEN )
float D_Charlie( float roughness, float dotNH ) {
	float alpha = pow2( roughness );
	float invAlpha = 1.0 / alpha;
	float cos2h = dotNH * dotNH;
	float sin2h = max( 1.0 - cos2h, 0.0078125 );
	return ( 2.0 + invAlpha ) * pow( sin2h, invAlpha * 0.5 ) / ( 2.0 * PI );
}
float V_Neubelt( float dotNV, float dotNL ) {
	return saturate( 1.0 / ( 4.0 * ( dotNL + dotNV - dotNL * dotNV ) ) );
}
vec3 BRDF_Sheen( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, vec3 sheenColor, const in float sheenRoughness ) {
	vec3 halfDir = normalize( lightDir + viewDir );
	float dotNL = saturate( dot( normal, lightDir ) );
	float dotNV = saturate( dot( normal, viewDir ) );
	float dotNH = saturate( dot( normal, halfDir ) );
	float D = D_Charlie( sheenRoughness, dotNH );
	float V = V_Neubelt( dotNV, dotNL );
	return sheenColor * ( D * V );
}
#endif
float IBLSheenBRDF( const in vec3 normal, const in vec3 viewDir, const in float roughness ) {
	float dotNV = saturate( dot( normal, viewDir ) );
	float r2 = roughness * roughness;
	float a = roughness < 0.25 ? -339.2 * r2 + 161.4 * roughness - 25.9 : -8.48 * r2 + 14.3 * roughness - 9.95;
	float b = roughness < 0.25 ? 44.0 * r2 - 23.7 * roughness + 3.26 : 1.97 * r2 - 3.27 * roughness + 0.72;
	float DG = exp( a * dotNV + b ) + ( roughness < 0.25 ? 0.0 : 0.1 * ( roughness - 0.25 ) );
	return saturate( DG * RECIPROCAL_PI );
}
vec2 DFGApprox( const in vec3 normal, const in vec3 viewDir, const in float roughness ) {
	float dotNV = saturate( dot( normal, viewDir ) );
	const vec4 c0 = vec4( - 1, - 0.0275, - 0.572, 0.022 );
	const vec4 c1 = vec4( 1, 0.0425, 1.04, - 0.04 );
	vec4 r = roughness * c0 + c1;
	float a004 = min( r.x * r.x, exp2( - 9.28 * dotNV ) ) * r.x + r.y;
	vec2 fab = vec2( - 1.04, 1.04 ) * a004 + r.zw;
	return fab;
}
vec3 EnvironmentBRDF( const in vec3 normal, const in vec3 viewDir, const in vec3 specularColor, const in float specularF90, const in float roughness ) {
	vec2 fab = DFGApprox( normal, viewDir, roughness );
	return specularColor * fab.x + specularF90 * fab.y;
}
#ifdef USE_IRIDESCENCE
void computeMultiscatteringIridescence( const in vec3 normal, const in vec3 viewDir, const in vec3 specularColor, const in float specularF90, const in float iridescence, const in vec3 iridescenceF0, const in float roughness, inout vec3 singleScatter, inout vec3 multiScatter ) {
#else
void computeMultiscattering( const in vec3 normal, const in vec3 viewDir, const in vec3 specularColor, const in float specularF90, const in float roughness, inout vec3 singleScatter, inout vec3 multiScatter ) {
#endif
	vec2 fab = DFGApprox( normal, viewDir, roughness );
	#ifdef USE_IRIDESCENCE
		vec3 Fr = mix( specularColor, iridescenceF0, iridescence );
	#else
		vec3 Fr = specularColor;
	#endif
	vec3 FssEss = Fr * fab.x + specularF90 * fab.y;
	float Ess = fab.x + fab.y;
	float Ems = 1.0 - Ess;
	vec3 Favg = Fr + ( 1.0 - Fr ) * 0.047619;	vec3 Fms = FssEss * Favg / ( 1.0 - Ems * Favg );
	singleScatter += FssEss;
	multiScatter += Fms * Ems;
}
#if NUM_RECT_AREA_LIGHTS > 0
	void RE_Direct_RectArea_Physical( const in RectAreaLight rectAreaLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {
		vec3 normal = geometryNormal;
		vec3 viewDir = geometryViewDir;
		vec3 position = geometryPosition;
		vec3 lightPos = rectAreaLight.position;
		vec3 halfWidth = rectAreaLight.halfWidth;
		vec3 halfHeight = rectAreaLight.halfHeight;
		vec3 lightColor = rectAreaLight.color;
		float roughness = material.roughness;
		vec3 rectCoords[ 4 ];
		rectCoords[ 0 ] = lightPos + halfWidth - halfHeight;		rectCoords[ 1 ] = lightPos - halfWidth - halfHeight;
		rectCoords[ 2 ] = lightPos - halfWidth + halfHeight;
		rectCoords[ 3 ] = lightPos + halfWidth + halfHeight;
		vec2 uv = LTC_Uv( normal, viewDir, roughness );
		vec4 t1 = texture2D( ltc_1, uv );
		vec4 t2 = texture2D( ltc_2, uv );
		mat3 mInv = mat3(
			vec3( t1.x, 0, t1.y ),
			vec3(    0, 1,    0 ),
			vec3( t1.z, 0, t1.w )
		);
		vec3 fresnel = ( material.specularColor * t2.x + ( vec3( 1.0 ) - material.specularColor ) * t2.y );
		reflectedLight.directSpecular += lightColor * fresnel * LTC_Evaluate( normal, viewDir, position, mInv, rectCoords );
		reflectedLight.directDiffuse += lightColor * material.diffuseColor * LTC_Evaluate( normal, viewDir, position, mat3( 1.0 ), rectCoords );
	}
#endif
void RE_Direct_Physical( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {
	float dotNL = saturate( dot( geometryNormal, directLight.direction ) );
	vec3 irradiance = dotNL * directLight.color;
	#ifdef USE_CLEARCOAT
		float dotNLcc = saturate( dot( geometryClearcoatNormal, directLight.direction ) );
		vec3 ccIrradiance = dotNLcc * directLight.color;
		clearcoatSpecularDirect += ccIrradiance * BRDF_GGX_Clearcoat( directLight.direction, geometryViewDir, geometryClearcoatNormal, material );
	#endif
	#ifdef USE_SHEEN
		sheenSpecularDirect += irradiance * BRDF_Sheen( directLight.direction, geometryViewDir, geometryNormal, material.sheenColor, material.sheenRoughness );
	#endif
	reflectedLight.directSpecular += irradiance * BRDF_GGX( directLight.direction, geometryViewDir, geometryNormal, material );
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
void RE_IndirectDiffuse_Physical( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
void RE_IndirectSpecular_Physical( const in vec3 radiance, const in vec3 irradiance, const in vec3 clearcoatRadiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight) {
	#ifdef USE_CLEARCOAT
		clearcoatSpecularIndirect += clearcoatRadiance * EnvironmentBRDF( geometryClearcoatNormal, geometryViewDir, material.clearcoatF0, material.clearcoatF90, material.clearcoatRoughness );
	#endif
	#ifdef USE_SHEEN
		sheenSpecularIndirect += irradiance * material.sheenColor * IBLSheenBRDF( geometryNormal, geometryViewDir, material.sheenRoughness );
	#endif
	vec3 singleScattering = vec3( 0.0 );
	vec3 multiScattering = vec3( 0.0 );
	vec3 cosineWeightedIrradiance = irradiance * RECIPROCAL_PI;
	#ifdef USE_IRIDESCENCE
		computeMultiscatteringIridescence( geometryNormal, geometryViewDir, material.specularColor, material.specularF90, material.iridescence, material.iridescenceFresnel, material.roughness, singleScattering, multiScattering );
	#else
		computeMultiscattering( geometryNormal, geometryViewDir, material.specularColor, material.specularF90, material.roughness, singleScattering, multiScattering );
	#endif
	vec3 totalScattering = singleScattering + multiScattering;
	vec3 diffuse = material.diffuseColor * ( 1.0 - max( max( totalScattering.r, totalScattering.g ), totalScattering.b ) );
	reflectedLight.indirectSpecular += radiance * singleScattering;
	reflectedLight.indirectSpecular += multiScattering * cosineWeightedIrradiance;
	reflectedLight.indirectDiffuse += diffuse * cosineWeightedIrradiance;
}
#define RE_Direct				RE_Direct_Physical
#define RE_Direct_RectArea		RE_Direct_RectArea_Physical
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Physical
#define RE_IndirectSpecular		RE_IndirectSpecular_Physical
float computeSpecularOcclusion( const in float dotNV, const in float ambientOcclusion, const in float roughness ) {
	return saturate( pow( dotNV + ambientOcclusion, exp2( - 16.0 * roughness - 1.0 ) ) - 1.0 + ambientOcclusion );
}`,Lh=`
vec3 geometryPosition = - vViewPosition;
vec3 geometryNormal = normal;
vec3 geometryViewDir = ( isOrthographic ) ? vec3( 0, 0, 1 ) : normalize( vViewPosition );
vec3 geometryClearcoatNormal = vec3( 0.0 );
#ifdef USE_CLEARCOAT
	geometryClearcoatNormal = clearcoatNormal;
#endif
#ifdef USE_IRIDESCENCE
	float dotNVi = saturate( dot( normal, geometryViewDir ) );
	if ( material.iridescenceThickness == 0.0 ) {
		material.iridescence = 0.0;
	} else {
		material.iridescence = saturate( material.iridescence );
	}
	if ( material.iridescence > 0.0 ) {
		material.iridescenceFresnel = evalIridescence( 1.0, material.iridescenceIOR, dotNVi, material.iridescenceThickness, material.specularColor );
		material.iridescenceF0 = Schlick_to_F0( material.iridescenceFresnel, 1.0, dotNVi );
	}
#endif
IncidentLight directLight;
#if ( NUM_POINT_LIGHTS > 0 ) && defined( RE_Direct )
	PointLight pointLight;
	#if defined( USE_SHADOWMAP ) && NUM_POINT_LIGHT_SHADOWS > 0
	PointLightShadow pointLightShadow;
	#endif
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_POINT_LIGHTS; i ++ ) {
		pointLight = pointLights[ i ];
		getPointLightInfo( pointLight, geometryPosition, directLight );
		#if defined( USE_SHADOWMAP ) && ( UNROLLED_LOOP_INDEX < NUM_POINT_LIGHT_SHADOWS )
		pointLightShadow = pointLightShadows[ i ];
		directLight.color *= ( directLight.visible && receiveShadow ) ? getPointShadow( pointShadowMap[ i ], pointLightShadow.shadowMapSize, pointLightShadow.shadowIntensity, pointLightShadow.shadowBias, pointLightShadow.shadowRadius, vPointShadowCoord[ i ], pointLightShadow.shadowCameraNear, pointLightShadow.shadowCameraFar ) : 1.0;
		#endif
		RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if ( NUM_SPOT_LIGHTS > 0 ) && defined( RE_Direct )
	SpotLight spotLight;
	vec4 spotColor;
	vec3 spotLightCoord;
	bool inSpotLightMap;
	#if defined( USE_SHADOWMAP ) && NUM_SPOT_LIGHT_SHADOWS > 0
	SpotLightShadow spotLightShadow;
	#endif
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_SPOT_LIGHTS; i ++ ) {
		spotLight = spotLights[ i ];
		getSpotLightInfo( spotLight, geometryPosition, directLight );
		#if ( UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS_WITH_MAPS )
		#define SPOT_LIGHT_MAP_INDEX UNROLLED_LOOP_INDEX
		#elif ( UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS )
		#define SPOT_LIGHT_MAP_INDEX NUM_SPOT_LIGHT_MAPS
		#else
		#define SPOT_LIGHT_MAP_INDEX ( UNROLLED_LOOP_INDEX - NUM_SPOT_LIGHT_SHADOWS + NUM_SPOT_LIGHT_SHADOWS_WITH_MAPS )
		#endif
		#if ( SPOT_LIGHT_MAP_INDEX < NUM_SPOT_LIGHT_MAPS )
			spotLightCoord = vSpotLightCoord[ i ].xyz / vSpotLightCoord[ i ].w;
			inSpotLightMap = all( lessThan( abs( spotLightCoord * 2. - 1. ), vec3( 1.0 ) ) );
			spotColor = texture2D( spotLightMap[ SPOT_LIGHT_MAP_INDEX ], spotLightCoord.xy );
			directLight.color = inSpotLightMap ? directLight.color * spotColor.rgb : directLight.color;
		#endif
		#undef SPOT_LIGHT_MAP_INDEX
		#if defined( USE_SHADOWMAP ) && ( UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS )
		spotLightShadow = spotLightShadows[ i ];
		directLight.color *= ( directLight.visible && receiveShadow ) ? getShadow( spotShadowMap[ i ], spotLightShadow.shadowMapSize, spotLightShadow.shadowIntensity, spotLightShadow.shadowBias, spotLightShadow.shadowRadius, vSpotLightCoord[ i ] ) : 1.0;
		#endif
		RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if ( NUM_DIR_LIGHTS > 0 ) && defined( RE_Direct )
	DirectionalLight directionalLight;
	#if defined( USE_SHADOWMAP ) && NUM_DIR_LIGHT_SHADOWS > 0
	DirectionalLightShadow directionalLightShadow;
	#endif
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_DIR_LIGHTS; i ++ ) {
		directionalLight = directionalLights[ i ];
		getDirectionalLightInfo( directionalLight, directLight );
		#if defined( USE_SHADOWMAP ) && ( UNROLLED_LOOP_INDEX < NUM_DIR_LIGHT_SHADOWS )
		directionalLightShadow = directionalLightShadows[ i ];
		directLight.color *= ( directLight.visible && receiveShadow ) ? getShadow( directionalShadowMap[ i ], directionalLightShadow.shadowMapSize, directionalLightShadow.shadowIntensity, directionalLightShadow.shadowBias, directionalLightShadow.shadowRadius, vDirectionalShadowCoord[ i ] ) : 1.0;
		#endif
		RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if ( NUM_RECT_AREA_LIGHTS > 0 ) && defined( RE_Direct_RectArea )
	RectAreaLight rectAreaLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_RECT_AREA_LIGHTS; i ++ ) {
		rectAreaLight = rectAreaLights[ i ];
		RE_Direct_RectArea( rectAreaLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if defined( RE_IndirectDiffuse )
	vec3 iblIrradiance = vec3( 0.0 );
	vec3 irradiance = getAmbientLightIrradiance( ambientLightColor );
	#if defined( USE_LIGHT_PROBES )
		irradiance += getLightProbeIrradiance( lightProbe, geometryNormal );
	#endif
	#if ( NUM_HEMI_LIGHTS > 0 )
		#pragma unroll_loop_start
		for ( int i = 0; i < NUM_HEMI_LIGHTS; i ++ ) {
			irradiance += getHemisphereLightIrradiance( hemisphereLights[ i ], geometryNormal );
		}
		#pragma unroll_loop_end
	#endif
#endif
#if defined( RE_IndirectSpecular )
	vec3 radiance = vec3( 0.0 );
	vec3 clearcoatRadiance = vec3( 0.0 );
#endif`,Dh=`#if defined( RE_IndirectDiffuse )
	#ifdef USE_LIGHTMAP
		vec4 lightMapTexel = texture2D( lightMap, vLightMapUv );
		vec3 lightMapIrradiance = lightMapTexel.rgb * lightMapIntensity;
		irradiance += lightMapIrradiance;
	#endif
	#if defined( USE_ENVMAP ) && defined( STANDARD ) && defined( ENVMAP_TYPE_CUBE_UV )
		iblIrradiance += getIBLIrradiance( geometryNormal );
	#endif
#endif
#if defined( USE_ENVMAP ) && defined( RE_IndirectSpecular )
	#ifdef USE_ANISOTROPY
		radiance += getIBLAnisotropyRadiance( geometryViewDir, geometryNormal, material.roughness, material.anisotropyB, material.anisotropy );
	#else
		radiance += getIBLRadiance( geometryViewDir, geometryNormal, material.roughness );
	#endif
	#ifdef USE_CLEARCOAT
		clearcoatRadiance += getIBLRadiance( geometryViewDir, geometryClearcoatNormal, material.clearcoatRoughness );
	#endif
#endif`,Ih=`#if defined( RE_IndirectDiffuse )
	RE_IndirectDiffuse( irradiance, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
#endif
#if defined( RE_IndirectSpecular )
	RE_IndirectSpecular( radiance, iblIrradiance, clearcoatRadiance, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
#endif`,Uh=`#if defined( USE_LOGDEPTHBUF )
	gl_FragDepth = vIsPerspective == 0.0 ? gl_FragCoord.z : log2( vFragDepth ) * logDepthBufFC * 0.5;
#endif`,Fh=`#if defined( USE_LOGDEPTHBUF )
	uniform float logDepthBufFC;
	varying float vFragDepth;
	varying float vIsPerspective;
#endif`,Nh=`#ifdef USE_LOGDEPTHBUF
	varying float vFragDepth;
	varying float vIsPerspective;
#endif`,Oh=`#ifdef USE_LOGDEPTHBUF
	vFragDepth = 1.0 + gl_Position.w;
	vIsPerspective = float( isPerspectiveMatrix( projectionMatrix ) );
#endif`,Bh=`#ifdef USE_MAP
	vec4 sampledDiffuseColor = texture2D( map, vMapUv );
	#ifdef DECODE_VIDEO_TEXTURE
		sampledDiffuseColor = sRGBTransferEOTF( sampledDiffuseColor );
	#endif
	diffuseColor *= sampledDiffuseColor;
#endif`,zh=`#ifdef USE_MAP
	uniform sampler2D map;
#endif`,kh=`#if defined( USE_MAP ) || defined( USE_ALPHAMAP )
	#if defined( USE_POINTS_UV )
		vec2 uv = vUv;
	#else
		vec2 uv = ( uvTransform * vec3( gl_PointCoord.x, 1.0 - gl_PointCoord.y, 1 ) ).xy;
	#endif
#endif
#ifdef USE_MAP
	diffuseColor *= texture2D( map, uv );
#endif
#ifdef USE_ALPHAMAP
	diffuseColor.a *= texture2D( alphaMap, uv ).g;
#endif`,Hh=`#if defined( USE_POINTS_UV )
	varying vec2 vUv;
#else
	#if defined( USE_MAP ) || defined( USE_ALPHAMAP )
		uniform mat3 uvTransform;
	#endif
#endif
#ifdef USE_MAP
	uniform sampler2D map;
#endif
#ifdef USE_ALPHAMAP
	uniform sampler2D alphaMap;
#endif`,Vh=`float metalnessFactor = metalness;
#ifdef USE_METALNESSMAP
	vec4 texelMetalness = texture2D( metalnessMap, vMetalnessMapUv );
	metalnessFactor *= texelMetalness.b;
#endif`,Gh=`#ifdef USE_METALNESSMAP
	uniform sampler2D metalnessMap;
#endif`,Wh=`#ifdef USE_INSTANCING_MORPH
	float morphTargetInfluences[ MORPHTARGETS_COUNT ];
	float morphTargetBaseInfluence = texelFetch( morphTexture, ivec2( 0, gl_InstanceID ), 0 ).r;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		morphTargetInfluences[i] =  texelFetch( morphTexture, ivec2( i + 1, gl_InstanceID ), 0 ).r;
	}
#endif`,Xh=`#if defined( USE_MORPHCOLORS )
	vColor *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		#if defined( USE_COLOR_ALPHA )
			if ( morphTargetInfluences[ i ] != 0.0 ) vColor += getMorph( gl_VertexID, i, 2 ) * morphTargetInfluences[ i ];
		#elif defined( USE_COLOR )
			if ( morphTargetInfluences[ i ] != 0.0 ) vColor += getMorph( gl_VertexID, i, 2 ).rgb * morphTargetInfluences[ i ];
		#endif
	}
#endif`,qh=`#ifdef USE_MORPHNORMALS
	objectNormal *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		if ( morphTargetInfluences[ i ] != 0.0 ) objectNormal += getMorph( gl_VertexID, i, 1 ).xyz * morphTargetInfluences[ i ];
	}
#endif`,$h=`#ifdef USE_MORPHTARGETS
	#ifndef USE_INSTANCING_MORPH
		uniform float morphTargetBaseInfluence;
		uniform float morphTargetInfluences[ MORPHTARGETS_COUNT ];
	#endif
	uniform sampler2DArray morphTargetsTexture;
	uniform ivec2 morphTargetsTextureSize;
	vec4 getMorph( const in int vertexIndex, const in int morphTargetIndex, const in int offset ) {
		int texelIndex = vertexIndex * MORPHTARGETS_TEXTURE_STRIDE + offset;
		int y = texelIndex / morphTargetsTextureSize.x;
		int x = texelIndex - y * morphTargetsTextureSize.x;
		ivec3 morphUV = ivec3( x, y, morphTargetIndex );
		return texelFetch( morphTargetsTexture, morphUV, 0 );
	}
#endif`,Yh=`#ifdef USE_MORPHTARGETS
	transformed *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		if ( morphTargetInfluences[ i ] != 0.0 ) transformed += getMorph( gl_VertexID, i, 0 ).xyz * morphTargetInfluences[ i ];
	}
#endif`,jh=`float faceDirection = gl_FrontFacing ? 1.0 : - 1.0;
#ifdef FLAT_SHADED
	vec3 fdx = dFdx( vViewPosition );
	vec3 fdy = dFdy( vViewPosition );
	vec3 normal = normalize( cross( fdx, fdy ) );
#else
	vec3 normal = normalize( vNormal );
	#ifdef DOUBLE_SIDED
		normal *= faceDirection;
	#endif
#endif
#if defined( USE_NORMALMAP_TANGENTSPACE ) || defined( USE_CLEARCOAT_NORMALMAP ) || defined( USE_ANISOTROPY )
	#ifdef USE_TANGENT
		mat3 tbn = mat3( normalize( vTangent ), normalize( vBitangent ), normal );
	#else
		mat3 tbn = getTangentFrame( - vViewPosition, normal,
		#if defined( USE_NORMALMAP )
			vNormalMapUv
		#elif defined( USE_CLEARCOAT_NORMALMAP )
			vClearcoatNormalMapUv
		#else
			vUv
		#endif
		);
	#endif
	#if defined( DOUBLE_SIDED ) && ! defined( FLAT_SHADED )
		tbn[0] *= faceDirection;
		tbn[1] *= faceDirection;
	#endif
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	#ifdef USE_TANGENT
		mat3 tbn2 = mat3( normalize( vTangent ), normalize( vBitangent ), normal );
	#else
		mat3 tbn2 = getTangentFrame( - vViewPosition, normal, vClearcoatNormalMapUv );
	#endif
	#if defined( DOUBLE_SIDED ) && ! defined( FLAT_SHADED )
		tbn2[0] *= faceDirection;
		tbn2[1] *= faceDirection;
	#endif
#endif
vec3 nonPerturbedNormal = normal;`,Kh=`#ifdef USE_NORMALMAP_OBJECTSPACE
	normal = texture2D( normalMap, vNormalMapUv ).xyz * 2.0 - 1.0;
	#ifdef FLIP_SIDED
		normal = - normal;
	#endif
	#ifdef DOUBLE_SIDED
		normal = normal * faceDirection;
	#endif
	normal = normalize( normalMatrix * normal );
#elif defined( USE_NORMALMAP_TANGENTSPACE )
	vec3 mapN = texture2D( normalMap, vNormalMapUv ).xyz * 2.0 - 1.0;
	mapN.xy *= normalScale;
	normal = normalize( tbn * mapN );
#elif defined( USE_BUMPMAP )
	normal = perturbNormalArb( - vViewPosition, normal, dHdxy_fwd(), faceDirection );
#endif`,Zh=`#ifndef FLAT_SHADED
	varying vec3 vNormal;
	#ifdef USE_TANGENT
		varying vec3 vTangent;
		varying vec3 vBitangent;
	#endif
#endif`,Jh=`#ifndef FLAT_SHADED
	varying vec3 vNormal;
	#ifdef USE_TANGENT
		varying vec3 vTangent;
		varying vec3 vBitangent;
	#endif
#endif`,Qh=`#ifndef FLAT_SHADED
	vNormal = normalize( transformedNormal );
	#ifdef USE_TANGENT
		vTangent = normalize( transformedTangent );
		vBitangent = normalize( cross( vNormal, vTangent ) * tangent.w );
	#endif
#endif`,td=`#ifdef USE_NORMALMAP
	uniform sampler2D normalMap;
	uniform vec2 normalScale;
#endif
#ifdef USE_NORMALMAP_OBJECTSPACE
	uniform mat3 normalMatrix;
#endif
#if ! defined ( USE_TANGENT ) && ( defined ( USE_NORMALMAP_TANGENTSPACE ) || defined ( USE_CLEARCOAT_NORMALMAP ) || defined( USE_ANISOTROPY ) )
	mat3 getTangentFrame( vec3 eye_pos, vec3 surf_norm, vec2 uv ) {
		vec3 q0 = dFdx( eye_pos.xyz );
		vec3 q1 = dFdy( eye_pos.xyz );
		vec2 st0 = dFdx( uv.st );
		vec2 st1 = dFdy( uv.st );
		vec3 N = surf_norm;
		vec3 q1perp = cross( q1, N );
		vec3 q0perp = cross( N, q0 );
		vec3 T = q1perp * st0.x + q0perp * st1.x;
		vec3 B = q1perp * st0.y + q0perp * st1.y;
		float det = max( dot( T, T ), dot( B, B ) );
		float scale = ( det == 0.0 ) ? 0.0 : inversesqrt( det );
		return mat3( T * scale, B * scale, N );
	}
#endif`,ed=`#ifdef USE_CLEARCOAT
	vec3 clearcoatNormal = nonPerturbedNormal;
#endif`,nd=`#ifdef USE_CLEARCOAT_NORMALMAP
	vec3 clearcoatMapN = texture2D( clearcoatNormalMap, vClearcoatNormalMapUv ).xyz * 2.0 - 1.0;
	clearcoatMapN.xy *= clearcoatNormalScale;
	clearcoatNormal = normalize( tbn2 * clearcoatMapN );
#endif`,id=`#ifdef USE_CLEARCOATMAP
	uniform sampler2D clearcoatMap;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	uniform sampler2D clearcoatNormalMap;
	uniform vec2 clearcoatNormalScale;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	uniform sampler2D clearcoatRoughnessMap;
#endif`,rd=`#ifdef USE_IRIDESCENCEMAP
	uniform sampler2D iridescenceMap;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	uniform sampler2D iridescenceThicknessMap;
#endif`,sd=`#ifdef OPAQUE
diffuseColor.a = 1.0;
#endif
#ifdef USE_TRANSMISSION
diffuseColor.a *= material.transmissionAlpha;
#endif
gl_FragColor = vec4( outgoingLight, diffuseColor.a );`,ad=`vec3 packNormalToRGB( const in vec3 normal ) {
	return normalize( normal ) * 0.5 + 0.5;
}
vec3 unpackRGBToNormal( const in vec3 rgb ) {
	return 2.0 * rgb.xyz - 1.0;
}
const float PackUpscale = 256. / 255.;const float UnpackDownscale = 255. / 256.;const float ShiftRight8 = 1. / 256.;
const float Inv255 = 1. / 255.;
const vec4 PackFactors = vec4( 1.0, 256.0, 256.0 * 256.0, 256.0 * 256.0 * 256.0 );
const vec2 UnpackFactors2 = vec2( UnpackDownscale, 1.0 / PackFactors.g );
const vec3 UnpackFactors3 = vec3( UnpackDownscale / PackFactors.rg, 1.0 / PackFactors.b );
const vec4 UnpackFactors4 = vec4( UnpackDownscale / PackFactors.rgb, 1.0 / PackFactors.a );
vec4 packDepthToRGBA( const in float v ) {
	if( v <= 0.0 )
		return vec4( 0., 0., 0., 0. );
	if( v >= 1.0 )
		return vec4( 1., 1., 1., 1. );
	float vuf;
	float af = modf( v * PackFactors.a, vuf );
	float bf = modf( vuf * ShiftRight8, vuf );
	float gf = modf( vuf * ShiftRight8, vuf );
	return vec4( vuf * Inv255, gf * PackUpscale, bf * PackUpscale, af );
}
vec3 packDepthToRGB( const in float v ) {
	if( v <= 0.0 )
		return vec3( 0., 0., 0. );
	if( v >= 1.0 )
		return vec3( 1., 1., 1. );
	float vuf;
	float bf = modf( v * PackFactors.b, vuf );
	float gf = modf( vuf * ShiftRight8, vuf );
	return vec3( vuf * Inv255, gf * PackUpscale, bf );
}
vec2 packDepthToRG( const in float v ) {
	if( v <= 0.0 )
		return vec2( 0., 0. );
	if( v >= 1.0 )
		return vec2( 1., 1. );
	float vuf;
	float gf = modf( v * 256., vuf );
	return vec2( vuf * Inv255, gf );
}
float unpackRGBAToDepth( const in vec4 v ) {
	return dot( v, UnpackFactors4 );
}
float unpackRGBToDepth( const in vec3 v ) {
	return dot( v, UnpackFactors3 );
}
float unpackRGToDepth( const in vec2 v ) {
	return v.r * UnpackFactors2.r + v.g * UnpackFactors2.g;
}
vec4 pack2HalfToRGBA( const in vec2 v ) {
	vec4 r = vec4( v.x, fract( v.x * 255.0 ), v.y, fract( v.y * 255.0 ) );
	return vec4( r.x - r.y / 255.0, r.y, r.z - r.w / 255.0, r.w );
}
vec2 unpackRGBATo2Half( const in vec4 v ) {
	return vec2( v.x + ( v.y / 255.0 ), v.z + ( v.w / 255.0 ) );
}
float viewZToOrthographicDepth( const in float viewZ, const in float near, const in float far ) {
	return ( viewZ + near ) / ( near - far );
}
float orthographicDepthToViewZ( const in float depth, const in float near, const in float far ) {
	return depth * ( near - far ) - near;
}
float viewZToPerspectiveDepth( const in float viewZ, const in float near, const in float far ) {
	return ( ( near + viewZ ) * far ) / ( ( far - near ) * viewZ );
}
float perspectiveDepthToViewZ( const in float depth, const in float near, const in float far ) {
	return ( near * far ) / ( ( far - near ) * depth - far );
}`,od=`#ifdef PREMULTIPLIED_ALPHA
	gl_FragColor.rgb *= gl_FragColor.a;
#endif`,cd=`vec4 mvPosition = vec4( transformed, 1.0 );
#ifdef USE_BATCHING
	mvPosition = batchingMatrix * mvPosition;
#endif
#ifdef USE_INSTANCING
	mvPosition = instanceMatrix * mvPosition;
#endif
mvPosition = modelViewMatrix * mvPosition;
gl_Position = projectionMatrix * mvPosition;`,ld=`#ifdef DITHERING
	gl_FragColor.rgb = dithering( gl_FragColor.rgb );
#endif`,ud=`#ifdef DITHERING
	vec3 dithering( vec3 color ) {
		float grid_position = rand( gl_FragCoord.xy );
		vec3 dither_shift_RGB = vec3( 0.25 / 255.0, -0.25 / 255.0, 0.25 / 255.0 );
		dither_shift_RGB = mix( 2.0 * dither_shift_RGB, -2.0 * dither_shift_RGB, grid_position );
		return color + dither_shift_RGB;
	}
#endif`,fd=`float roughnessFactor = roughness;
#ifdef USE_ROUGHNESSMAP
	vec4 texelRoughness = texture2D( roughnessMap, vRoughnessMapUv );
	roughnessFactor *= texelRoughness.g;
#endif`,hd=`#ifdef USE_ROUGHNESSMAP
	uniform sampler2D roughnessMap;
#endif`,dd=`#if NUM_SPOT_LIGHT_COORDS > 0
	varying vec4 vSpotLightCoord[ NUM_SPOT_LIGHT_COORDS ];
#endif
#if NUM_SPOT_LIGHT_MAPS > 0
	uniform sampler2D spotLightMap[ NUM_SPOT_LIGHT_MAPS ];
#endif
#ifdef USE_SHADOWMAP
	#if NUM_DIR_LIGHT_SHADOWS > 0
		uniform sampler2D directionalShadowMap[ NUM_DIR_LIGHT_SHADOWS ];
		varying vec4 vDirectionalShadowCoord[ NUM_DIR_LIGHT_SHADOWS ];
		struct DirectionalLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform DirectionalLightShadow directionalLightShadows[ NUM_DIR_LIGHT_SHADOWS ];
	#endif
	#if NUM_SPOT_LIGHT_SHADOWS > 0
		uniform sampler2D spotShadowMap[ NUM_SPOT_LIGHT_SHADOWS ];
		struct SpotLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform SpotLightShadow spotLightShadows[ NUM_SPOT_LIGHT_SHADOWS ];
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
		uniform sampler2D pointShadowMap[ NUM_POINT_LIGHT_SHADOWS ];
		varying vec4 vPointShadowCoord[ NUM_POINT_LIGHT_SHADOWS ];
		struct PointLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
			float shadowCameraNear;
			float shadowCameraFar;
		};
		uniform PointLightShadow pointLightShadows[ NUM_POINT_LIGHT_SHADOWS ];
	#endif
	float texture2DCompare( sampler2D depths, vec2 uv, float compare ) {
		return step( compare, unpackRGBAToDepth( texture2D( depths, uv ) ) );
	}
	vec2 texture2DDistribution( sampler2D shadow, vec2 uv ) {
		return unpackRGBATo2Half( texture2D( shadow, uv ) );
	}
	float VSMShadow (sampler2D shadow, vec2 uv, float compare ){
		float occlusion = 1.0;
		vec2 distribution = texture2DDistribution( shadow, uv );
		float hard_shadow = step( compare , distribution.x );
		if (hard_shadow != 1.0 ) {
			float distance = compare - distribution.x ;
			float variance = max( 0.00000, distribution.y * distribution.y );
			float softness_probability = variance / (variance + distance * distance );			softness_probability = clamp( ( softness_probability - 0.3 ) / ( 0.95 - 0.3 ), 0.0, 1.0 );			occlusion = clamp( max( hard_shadow, softness_probability ), 0.0, 1.0 );
		}
		return occlusion;
	}
	float getShadow( sampler2D shadowMap, vec2 shadowMapSize, float shadowIntensity, float shadowBias, float shadowRadius, vec4 shadowCoord ) {
		float shadow = 1.0;
		shadowCoord.xyz /= shadowCoord.w;
		shadowCoord.z += shadowBias;
		bool inFrustum = shadowCoord.x >= 0.0 && shadowCoord.x <= 1.0 && shadowCoord.y >= 0.0 && shadowCoord.y <= 1.0;
		bool frustumTest = inFrustum && shadowCoord.z <= 1.0;
		if ( frustumTest ) {
		#if defined( SHADOWMAP_TYPE_PCF )
			vec2 texelSize = vec2( 1.0 ) / shadowMapSize;
			float dx0 = - texelSize.x * shadowRadius;
			float dy0 = - texelSize.y * shadowRadius;
			float dx1 = + texelSize.x * shadowRadius;
			float dy1 = + texelSize.y * shadowRadius;
			float dx2 = dx0 / 2.0;
			float dy2 = dy0 / 2.0;
			float dx3 = dx1 / 2.0;
			float dy3 = dy1 / 2.0;
			shadow = (
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx0, dy0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( 0.0, dy0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx1, dy0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx2, dy2 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( 0.0, dy2 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx3, dy2 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx0, 0.0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx2, 0.0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy, shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx3, 0.0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx1, 0.0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx2, dy3 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( 0.0, dy3 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx3, dy3 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx0, dy1 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( 0.0, dy1 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx1, dy1 ), shadowCoord.z )
			) * ( 1.0 / 17.0 );
		#elif defined( SHADOWMAP_TYPE_PCF_SOFT )
			vec2 texelSize = vec2( 1.0 ) / shadowMapSize;
			float dx = texelSize.x;
			float dy = texelSize.y;
			vec2 uv = shadowCoord.xy;
			vec2 f = fract( uv * shadowMapSize + 0.5 );
			uv -= f * texelSize;
			shadow = (
				texture2DCompare( shadowMap, uv, shadowCoord.z ) +
				texture2DCompare( shadowMap, uv + vec2( dx, 0.0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, uv + vec2( 0.0, dy ), shadowCoord.z ) +
				texture2DCompare( shadowMap, uv + texelSize, shadowCoord.z ) +
				mix( texture2DCompare( shadowMap, uv + vec2( -dx, 0.0 ), shadowCoord.z ),
					 texture2DCompare( shadowMap, uv + vec2( 2.0 * dx, 0.0 ), shadowCoord.z ),
					 f.x ) +
				mix( texture2DCompare( shadowMap, uv + vec2( -dx, dy ), shadowCoord.z ),
					 texture2DCompare( shadowMap, uv + vec2( 2.0 * dx, dy ), shadowCoord.z ),
					 f.x ) +
				mix( texture2DCompare( shadowMap, uv + vec2( 0.0, -dy ), shadowCoord.z ),
					 texture2DCompare( shadowMap, uv + vec2( 0.0, 2.0 * dy ), shadowCoord.z ),
					 f.y ) +
				mix( texture2DCompare( shadowMap, uv + vec2( dx, -dy ), shadowCoord.z ),
					 texture2DCompare( shadowMap, uv + vec2( dx, 2.0 * dy ), shadowCoord.z ),
					 f.y ) +
				mix( mix( texture2DCompare( shadowMap, uv + vec2( -dx, -dy ), shadowCoord.z ),
						  texture2DCompare( shadowMap, uv + vec2( 2.0 * dx, -dy ), shadowCoord.z ),
						  f.x ),
					 mix( texture2DCompare( shadowMap, uv + vec2( -dx, 2.0 * dy ), shadowCoord.z ),
						  texture2DCompare( shadowMap, uv + vec2( 2.0 * dx, 2.0 * dy ), shadowCoord.z ),
						  f.x ),
					 f.y )
			) * ( 1.0 / 9.0 );
		#elif defined( SHADOWMAP_TYPE_VSM )
			shadow = VSMShadow( shadowMap, shadowCoord.xy, shadowCoord.z );
		#else
			shadow = texture2DCompare( shadowMap, shadowCoord.xy, shadowCoord.z );
		#endif
		}
		return mix( 1.0, shadow, shadowIntensity );
	}
	vec2 cubeToUV( vec3 v, float texelSizeY ) {
		vec3 absV = abs( v );
		float scaleToCube = 1.0 / max( absV.x, max( absV.y, absV.z ) );
		absV *= scaleToCube;
		v *= scaleToCube * ( 1.0 - 2.0 * texelSizeY );
		vec2 planar = v.xy;
		float almostATexel = 1.5 * texelSizeY;
		float almostOne = 1.0 - almostATexel;
		if ( absV.z >= almostOne ) {
			if ( v.z > 0.0 )
				planar.x = 4.0 - v.x;
		} else if ( absV.x >= almostOne ) {
			float signX = sign( v.x );
			planar.x = v.z * signX + 2.0 * signX;
		} else if ( absV.y >= almostOne ) {
			float signY = sign( v.y );
			planar.x = v.x + 2.0 * signY + 2.0;
			planar.y = v.z * signY - 2.0;
		}
		return vec2( 0.125, 0.25 ) * planar + vec2( 0.375, 0.75 );
	}
	float getPointShadow( sampler2D shadowMap, vec2 shadowMapSize, float shadowIntensity, float shadowBias, float shadowRadius, vec4 shadowCoord, float shadowCameraNear, float shadowCameraFar ) {
		float shadow = 1.0;
		vec3 lightToPosition = shadowCoord.xyz;
		
		float lightToPositionLength = length( lightToPosition );
		if ( lightToPositionLength - shadowCameraFar <= 0.0 && lightToPositionLength - shadowCameraNear >= 0.0 ) {
			float dp = ( lightToPositionLength - shadowCameraNear ) / ( shadowCameraFar - shadowCameraNear );			dp += shadowBias;
			vec3 bd3D = normalize( lightToPosition );
			vec2 texelSize = vec2( 1.0 ) / ( shadowMapSize * vec2( 4.0, 2.0 ) );
			#if defined( SHADOWMAP_TYPE_PCF ) || defined( SHADOWMAP_TYPE_PCF_SOFT ) || defined( SHADOWMAP_TYPE_VSM )
				vec2 offset = vec2( - 1, 1 ) * shadowRadius * texelSize.y;
				shadow = (
					texture2DCompare( shadowMap, cubeToUV( bd3D + offset.xyy, texelSize.y ), dp ) +
					texture2DCompare( shadowMap, cubeToUV( bd3D + offset.yyy, texelSize.y ), dp ) +
					texture2DCompare( shadowMap, cubeToUV( bd3D + offset.xyx, texelSize.y ), dp ) +
					texture2DCompare( shadowMap, cubeToUV( bd3D + offset.yyx, texelSize.y ), dp ) +
					texture2DCompare( shadowMap, cubeToUV( bd3D, texelSize.y ), dp ) +
					texture2DCompare( shadowMap, cubeToUV( bd3D + offset.xxy, texelSize.y ), dp ) +
					texture2DCompare( shadowMap, cubeToUV( bd3D + offset.yxy, texelSize.y ), dp ) +
					texture2DCompare( shadowMap, cubeToUV( bd3D + offset.xxx, texelSize.y ), dp ) +
					texture2DCompare( shadowMap, cubeToUV( bd3D + offset.yxx, texelSize.y ), dp )
				) * ( 1.0 / 9.0 );
			#else
				shadow = texture2DCompare( shadowMap, cubeToUV( bd3D, texelSize.y ), dp );
			#endif
		}
		return mix( 1.0, shadow, shadowIntensity );
	}
#endif`,pd=`#if NUM_SPOT_LIGHT_COORDS > 0
	uniform mat4 spotLightMatrix[ NUM_SPOT_LIGHT_COORDS ];
	varying vec4 vSpotLightCoord[ NUM_SPOT_LIGHT_COORDS ];
#endif
#ifdef USE_SHADOWMAP
	#if NUM_DIR_LIGHT_SHADOWS > 0
		uniform mat4 directionalShadowMatrix[ NUM_DIR_LIGHT_SHADOWS ];
		varying vec4 vDirectionalShadowCoord[ NUM_DIR_LIGHT_SHADOWS ];
		struct DirectionalLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform DirectionalLightShadow directionalLightShadows[ NUM_DIR_LIGHT_SHADOWS ];
	#endif
	#if NUM_SPOT_LIGHT_SHADOWS > 0
		struct SpotLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform SpotLightShadow spotLightShadows[ NUM_SPOT_LIGHT_SHADOWS ];
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
		uniform mat4 pointShadowMatrix[ NUM_POINT_LIGHT_SHADOWS ];
		varying vec4 vPointShadowCoord[ NUM_POINT_LIGHT_SHADOWS ];
		struct PointLightShadow {
			float shadowIntensity;
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
			float shadowCameraNear;
			float shadowCameraFar;
		};
		uniform PointLightShadow pointLightShadows[ NUM_POINT_LIGHT_SHADOWS ];
	#endif
#endif`,md=`#if ( defined( USE_SHADOWMAP ) && ( NUM_DIR_LIGHT_SHADOWS > 0 || NUM_POINT_LIGHT_SHADOWS > 0 ) ) || ( NUM_SPOT_LIGHT_COORDS > 0 )
	vec3 shadowWorldNormal = inverseTransformDirection( transformedNormal, viewMatrix );
	vec4 shadowWorldPosition;
#endif
#if defined( USE_SHADOWMAP )
	#if NUM_DIR_LIGHT_SHADOWS > 0
		#pragma unroll_loop_start
		for ( int i = 0; i < NUM_DIR_LIGHT_SHADOWS; i ++ ) {
			shadowWorldPosition = worldPosition + vec4( shadowWorldNormal * directionalLightShadows[ i ].shadowNormalBias, 0 );
			vDirectionalShadowCoord[ i ] = directionalShadowMatrix[ i ] * shadowWorldPosition;
		}
		#pragma unroll_loop_end
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
		#pragma unroll_loop_start
		for ( int i = 0; i < NUM_POINT_LIGHT_SHADOWS; i ++ ) {
			shadowWorldPosition = worldPosition + vec4( shadowWorldNormal * pointLightShadows[ i ].shadowNormalBias, 0 );
			vPointShadowCoord[ i ] = pointShadowMatrix[ i ] * shadowWorldPosition;
		}
		#pragma unroll_loop_end
	#endif
#endif
#if NUM_SPOT_LIGHT_COORDS > 0
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_SPOT_LIGHT_COORDS; i ++ ) {
		shadowWorldPosition = worldPosition;
		#if ( defined( USE_SHADOWMAP ) && UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS )
			shadowWorldPosition.xyz += shadowWorldNormal * spotLightShadows[ i ].shadowNormalBias;
		#endif
		vSpotLightCoord[ i ] = spotLightMatrix[ i ] * shadowWorldPosition;
	}
	#pragma unroll_loop_end
#endif`,_d=`float getShadowMask() {
	float shadow = 1.0;
	#ifdef USE_SHADOWMAP
	#if NUM_DIR_LIGHT_SHADOWS > 0
	DirectionalLightShadow directionalLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_DIR_LIGHT_SHADOWS; i ++ ) {
		directionalLight = directionalLightShadows[ i ];
		shadow *= receiveShadow ? getShadow( directionalShadowMap[ i ], directionalLight.shadowMapSize, directionalLight.shadowIntensity, directionalLight.shadowBias, directionalLight.shadowRadius, vDirectionalShadowCoord[ i ] ) : 1.0;
	}
	#pragma unroll_loop_end
	#endif
	#if NUM_SPOT_LIGHT_SHADOWS > 0
	SpotLightShadow spotLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_SPOT_LIGHT_SHADOWS; i ++ ) {
		spotLight = spotLightShadows[ i ];
		shadow *= receiveShadow ? getShadow( spotShadowMap[ i ], spotLight.shadowMapSize, spotLight.shadowIntensity, spotLight.shadowBias, spotLight.shadowRadius, vSpotLightCoord[ i ] ) : 1.0;
	}
	#pragma unroll_loop_end
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
	PointLightShadow pointLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_POINT_LIGHT_SHADOWS; i ++ ) {
		pointLight = pointLightShadows[ i ];
		shadow *= receiveShadow ? getPointShadow( pointShadowMap[ i ], pointLight.shadowMapSize, pointLight.shadowIntensity, pointLight.shadowBias, pointLight.shadowRadius, vPointShadowCoord[ i ], pointLight.shadowCameraNear, pointLight.shadowCameraFar ) : 1.0;
	}
	#pragma unroll_loop_end
	#endif
	#endif
	return shadow;
}`,gd=`#ifdef USE_SKINNING
	mat4 boneMatX = getBoneMatrix( skinIndex.x );
	mat4 boneMatY = getBoneMatrix( skinIndex.y );
	mat4 boneMatZ = getBoneMatrix( skinIndex.z );
	mat4 boneMatW = getBoneMatrix( skinIndex.w );
#endif`,xd=`#ifdef USE_SKINNING
	uniform mat4 bindMatrix;
	uniform mat4 bindMatrixInverse;
	uniform highp sampler2D boneTexture;
	mat4 getBoneMatrix( const in float i ) {
		int size = textureSize( boneTexture, 0 ).x;
		int j = int( i ) * 4;
		int x = j % size;
		int y = j / size;
		vec4 v1 = texelFetch( boneTexture, ivec2( x, y ), 0 );
		vec4 v2 = texelFetch( boneTexture, ivec2( x + 1, y ), 0 );
		vec4 v3 = texelFetch( boneTexture, ivec2( x + 2, y ), 0 );
		vec4 v4 = texelFetch( boneTexture, ivec2( x + 3, y ), 0 );
		return mat4( v1, v2, v3, v4 );
	}
#endif`,vd=`#ifdef USE_SKINNING
	vec4 skinVertex = bindMatrix * vec4( transformed, 1.0 );
	vec4 skinned = vec4( 0.0 );
	skinned += boneMatX * skinVertex * skinWeight.x;
	skinned += boneMatY * skinVertex * skinWeight.y;
	skinned += boneMatZ * skinVertex * skinWeight.z;
	skinned += boneMatW * skinVertex * skinWeight.w;
	transformed = ( bindMatrixInverse * skinned ).xyz;
#endif`,yd=`#ifdef USE_SKINNING
	mat4 skinMatrix = mat4( 0.0 );
	skinMatrix += skinWeight.x * boneMatX;
	skinMatrix += skinWeight.y * boneMatY;
	skinMatrix += skinWeight.z * boneMatZ;
	skinMatrix += skinWeight.w * boneMatW;
	skinMatrix = bindMatrixInverse * skinMatrix * bindMatrix;
	objectNormal = vec4( skinMatrix * vec4( objectNormal, 0.0 ) ).xyz;
	#ifdef USE_TANGENT
		objectTangent = vec4( skinMatrix * vec4( objectTangent, 0.0 ) ).xyz;
	#endif
#endif`,Sd=`float specularStrength;
#ifdef USE_SPECULARMAP
	vec4 texelSpecular = texture2D( specularMap, vSpecularMapUv );
	specularStrength = texelSpecular.r;
#else
	specularStrength = 1.0;
#endif`,Md=`#ifdef USE_SPECULARMAP
	uniform sampler2D specularMap;
#endif`,Ed=`#if defined( TONE_MAPPING )
	gl_FragColor.rgb = toneMapping( gl_FragColor.rgb );
#endif`,Td=`#ifndef saturate
#define saturate( a ) clamp( a, 0.0, 1.0 )
#endif
uniform float toneMappingExposure;
vec3 LinearToneMapping( vec3 color ) {
	return saturate( toneMappingExposure * color );
}
vec3 ReinhardToneMapping( vec3 color ) {
	color *= toneMappingExposure;
	return saturate( color / ( vec3( 1.0 ) + color ) );
}
vec3 CineonToneMapping( vec3 color ) {
	color *= toneMappingExposure;
	color = max( vec3( 0.0 ), color - 0.004 );
	return pow( ( color * ( 6.2 * color + 0.5 ) ) / ( color * ( 6.2 * color + 1.7 ) + 0.06 ), vec3( 2.2 ) );
}
vec3 RRTAndODTFit( vec3 v ) {
	vec3 a = v * ( v + 0.0245786 ) - 0.000090537;
	vec3 b = v * ( 0.983729 * v + 0.4329510 ) + 0.238081;
	return a / b;
}
vec3 ACESFilmicToneMapping( vec3 color ) {
	const mat3 ACESInputMat = mat3(
		vec3( 0.59719, 0.07600, 0.02840 ),		vec3( 0.35458, 0.90834, 0.13383 ),
		vec3( 0.04823, 0.01566, 0.83777 )
	);
	const mat3 ACESOutputMat = mat3(
		vec3(  1.60475, -0.10208, -0.00327 ),		vec3( -0.53108,  1.10813, -0.07276 ),
		vec3( -0.07367, -0.00605,  1.07602 )
	);
	color *= toneMappingExposure / 0.6;
	color = ACESInputMat * color;
	color = RRTAndODTFit( color );
	color = ACESOutputMat * color;
	return saturate( color );
}
const mat3 LINEAR_REC2020_TO_LINEAR_SRGB = mat3(
	vec3( 1.6605, - 0.1246, - 0.0182 ),
	vec3( - 0.5876, 1.1329, - 0.1006 ),
	vec3( - 0.0728, - 0.0083, 1.1187 )
);
const mat3 LINEAR_SRGB_TO_LINEAR_REC2020 = mat3(
	vec3( 0.6274, 0.0691, 0.0164 ),
	vec3( 0.3293, 0.9195, 0.0880 ),
	vec3( 0.0433, 0.0113, 0.8956 )
);
vec3 agxDefaultContrastApprox( vec3 x ) {
	vec3 x2 = x * x;
	vec3 x4 = x2 * x2;
	return + 15.5 * x4 * x2
		- 40.14 * x4 * x
		+ 31.96 * x4
		- 6.868 * x2 * x
		+ 0.4298 * x2
		+ 0.1191 * x
		- 0.00232;
}
vec3 AgXToneMapping( vec3 color ) {
	const mat3 AgXInsetMatrix = mat3(
		vec3( 0.856627153315983, 0.137318972929847, 0.11189821299995 ),
		vec3( 0.0951212405381588, 0.761241990602591, 0.0767994186031903 ),
		vec3( 0.0482516061458583, 0.101439036467562, 0.811302368396859 )
	);
	const mat3 AgXOutsetMatrix = mat3(
		vec3( 1.1271005818144368, - 0.1413297634984383, - 0.14132976349843826 ),
		vec3( - 0.11060664309660323, 1.157823702216272, - 0.11060664309660294 ),
		vec3( - 0.016493938717834573, - 0.016493938717834257, 1.2519364065950405 )
	);
	const float AgxMinEv = - 12.47393;	const float AgxMaxEv = 4.026069;
	color *= toneMappingExposure;
	color = LINEAR_SRGB_TO_LINEAR_REC2020 * color;
	color = AgXInsetMatrix * color;
	color = max( color, 1e-10 );	color = log2( color );
	color = ( color - AgxMinEv ) / ( AgxMaxEv - AgxMinEv );
	color = clamp( color, 0.0, 1.0 );
	color = agxDefaultContrastApprox( color );
	color = AgXOutsetMatrix * color;
	color = pow( max( vec3( 0.0 ), color ), vec3( 2.2 ) );
	color = LINEAR_REC2020_TO_LINEAR_SRGB * color;
	color = clamp( color, 0.0, 1.0 );
	return color;
}
vec3 NeutralToneMapping( vec3 color ) {
	const float StartCompression = 0.8 - 0.04;
	const float Desaturation = 0.15;
	color *= toneMappingExposure;
	float x = min( color.r, min( color.g, color.b ) );
	float offset = x < 0.08 ? x - 6.25 * x * x : 0.04;
	color -= offset;
	float peak = max( color.r, max( color.g, color.b ) );
	if ( peak < StartCompression ) return color;
	float d = 1. - StartCompression;
	float newPeak = 1. - d * d / ( peak + d - StartCompression );
	color *= newPeak / peak;
	float g = 1. - 1. / ( Desaturation * ( peak - newPeak ) + 1. );
	return mix( color, vec3( newPeak ), g );
}
vec3 CustomToneMapping( vec3 color ) { return color; }`,wd=`#ifdef USE_TRANSMISSION
	material.transmission = transmission;
	material.transmissionAlpha = 1.0;
	material.thickness = thickness;
	material.attenuationDistance = attenuationDistance;
	material.attenuationColor = attenuationColor;
	#ifdef USE_TRANSMISSIONMAP
		material.transmission *= texture2D( transmissionMap, vTransmissionMapUv ).r;
	#endif
	#ifdef USE_THICKNESSMAP
		material.thickness *= texture2D( thicknessMap, vThicknessMapUv ).g;
	#endif
	vec3 pos = vWorldPosition;
	vec3 v = normalize( cameraPosition - pos );
	vec3 n = inverseTransformDirection( normal, viewMatrix );
	vec4 transmitted = getIBLVolumeRefraction(
		n, v, material.roughness, material.diffuseColor, material.specularColor, material.specularF90,
		pos, modelMatrix, viewMatrix, projectionMatrix, material.dispersion, material.ior, material.thickness,
		material.attenuationColor, material.attenuationDistance );
	material.transmissionAlpha = mix( material.transmissionAlpha, transmitted.a, material.transmission );
	totalDiffuse = mix( totalDiffuse, transmitted.rgb, material.transmission );
#endif`,Ad=`#ifdef USE_TRANSMISSION
	uniform float transmission;
	uniform float thickness;
	uniform float attenuationDistance;
	uniform vec3 attenuationColor;
	#ifdef USE_TRANSMISSIONMAP
		uniform sampler2D transmissionMap;
	#endif
	#ifdef USE_THICKNESSMAP
		uniform sampler2D thicknessMap;
	#endif
	uniform vec2 transmissionSamplerSize;
	uniform sampler2D transmissionSamplerMap;
	uniform mat4 modelMatrix;
	uniform mat4 projectionMatrix;
	varying vec3 vWorldPosition;
	float w0( float a ) {
		return ( 1.0 / 6.0 ) * ( a * ( a * ( - a + 3.0 ) - 3.0 ) + 1.0 );
	}
	float w1( float a ) {
		return ( 1.0 / 6.0 ) * ( a *  a * ( 3.0 * a - 6.0 ) + 4.0 );
	}
	float w2( float a ){
		return ( 1.0 / 6.0 ) * ( a * ( a * ( - 3.0 * a + 3.0 ) + 3.0 ) + 1.0 );
	}
	float w3( float a ) {
		return ( 1.0 / 6.0 ) * ( a * a * a );
	}
	float g0( float a ) {
		return w0( a ) + w1( a );
	}
	float g1( float a ) {
		return w2( a ) + w3( a );
	}
	float h0( float a ) {
		return - 1.0 + w1( a ) / ( w0( a ) + w1( a ) );
	}
	float h1( float a ) {
		return 1.0 + w3( a ) / ( w2( a ) + w3( a ) );
	}
	vec4 bicubic( sampler2D tex, vec2 uv, vec4 texelSize, float lod ) {
		uv = uv * texelSize.zw + 0.5;
		vec2 iuv = floor( uv );
		vec2 fuv = fract( uv );
		float g0x = g0( fuv.x );
		float g1x = g1( fuv.x );
		float h0x = h0( fuv.x );
		float h1x = h1( fuv.x );
		float h0y = h0( fuv.y );
		float h1y = h1( fuv.y );
		vec2 p0 = ( vec2( iuv.x + h0x, iuv.y + h0y ) - 0.5 ) * texelSize.xy;
		vec2 p1 = ( vec2( iuv.x + h1x, iuv.y + h0y ) - 0.5 ) * texelSize.xy;
		vec2 p2 = ( vec2( iuv.x + h0x, iuv.y + h1y ) - 0.5 ) * texelSize.xy;
		vec2 p3 = ( vec2( iuv.x + h1x, iuv.y + h1y ) - 0.5 ) * texelSize.xy;
		return g0( fuv.y ) * ( g0x * textureLod( tex, p0, lod ) + g1x * textureLod( tex, p1, lod ) ) +
			g1( fuv.y ) * ( g0x * textureLod( tex, p2, lod ) + g1x * textureLod( tex, p3, lod ) );
	}
	vec4 textureBicubic( sampler2D sampler, vec2 uv, float lod ) {
		vec2 fLodSize = vec2( textureSize( sampler, int( lod ) ) );
		vec2 cLodSize = vec2( textureSize( sampler, int( lod + 1.0 ) ) );
		vec2 fLodSizeInv = 1.0 / fLodSize;
		vec2 cLodSizeInv = 1.0 / cLodSize;
		vec4 fSample = bicubic( sampler, uv, vec4( fLodSizeInv, fLodSize ), floor( lod ) );
		vec4 cSample = bicubic( sampler, uv, vec4( cLodSizeInv, cLodSize ), ceil( lod ) );
		return mix( fSample, cSample, fract( lod ) );
	}
	vec3 getVolumeTransmissionRay( const in vec3 n, const in vec3 v, const in float thickness, const in float ior, const in mat4 modelMatrix ) {
		vec3 refractionVector = refract( - v, normalize( n ), 1.0 / ior );
		vec3 modelScale;
		modelScale.x = length( vec3( modelMatrix[ 0 ].xyz ) );
		modelScale.y = length( vec3( modelMatrix[ 1 ].xyz ) );
		modelScale.z = length( vec3( modelMatrix[ 2 ].xyz ) );
		return normalize( refractionVector ) * thickness * modelScale;
	}
	float applyIorToRoughness( const in float roughness, const in float ior ) {
		return roughness * clamp( ior * 2.0 - 2.0, 0.0, 1.0 );
	}
	vec4 getTransmissionSample( const in vec2 fragCoord, const in float roughness, const in float ior ) {
		float lod = log2( transmissionSamplerSize.x ) * applyIorToRoughness( roughness, ior );
		return textureBicubic( transmissionSamplerMap, fragCoord.xy, lod );
	}
	vec3 volumeAttenuation( const in float transmissionDistance, const in vec3 attenuationColor, const in float attenuationDistance ) {
		if ( isinf( attenuationDistance ) ) {
			return vec3( 1.0 );
		} else {
			vec3 attenuationCoefficient = -log( attenuationColor ) / attenuationDistance;
			vec3 transmittance = exp( - attenuationCoefficient * transmissionDistance );			return transmittance;
		}
	}
	vec4 getIBLVolumeRefraction( const in vec3 n, const in vec3 v, const in float roughness, const in vec3 diffuseColor,
		const in vec3 specularColor, const in float specularF90, const in vec3 position, const in mat4 modelMatrix,
		const in mat4 viewMatrix, const in mat4 projMatrix, const in float dispersion, const in float ior, const in float thickness,
		const in vec3 attenuationColor, const in float attenuationDistance ) {
		vec4 transmittedLight;
		vec3 transmittance;
		#ifdef USE_DISPERSION
			float halfSpread = ( ior - 1.0 ) * 0.025 * dispersion;
			vec3 iors = vec3( ior - halfSpread, ior, ior + halfSpread );
			for ( int i = 0; i < 3; i ++ ) {
				vec3 transmissionRay = getVolumeTransmissionRay( n, v, thickness, iors[ i ], modelMatrix );
				vec3 refractedRayExit = position + transmissionRay;
				vec4 ndcPos = projMatrix * viewMatrix * vec4( refractedRayExit, 1.0 );
				vec2 refractionCoords = ndcPos.xy / ndcPos.w;
				refractionCoords += 1.0;
				refractionCoords /= 2.0;
				vec4 transmissionSample = getTransmissionSample( refractionCoords, roughness, iors[ i ] );
				transmittedLight[ i ] = transmissionSample[ i ];
				transmittedLight.a += transmissionSample.a;
				transmittance[ i ] = diffuseColor[ i ] * volumeAttenuation( length( transmissionRay ), attenuationColor, attenuationDistance )[ i ];
			}
			transmittedLight.a /= 3.0;
		#else
			vec3 transmissionRay = getVolumeTransmissionRay( n, v, thickness, ior, modelMatrix );
			vec3 refractedRayExit = position + transmissionRay;
			vec4 ndcPos = projMatrix * viewMatrix * vec4( refractedRayExit, 1.0 );
			vec2 refractionCoords = ndcPos.xy / ndcPos.w;
			refractionCoords += 1.0;
			refractionCoords /= 2.0;
			transmittedLight = getTransmissionSample( refractionCoords, roughness, ior );
			transmittance = diffuseColor * volumeAttenuation( length( transmissionRay ), attenuationColor, attenuationDistance );
		#endif
		vec3 attenuatedColor = transmittance * transmittedLight.rgb;
		vec3 F = EnvironmentBRDF( n, v, specularColor, specularF90, roughness );
		float transmittanceFactor = ( transmittance.r + transmittance.g + transmittance.b ) / 3.0;
		return vec4( ( 1.0 - F ) * attenuatedColor, 1.0 - ( 1.0 - transmittedLight.a ) * transmittanceFactor );
	}
#endif`,bd=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
	varying vec2 vUv;
#endif
#ifdef USE_MAP
	varying vec2 vMapUv;
#endif
#ifdef USE_ALPHAMAP
	varying vec2 vAlphaMapUv;
#endif
#ifdef USE_LIGHTMAP
	varying vec2 vLightMapUv;
#endif
#ifdef USE_AOMAP
	varying vec2 vAoMapUv;
#endif
#ifdef USE_BUMPMAP
	varying vec2 vBumpMapUv;
#endif
#ifdef USE_NORMALMAP
	varying vec2 vNormalMapUv;
#endif
#ifdef USE_EMISSIVEMAP
	varying vec2 vEmissiveMapUv;
#endif
#ifdef USE_METALNESSMAP
	varying vec2 vMetalnessMapUv;
#endif
#ifdef USE_ROUGHNESSMAP
	varying vec2 vRoughnessMapUv;
#endif
#ifdef USE_ANISOTROPYMAP
	varying vec2 vAnisotropyMapUv;
#endif
#ifdef USE_CLEARCOATMAP
	varying vec2 vClearcoatMapUv;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	varying vec2 vClearcoatNormalMapUv;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	varying vec2 vClearcoatRoughnessMapUv;
#endif
#ifdef USE_IRIDESCENCEMAP
	varying vec2 vIridescenceMapUv;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	varying vec2 vIridescenceThicknessMapUv;
#endif
#ifdef USE_SHEEN_COLORMAP
	varying vec2 vSheenColorMapUv;
#endif
#ifdef USE_SHEEN_ROUGHNESSMAP
	varying vec2 vSheenRoughnessMapUv;
#endif
#ifdef USE_SPECULARMAP
	varying vec2 vSpecularMapUv;
#endif
#ifdef USE_SPECULAR_COLORMAP
	varying vec2 vSpecularColorMapUv;
#endif
#ifdef USE_SPECULAR_INTENSITYMAP
	varying vec2 vSpecularIntensityMapUv;
#endif
#ifdef USE_TRANSMISSIONMAP
	uniform mat3 transmissionMapTransform;
	varying vec2 vTransmissionMapUv;
#endif
#ifdef USE_THICKNESSMAP
	uniform mat3 thicknessMapTransform;
	varying vec2 vThicknessMapUv;
#endif`,Cd=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
	varying vec2 vUv;
#endif
#ifdef USE_MAP
	uniform mat3 mapTransform;
	varying vec2 vMapUv;
#endif
#ifdef USE_ALPHAMAP
	uniform mat3 alphaMapTransform;
	varying vec2 vAlphaMapUv;
#endif
#ifdef USE_LIGHTMAP
	uniform mat3 lightMapTransform;
	varying vec2 vLightMapUv;
#endif
#ifdef USE_AOMAP
	uniform mat3 aoMapTransform;
	varying vec2 vAoMapUv;
#endif
#ifdef USE_BUMPMAP
	uniform mat3 bumpMapTransform;
	varying vec2 vBumpMapUv;
#endif
#ifdef USE_NORMALMAP
	uniform mat3 normalMapTransform;
	varying vec2 vNormalMapUv;
#endif
#ifdef USE_DISPLACEMENTMAP
	uniform mat3 displacementMapTransform;
	varying vec2 vDisplacementMapUv;
#endif
#ifdef USE_EMISSIVEMAP
	uniform mat3 emissiveMapTransform;
	varying vec2 vEmissiveMapUv;
#endif
#ifdef USE_METALNESSMAP
	uniform mat3 metalnessMapTransform;
	varying vec2 vMetalnessMapUv;
#endif
#ifdef USE_ROUGHNESSMAP
	uniform mat3 roughnessMapTransform;
	varying vec2 vRoughnessMapUv;
#endif
#ifdef USE_ANISOTROPYMAP
	uniform mat3 anisotropyMapTransform;
	varying vec2 vAnisotropyMapUv;
#endif
#ifdef USE_CLEARCOATMAP
	uniform mat3 clearcoatMapTransform;
	varying vec2 vClearcoatMapUv;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	uniform mat3 clearcoatNormalMapTransform;
	varying vec2 vClearcoatNormalMapUv;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	uniform mat3 clearcoatRoughnessMapTransform;
	varying vec2 vClearcoatRoughnessMapUv;
#endif
#ifdef USE_SHEEN_COLORMAP
	uniform mat3 sheenColorMapTransform;
	varying vec2 vSheenColorMapUv;
#endif
#ifdef USE_SHEEN_ROUGHNESSMAP
	uniform mat3 sheenRoughnessMapTransform;
	varying vec2 vSheenRoughnessMapUv;
#endif
#ifdef USE_IRIDESCENCEMAP
	uniform mat3 iridescenceMapTransform;
	varying vec2 vIridescenceMapUv;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	uniform mat3 iridescenceThicknessMapTransform;
	varying vec2 vIridescenceThicknessMapUv;
#endif
#ifdef USE_SPECULARMAP
	uniform mat3 specularMapTransform;
	varying vec2 vSpecularMapUv;
#endif
#ifdef USE_SPECULAR_COLORMAP
	uniform mat3 specularColorMapTransform;
	varying vec2 vSpecularColorMapUv;
#endif
#ifdef USE_SPECULAR_INTENSITYMAP
	uniform mat3 specularIntensityMapTransform;
	varying vec2 vSpecularIntensityMapUv;
#endif
#ifdef USE_TRANSMISSIONMAP
	uniform mat3 transmissionMapTransform;
	varying vec2 vTransmissionMapUv;
#endif
#ifdef USE_THICKNESSMAP
	uniform mat3 thicknessMapTransform;
	varying vec2 vThicknessMapUv;
#endif`,Rd=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
	vUv = vec3( uv, 1 ).xy;
#endif
#ifdef USE_MAP
	vMapUv = ( mapTransform * vec3( MAP_UV, 1 ) ).xy;
#endif
#ifdef USE_ALPHAMAP
	vAlphaMapUv = ( alphaMapTransform * vec3( ALPHAMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_LIGHTMAP
	vLightMapUv = ( lightMapTransform * vec3( LIGHTMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_AOMAP
	vAoMapUv = ( aoMapTransform * vec3( AOMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_BUMPMAP
	vBumpMapUv = ( bumpMapTransform * vec3( BUMPMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_NORMALMAP
	vNormalMapUv = ( normalMapTransform * vec3( NORMALMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_DISPLACEMENTMAP
	vDisplacementMapUv = ( displacementMapTransform * vec3( DISPLACEMENTMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_EMISSIVEMAP
	vEmissiveMapUv = ( emissiveMapTransform * vec3( EMISSIVEMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_METALNESSMAP
	vMetalnessMapUv = ( metalnessMapTransform * vec3( METALNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_ROUGHNESSMAP
	vRoughnessMapUv = ( roughnessMapTransform * vec3( ROUGHNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_ANISOTROPYMAP
	vAnisotropyMapUv = ( anisotropyMapTransform * vec3( ANISOTROPYMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_CLEARCOATMAP
	vClearcoatMapUv = ( clearcoatMapTransform * vec3( CLEARCOATMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	vClearcoatNormalMapUv = ( clearcoatNormalMapTransform * vec3( CLEARCOAT_NORMALMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	vClearcoatRoughnessMapUv = ( clearcoatRoughnessMapTransform * vec3( CLEARCOAT_ROUGHNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_IRIDESCENCEMAP
	vIridescenceMapUv = ( iridescenceMapTransform * vec3( IRIDESCENCEMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	vIridescenceThicknessMapUv = ( iridescenceThicknessMapTransform * vec3( IRIDESCENCE_THICKNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SHEEN_COLORMAP
	vSheenColorMapUv = ( sheenColorMapTransform * vec3( SHEEN_COLORMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SHEEN_ROUGHNESSMAP
	vSheenRoughnessMapUv = ( sheenRoughnessMapTransform * vec3( SHEEN_ROUGHNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SPECULARMAP
	vSpecularMapUv = ( specularMapTransform * vec3( SPECULARMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SPECULAR_COLORMAP
	vSpecularColorMapUv = ( specularColorMapTransform * vec3( SPECULAR_COLORMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SPECULAR_INTENSITYMAP
	vSpecularIntensityMapUv = ( specularIntensityMapTransform * vec3( SPECULAR_INTENSITYMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_TRANSMISSIONMAP
	vTransmissionMapUv = ( transmissionMapTransform * vec3( TRANSMISSIONMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_THICKNESSMAP
	vThicknessMapUv = ( thicknessMapTransform * vec3( THICKNESSMAP_UV, 1 ) ).xy;
#endif`,Pd=`#if defined( USE_ENVMAP ) || defined( DISTANCE ) || defined ( USE_SHADOWMAP ) || defined ( USE_TRANSMISSION ) || NUM_SPOT_LIGHT_COORDS > 0
	vec4 worldPosition = vec4( transformed, 1.0 );
	#ifdef USE_BATCHING
		worldPosition = batchingMatrix * worldPosition;
	#endif
	#ifdef USE_INSTANCING
		worldPosition = instanceMatrix * worldPosition;
	#endif
	worldPosition = modelMatrix * worldPosition;
#endif`;const Ld=`varying vec2 vUv;
uniform mat3 uvTransform;
void main() {
	vUv = ( uvTransform * vec3( uv, 1 ) ).xy;
	gl_Position = vec4( position.xy, 1.0, 1.0 );
}`,Dd=`uniform sampler2D t2D;
uniform float backgroundIntensity;
varying vec2 vUv;
void main() {
	vec4 texColor = texture2D( t2D, vUv );
	#ifdef DECODE_VIDEO_TEXTURE
		texColor = vec4( mix( pow( texColor.rgb * 0.9478672986 + vec3( 0.0521327014 ), vec3( 2.4 ) ), texColor.rgb * 0.0773993808, vec3( lessThanEqual( texColor.rgb, vec3( 0.04045 ) ) ) ), texColor.w );
	#endif
	texColor.rgb *= backgroundIntensity;
	gl_FragColor = texColor;
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,Id=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
	gl_Position.z = gl_Position.w;
}`,Ud=`#ifdef ENVMAP_TYPE_CUBE
	uniform samplerCube envMap;
#elif defined( ENVMAP_TYPE_CUBE_UV )
	uniform sampler2D envMap;
#endif
uniform float flipEnvMap;
uniform float backgroundBlurriness;
uniform float backgroundIntensity;
uniform mat3 backgroundRotation;
varying vec3 vWorldDirection;
#include <cube_uv_reflection_fragment>
void main() {
	#ifdef ENVMAP_TYPE_CUBE
		vec4 texColor = textureCube( envMap, backgroundRotation * vec3( flipEnvMap * vWorldDirection.x, vWorldDirection.yz ) );
	#elif defined( ENVMAP_TYPE_CUBE_UV )
		vec4 texColor = textureCubeUV( envMap, backgroundRotation * vWorldDirection, backgroundBlurriness );
	#else
		vec4 texColor = vec4( 0.0, 0.0, 0.0, 1.0 );
	#endif
	texColor.rgb *= backgroundIntensity;
	gl_FragColor = texColor;
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,Fd=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
	gl_Position.z = gl_Position.w;
}`,Nd=`uniform samplerCube tCube;
uniform float tFlip;
uniform float opacity;
varying vec3 vWorldDirection;
void main() {
	vec4 texColor = textureCube( tCube, vec3( tFlip * vWorldDirection.x, vWorldDirection.yz ) );
	gl_FragColor = texColor;
	gl_FragColor.a *= opacity;
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,Od=`#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
varying vec2 vHighPrecisionZW;
void main() {
	#include <uv_vertex>
	#include <batching_vertex>
	#include <skinbase_vertex>
	#include <morphinstance_vertex>
	#ifdef USE_DISPLACEMENTMAP
		#include <beginnormal_vertex>
		#include <morphnormal_vertex>
		#include <skinnormal_vertex>
	#endif
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vHighPrecisionZW = gl_Position.zw;
}`,Bd=`#if DEPTH_PACKING == 3200
	uniform float opacity;
#endif
#include <common>
#include <packing>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
varying vec2 vHighPrecisionZW;
void main() {
	vec4 diffuseColor = vec4( 1.0 );
	#include <clipping_planes_fragment>
	#if DEPTH_PACKING == 3200
		diffuseColor.a = opacity;
	#endif
	#include <map_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <logdepthbuf_fragment>
	float fragCoordZ = 0.5 * vHighPrecisionZW[0] / vHighPrecisionZW[1] + 0.5;
	#if DEPTH_PACKING == 3200
		gl_FragColor = vec4( vec3( 1.0 - fragCoordZ ), opacity );
	#elif DEPTH_PACKING == 3201
		gl_FragColor = packDepthToRGBA( fragCoordZ );
	#elif DEPTH_PACKING == 3202
		gl_FragColor = vec4( packDepthToRGB( fragCoordZ ), 1.0 );
	#elif DEPTH_PACKING == 3203
		gl_FragColor = vec4( packDepthToRG( fragCoordZ ), 0.0, 1.0 );
	#endif
}`,zd=`#define DISTANCE
varying vec3 vWorldPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <batching_vertex>
	#include <skinbase_vertex>
	#include <morphinstance_vertex>
	#ifdef USE_DISPLACEMENTMAP
		#include <beginnormal_vertex>
		#include <morphnormal_vertex>
		#include <skinnormal_vertex>
	#endif
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <worldpos_vertex>
	#include <clipping_planes_vertex>
	vWorldPosition = worldPosition.xyz;
}`,kd=`#define DISTANCE
uniform vec3 referencePosition;
uniform float nearDistance;
uniform float farDistance;
varying vec3 vWorldPosition;
#include <common>
#include <packing>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <clipping_planes_pars_fragment>
void main () {
	vec4 diffuseColor = vec4( 1.0 );
	#include <clipping_planes_fragment>
	#include <map_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	float dist = length( vWorldPosition - referencePosition );
	dist = ( dist - nearDistance ) / ( farDistance - nearDistance );
	dist = saturate( dist );
	gl_FragColor = packDepthToRGBA( dist );
}`,Hd=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
}`,Vd=`uniform sampler2D tEquirect;
varying vec3 vWorldDirection;
#include <common>
void main() {
	vec3 direction = normalize( vWorldDirection );
	vec2 sampleUV = equirectUv( direction );
	gl_FragColor = texture2D( tEquirect, sampleUV );
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,Gd=`uniform float scale;
attribute float lineDistance;
varying float vLineDistance;
#include <common>
#include <uv_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	vLineDistance = scale * lineDistance;
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <fog_vertex>
}`,Wd=`uniform vec3 diffuse;
uniform float opacity;
uniform float dashSize;
uniform float totalSize;
varying float vLineDistance;
#include <common>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <fog_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	if ( mod( vLineDistance, totalSize ) > dashSize ) {
		discard;
	}
	vec3 outgoingLight = vec3( 0.0 );
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	outgoingLight = diffuseColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
}`,Xd=`#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <envmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#if defined ( USE_ENVMAP ) || defined ( USE_SKINNING )
		#include <beginnormal_vertex>
		#include <morphnormal_vertex>
		#include <skinbase_vertex>
		#include <skinnormal_vertex>
		#include <defaultnormal_vertex>
	#endif
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <worldpos_vertex>
	#include <envmap_vertex>
	#include <fog_vertex>
}`,qd=`uniform vec3 diffuse;
uniform float opacity;
#ifndef FLAT_SHADED
	varying vec3 vNormal;
#endif
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_pars_fragment>
#include <fog_pars_fragment>
#include <specularmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <specularmap_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	#ifdef USE_LIGHTMAP
		vec4 lightMapTexel = texture2D( lightMap, vLightMapUv );
		reflectedLight.indirectDiffuse += lightMapTexel.rgb * lightMapIntensity * RECIPROCAL_PI;
	#else
		reflectedLight.indirectDiffuse += vec3( 1.0 );
	#endif
	#include <aomap_fragment>
	reflectedLight.indirectDiffuse *= diffuseColor.rgb;
	vec3 outgoingLight = reflectedLight.indirectDiffuse;
	#include <envmap_fragment>
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,$d=`#define LAMBERT
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <envmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <envmap_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,Yd=`#define LAMBERT
uniform vec3 diffuse;
uniform vec3 emissive;
uniform float opacity;
#include <common>
#include <packing>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_pars_fragment>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_lambert_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <specularmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <specularmap_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_lambert_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + totalEmissiveRadiance;
	#include <envmap_fragment>
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,jd=`#define MATCAP
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <color_pars_vertex>
#include <displacementmap_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <fog_vertex>
	vViewPosition = - mvPosition.xyz;
}`,Kd=`#define MATCAP
uniform vec3 diffuse;
uniform float opacity;
uniform sampler2D matcap;
varying vec3 vViewPosition;
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <fog_pars_fragment>
#include <normal_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	vec3 viewDir = normalize( vViewPosition );
	vec3 x = normalize( vec3( viewDir.z, 0.0, - viewDir.x ) );
	vec3 y = cross( viewDir, x );
	vec2 uv = vec2( dot( x, normal ), dot( y, normal ) ) * 0.495 + 0.5;
	#ifdef USE_MATCAP
		vec4 matcapColor = texture2D( matcap, uv );
	#else
		vec4 matcapColor = vec4( vec3( mix( 0.2, 0.8, uv.y ) ), 1.0 );
	#endif
	vec3 outgoingLight = diffuseColor.rgb * matcapColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,Zd=`#define NORMAL
#if defined( FLAT_SHADED ) || defined( USE_BUMPMAP ) || defined( USE_NORMALMAP_TANGENTSPACE )
	varying vec3 vViewPosition;
#endif
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphinstance_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
#if defined( FLAT_SHADED ) || defined( USE_BUMPMAP ) || defined( USE_NORMALMAP_TANGENTSPACE )
	vViewPosition = - mvPosition.xyz;
#endif
}`,Jd=`#define NORMAL
uniform float opacity;
#if defined( FLAT_SHADED ) || defined( USE_BUMPMAP ) || defined( USE_NORMALMAP_TANGENTSPACE )
	varying vec3 vViewPosition;
#endif
#include <packing>
#include <uv_pars_fragment>
#include <normal_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( 0.0, 0.0, 0.0, opacity );
	#include <clipping_planes_fragment>
	#include <logdepthbuf_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	gl_FragColor = vec4( packNormalToRGB( normal ), diffuseColor.a );
	#ifdef OPAQUE
		gl_FragColor.a = 1.0;
	#endif
}`,Qd=`#define PHONG
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <envmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphinstance_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <envmap_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,tp=`#define PHONG
uniform vec3 diffuse;
uniform vec3 emissive;
uniform vec3 specular;
uniform float shininess;
uniform float opacity;
#include <common>
#include <packing>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_pars_fragment>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_phong_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <specularmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <specularmap_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_phong_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + reflectedLight.directSpecular + reflectedLight.indirectSpecular + totalEmissiveRadiance;
	#include <envmap_fragment>
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,ep=`#define STANDARD
varying vec3 vViewPosition;
#ifdef USE_TRANSMISSION
	varying vec3 vWorldPosition;
#endif
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
#ifdef USE_TRANSMISSION
	vWorldPosition = worldPosition.xyz;
#endif
}`,np=`#define STANDARD
#ifdef PHYSICAL
	#define IOR
	#define USE_SPECULAR
#endif
uniform vec3 diffuse;
uniform vec3 emissive;
uniform float roughness;
uniform float metalness;
uniform float opacity;
#ifdef IOR
	uniform float ior;
#endif
#ifdef USE_SPECULAR
	uniform float specularIntensity;
	uniform vec3 specularColor;
	#ifdef USE_SPECULAR_COLORMAP
		uniform sampler2D specularColorMap;
	#endif
	#ifdef USE_SPECULAR_INTENSITYMAP
		uniform sampler2D specularIntensityMap;
	#endif
#endif
#ifdef USE_CLEARCOAT
	uniform float clearcoat;
	uniform float clearcoatRoughness;
#endif
#ifdef USE_DISPERSION
	uniform float dispersion;
#endif
#ifdef USE_IRIDESCENCE
	uniform float iridescence;
	uniform float iridescenceIOR;
	uniform float iridescenceThicknessMinimum;
	uniform float iridescenceThicknessMaximum;
#endif
#ifdef USE_SHEEN
	uniform vec3 sheenColor;
	uniform float sheenRoughness;
	#ifdef USE_SHEEN_COLORMAP
		uniform sampler2D sheenColorMap;
	#endif
	#ifdef USE_SHEEN_ROUGHNESSMAP
		uniform sampler2D sheenRoughnessMap;
	#endif
#endif
#ifdef USE_ANISOTROPY
	uniform vec2 anisotropyVector;
	#ifdef USE_ANISOTROPYMAP
		uniform sampler2D anisotropyMap;
	#endif
#endif
varying vec3 vViewPosition;
#include <common>
#include <packing>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <iridescence_fragment>
#include <cube_uv_reflection_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_physical_pars_fragment>
#include <fog_pars_fragment>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_physical_pars_fragment>
#include <transmission_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <clearcoat_pars_fragment>
#include <iridescence_pars_fragment>
#include <roughnessmap_pars_fragment>
#include <metalnessmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <roughnessmap_fragment>
	#include <metalnessmap_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <clearcoat_normal_fragment_begin>
	#include <clearcoat_normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_physical_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 totalDiffuse = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse;
	vec3 totalSpecular = reflectedLight.directSpecular + reflectedLight.indirectSpecular;
	#include <transmission_fragment>
	vec3 outgoingLight = totalDiffuse + totalSpecular + totalEmissiveRadiance;
	#ifdef USE_SHEEN
		float sheenEnergyComp = 1.0 - 0.157 * max3( material.sheenColor );
		outgoingLight = outgoingLight * sheenEnergyComp + sheenSpecularDirect + sheenSpecularIndirect;
	#endif
	#ifdef USE_CLEARCOAT
		float dotNVcc = saturate( dot( geometryClearcoatNormal, geometryViewDir ) );
		vec3 Fcc = F_Schlick( material.clearcoatF0, material.clearcoatF90, dotNVcc );
		outgoingLight = outgoingLight * ( 1.0 - material.clearcoat * Fcc ) + ( clearcoatSpecularDirect + clearcoatSpecularIndirect ) * material.clearcoat;
	#endif
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,ip=`#define TOON
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,rp=`#define TOON
uniform vec3 diffuse;
uniform vec3 emissive;
uniform float opacity;
#include <common>
#include <packing>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <gradientmap_pars_fragment>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_toon_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_toon_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + totalEmissiveRadiance;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,sp=`uniform float size;
uniform float scale;
#include <common>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
#ifdef USE_POINTS_UV
	varying vec2 vUv;
	uniform mat3 uvTransform;
#endif
void main() {
	#ifdef USE_POINTS_UV
		vUv = ( uvTransform * vec3( uv, 1 ) ).xy;
	#endif
	#include <color_vertex>
	#include <morphinstance_vertex>
	#include <morphcolor_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <project_vertex>
	gl_PointSize = size;
	#ifdef USE_SIZEATTENUATION
		bool isPerspective = isPerspectiveMatrix( projectionMatrix );
		if ( isPerspective ) gl_PointSize *= ( scale / - mvPosition.z );
	#endif
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <worldpos_vertex>
	#include <fog_vertex>
}`,ap=`uniform vec3 diffuse;
uniform float opacity;
#include <common>
#include <color_pars_fragment>
#include <map_particle_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <fog_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	vec3 outgoingLight = vec3( 0.0 );
	#include <logdepthbuf_fragment>
	#include <map_particle_fragment>
	#include <color_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	outgoingLight = diffuseColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
}`,op=`#include <common>
#include <batching_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <shadowmap_pars_vertex>
void main() {
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphinstance_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <worldpos_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,cp=`uniform vec3 color;
uniform float opacity;
#include <common>
#include <packing>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <logdepthbuf_pars_fragment>
#include <shadowmap_pars_fragment>
#include <shadowmask_pars_fragment>
void main() {
	#include <logdepthbuf_fragment>
	gl_FragColor = vec4( color, opacity * ( 1.0 - getShadowMask() ) );
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
}`,lp=`uniform float rotation;
uniform vec2 center;
#include <common>
#include <uv_pars_vertex>
#include <fog_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	vec4 mvPosition = modelViewMatrix[ 3 ];
	vec2 scale = vec2( length( modelMatrix[ 0 ].xyz ), length( modelMatrix[ 1 ].xyz ) );
	#ifndef USE_SIZEATTENUATION
		bool isPerspective = isPerspectiveMatrix( projectionMatrix );
		if ( isPerspective ) scale *= - mvPosition.z;
	#endif
	vec2 alignedPosition = ( position.xy - ( center - vec2( 0.5 ) ) ) * scale;
	vec2 rotatedPosition;
	rotatedPosition.x = cos( rotation ) * alignedPosition.x - sin( rotation ) * alignedPosition.y;
	rotatedPosition.y = sin( rotation ) * alignedPosition.x + cos( rotation ) * alignedPosition.y;
	mvPosition.xy += rotatedPosition;
	gl_Position = projectionMatrix * mvPosition;
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <fog_vertex>
}`,up=`uniform vec3 diffuse;
uniform float opacity;
#include <common>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <fog_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <clipping_planes_fragment>
	vec3 outgoingLight = vec3( 0.0 );
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	outgoingLight = diffuseColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
}`,Nt={alphahash_fragment:Df,alphahash_pars_fragment:If,alphamap_fragment:Uf,alphamap_pars_fragment:Ff,alphatest_fragment:Nf,alphatest_pars_fragment:Of,aomap_fragment:Bf,aomap_pars_fragment:zf,batching_pars_vertex:kf,batching_vertex:Hf,begin_vertex:Vf,beginnormal_vertex:Gf,bsdfs:Wf,iridescence_fragment:Xf,bumpmap_pars_fragment:qf,clipping_planes_fragment:$f,clipping_planes_pars_fragment:Yf,clipping_planes_pars_vertex:jf,clipping_planes_vertex:Kf,color_fragment:Zf,color_pars_fragment:Jf,color_pars_vertex:Qf,color_vertex:th,common:eh,cube_uv_reflection_fragment:nh,defaultnormal_vertex:ih,displacementmap_pars_vertex:rh,displacementmap_vertex:sh,emissivemap_fragment:ah,emissivemap_pars_fragment:oh,colorspace_fragment:ch,colorspace_pars_fragment:lh,envmap_fragment:uh,envmap_common_pars_fragment:fh,envmap_pars_fragment:hh,envmap_pars_vertex:dh,envmap_physical_pars_fragment:Th,envmap_vertex:ph,fog_vertex:mh,fog_pars_vertex:_h,fog_fragment:gh,fog_pars_fragment:xh,gradientmap_pars_fragment:vh,lightmap_pars_fragment:yh,lights_lambert_fragment:Sh,lights_lambert_pars_fragment:Mh,lights_pars_begin:Eh,lights_toon_fragment:wh,lights_toon_pars_fragment:Ah,lights_phong_fragment:bh,lights_phong_pars_fragment:Ch,lights_physical_fragment:Rh,lights_physical_pars_fragment:Ph,lights_fragment_begin:Lh,lights_fragment_maps:Dh,lights_fragment_end:Ih,logdepthbuf_fragment:Uh,logdepthbuf_pars_fragment:Fh,logdepthbuf_pars_vertex:Nh,logdepthbuf_vertex:Oh,map_fragment:Bh,map_pars_fragment:zh,map_particle_fragment:kh,map_particle_pars_fragment:Hh,metalnessmap_fragment:Vh,metalnessmap_pars_fragment:Gh,morphinstance_vertex:Wh,morphcolor_vertex:Xh,morphnormal_vertex:qh,morphtarget_pars_vertex:$h,morphtarget_vertex:Yh,normal_fragment_begin:jh,normal_fragment_maps:Kh,normal_pars_fragment:Zh,normal_pars_vertex:Jh,normal_vertex:Qh,normalmap_pars_fragment:td,clearcoat_normal_fragment_begin:ed,clearcoat_normal_fragment_maps:nd,clearcoat_pars_fragment:id,iridescence_pars_fragment:rd,opaque_fragment:sd,packing:ad,premultiplied_alpha_fragment:od,project_vertex:cd,dithering_fragment:ld,dithering_pars_fragment:ud,roughnessmap_fragment:fd,roughnessmap_pars_fragment:hd,shadowmap_pars_fragment:dd,shadowmap_pars_vertex:pd,shadowmap_vertex:md,shadowmask_pars_fragment:_d,skinbase_vertex:gd,skinning_pars_vertex:xd,skinning_vertex:vd,skinnormal_vertex:yd,specularmap_fragment:Sd,specularmap_pars_fragment:Md,tonemapping_fragment:Ed,tonemapping_pars_fragment:Td,transmission_fragment:wd,transmission_pars_fragment:Ad,uv_pars_fragment:bd,uv_pars_vertex:Cd,uv_vertex:Rd,worldpos_vertex:Pd,background_vert:Ld,background_frag:Dd,backgroundCube_vert:Id,backgroundCube_frag:Ud,cube_vert:Fd,cube_frag:Nd,depth_vert:Od,depth_frag:Bd,distanceRGBA_vert:zd,distanceRGBA_frag:kd,equirect_vert:Hd,equirect_frag:Vd,linedashed_vert:Gd,linedashed_frag:Wd,meshbasic_vert:Xd,meshbasic_frag:qd,meshlambert_vert:$d,meshlambert_frag:Yd,meshmatcap_vert:jd,meshmatcap_frag:Kd,meshnormal_vert:Zd,meshnormal_frag:Jd,meshphong_vert:Qd,meshphong_frag:tp,meshphysical_vert:ep,meshphysical_frag:np,meshtoon_vert:ip,meshtoon_frag:rp,points_vert:sp,points_frag:ap,shadow_vert:op,shadow_frag:cp,sprite_vert:lp,sprite_frag:up},lt={common:{diffuse:{value:new Wt(16777215)},opacity:{value:1},map:{value:null},mapTransform:{value:new Ft},alphaMap:{value:null},alphaMapTransform:{value:new Ft},alphaTest:{value:0}},specularmap:{specularMap:{value:null},specularMapTransform:{value:new Ft}},envmap:{envMap:{value:null},envMapRotation:{value:new Ft},flipEnvMap:{value:-1},reflectivity:{value:1},ior:{value:1.5},refractionRatio:{value:.98}},aomap:{aoMap:{value:null},aoMapIntensity:{value:1},aoMapTransform:{value:new Ft}},lightmap:{lightMap:{value:null},lightMapIntensity:{value:1},lightMapTransform:{value:new Ft}},bumpmap:{bumpMap:{value:null},bumpMapTransform:{value:new Ft},bumpScale:{value:1}},normalmap:{normalMap:{value:null},normalMapTransform:{value:new Ft},normalScale:{value:new Xt(1,1)}},displacementmap:{displacementMap:{value:null},displacementMapTransform:{value:new Ft},displacementScale:{value:1},displacementBias:{value:0}},emissivemap:{emissiveMap:{value:null},emissiveMapTransform:{value:new Ft}},metalnessmap:{metalnessMap:{value:null},metalnessMapTransform:{value:new Ft}},roughnessmap:{roughnessMap:{value:null},roughnessMapTransform:{value:new Ft}},gradientmap:{gradientMap:{value:null}},fog:{fogDensity:{value:25e-5},fogNear:{value:1},fogFar:{value:2e3},fogColor:{value:new Wt(16777215)}},lights:{ambientLightColor:{value:[]},lightProbe:{value:[]},directionalLights:{value:[],properties:{direction:{},color:{}}},directionalLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{}}},directionalShadowMap:{value:[]},directionalShadowMatrix:{value:[]},spotLights:{value:[],properties:{color:{},position:{},direction:{},distance:{},coneCos:{},penumbraCos:{},decay:{}}},spotLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{}}},spotLightMap:{value:[]},spotShadowMap:{value:[]},spotLightMatrix:{value:[]},pointLights:{value:[],properties:{color:{},position:{},decay:{},distance:{}}},pointLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{},shadowCameraNear:{},shadowCameraFar:{}}},pointShadowMap:{value:[]},pointShadowMatrix:{value:[]},hemisphereLights:{value:[],properties:{direction:{},skyColor:{},groundColor:{}}},rectAreaLights:{value:[],properties:{color:{},position:{},width:{},height:{}}},ltc_1:{value:null},ltc_2:{value:null}},points:{diffuse:{value:new Wt(16777215)},opacity:{value:1},size:{value:1},scale:{value:1},map:{value:null},alphaMap:{value:null},alphaMapTransform:{value:new Ft},alphaTest:{value:0},uvTransform:{value:new Ft}},sprite:{diffuse:{value:new Wt(16777215)},opacity:{value:1},center:{value:new Xt(.5,.5)},rotation:{value:0},map:{value:null},mapTransform:{value:new Ft},alphaMap:{value:null},alphaMapTransform:{value:new Ft},alphaTest:{value:0}}},je={basic:{uniforms:Me([lt.common,lt.specularmap,lt.envmap,lt.aomap,lt.lightmap,lt.fog]),vertexShader:Nt.meshbasic_vert,fragmentShader:Nt.meshbasic_frag},lambert:{uniforms:Me([lt.common,lt.specularmap,lt.envmap,lt.aomap,lt.lightmap,lt.emissivemap,lt.bumpmap,lt.normalmap,lt.displacementmap,lt.fog,lt.lights,{emissive:{value:new Wt(0)}}]),vertexShader:Nt.meshlambert_vert,fragmentShader:Nt.meshlambert_frag},phong:{uniforms:Me([lt.common,lt.specularmap,lt.envmap,lt.aomap,lt.lightmap,lt.emissivemap,lt.bumpmap,lt.normalmap,lt.displacementmap,lt.fog,lt.lights,{emissive:{value:new Wt(0)},specular:{value:new Wt(1118481)},shininess:{value:30}}]),vertexShader:Nt.meshphong_vert,fragmentShader:Nt.meshphong_frag},standard:{uniforms:Me([lt.common,lt.envmap,lt.aomap,lt.lightmap,lt.emissivemap,lt.bumpmap,lt.normalmap,lt.displacementmap,lt.roughnessmap,lt.metalnessmap,lt.fog,lt.lights,{emissive:{value:new Wt(0)},roughness:{value:1},metalness:{value:0},envMapIntensity:{value:1}}]),vertexShader:Nt.meshphysical_vert,fragmentShader:Nt.meshphysical_frag},toon:{uniforms:Me([lt.common,lt.aomap,lt.lightmap,lt.emissivemap,lt.bumpmap,lt.normalmap,lt.displacementmap,lt.gradientmap,lt.fog,lt.lights,{emissive:{value:new Wt(0)}}]),vertexShader:Nt.meshtoon_vert,fragmentShader:Nt.meshtoon_frag},matcap:{uniforms:Me([lt.common,lt.bumpmap,lt.normalmap,lt.displacementmap,lt.fog,{matcap:{value:null}}]),vertexShader:Nt.meshmatcap_vert,fragmentShader:Nt.meshmatcap_frag},points:{uniforms:Me([lt.points,lt.fog]),vertexShader:Nt.points_vert,fragmentShader:Nt.points_frag},dashed:{uniforms:Me([lt.common,lt.fog,{scale:{value:1},dashSize:{value:1},totalSize:{value:2}}]),vertexShader:Nt.linedashed_vert,fragmentShader:Nt.linedashed_frag},depth:{uniforms:Me([lt.common,lt.displacementmap]),vertexShader:Nt.depth_vert,fragmentShader:Nt.depth_frag},normal:{uniforms:Me([lt.common,lt.bumpmap,lt.normalmap,lt.displacementmap,{opacity:{value:1}}]),vertexShader:Nt.meshnormal_vert,fragmentShader:Nt.meshnormal_frag},sprite:{uniforms:Me([lt.sprite,lt.fog]),vertexShader:Nt.sprite_vert,fragmentShader:Nt.sprite_frag},background:{uniforms:{uvTransform:{value:new Ft},t2D:{value:null},backgroundIntensity:{value:1}},vertexShader:Nt.background_vert,fragmentShader:Nt.background_frag},backgroundCube:{uniforms:{envMap:{value:null},flipEnvMap:{value:-1},backgroundBlurriness:{value:0},backgroundIntensity:{value:1},backgroundRotation:{value:new Ft}},vertexShader:Nt.backgroundCube_vert,fragmentShader:Nt.backgroundCube_frag},cube:{uniforms:{tCube:{value:null},tFlip:{value:-1},opacity:{value:1}},vertexShader:Nt.cube_vert,fragmentShader:Nt.cube_frag},equirect:{uniforms:{tEquirect:{value:null}},vertexShader:Nt.equirect_vert,fragmentShader:Nt.equirect_frag},distanceRGBA:{uniforms:Me([lt.common,lt.displacementmap,{referencePosition:{value:new H},nearDistance:{value:1},farDistance:{value:1e3}}]),vertexShader:Nt.distanceRGBA_vert,fragmentShader:Nt.distanceRGBA_frag},shadow:{uniforms:Me([lt.lights,lt.fog,{color:{value:new Wt(0)},opacity:{value:1}}]),vertexShader:Nt.shadow_vert,fragmentShader:Nt.shadow_frag}};je.physical={uniforms:Me([je.standard.uniforms,{clearcoat:{value:0},clearcoatMap:{value:null},clearcoatMapTransform:{value:new Ft},clearcoatNormalMap:{value:null},clearcoatNormalMapTransform:{value:new Ft},clearcoatNormalScale:{value:new Xt(1,1)},clearcoatRoughness:{value:0},clearcoatRoughnessMap:{value:null},clearcoatRoughnessMapTransform:{value:new Ft},dispersion:{value:0},iridescence:{value:0},iridescenceMap:{value:null},iridescenceMapTransform:{value:new Ft},iridescenceIOR:{value:1.3},iridescenceThicknessMinimum:{value:100},iridescenceThicknessMaximum:{value:400},iridescenceThicknessMap:{value:null},iridescenceThicknessMapTransform:{value:new Ft},sheen:{value:0},sheenColor:{value:new Wt(0)},sheenColorMap:{value:null},sheenColorMapTransform:{value:new Ft},sheenRoughness:{value:1},sheenRoughnessMap:{value:null},sheenRoughnessMapTransform:{value:new Ft},transmission:{value:0},transmissionMap:{value:null},transmissionMapTransform:{value:new Ft},transmissionSamplerSize:{value:new Xt},transmissionSamplerMap:{value:null},thickness:{value:0},thicknessMap:{value:null},thicknessMapTransform:{value:new Ft},attenuationDistance:{value:0},attenuationColor:{value:new Wt(0)},specularColor:{value:new Wt(1,1,1)},specularColorMap:{value:null},specularColorMapTransform:{value:new Ft},specularIntensity:{value:1},specularIntensityMap:{value:null},specularIntensityMapTransform:{value:new Ft},anisotropyVector:{value:new Xt},anisotropyMap:{value:null},anisotropyMapTransform:{value:new Ft}}]),vertexShader:Nt.meshphysical_vert,fragmentShader:Nt.meshphysical_frag};const yr={r:0,b:0,g:0},kn=new Ye,fp=new te;function hp(n,t,e,i,r,s,o){const a=new Wt(0);let l=s===!0?0:1,u,h,p=null,m=0,_=null;function x(b){let w=b.isScene===!0?b.background:null;return w&&w.isTexture&&(w=(b.backgroundBlurriness>0?e:t).get(w)),w}function y(b){let w=!1;const D=x(b);D===null?d(a,l):D&&D.isColor&&(d(D,1),w=!0);const L=n.xr.getEnvironmentBlendMode();L==="additive"?i.buffers.color.setClear(0,0,0,1,o):L==="alpha-blend"&&i.buffers.color.setClear(0,0,0,0,o),(n.autoClear||w)&&(i.buffers.depth.setTest(!0),i.buffers.depth.setMask(!0),i.buffers.color.setMask(!0),n.clear(n.autoClearColor,n.autoClearDepth,n.autoClearStencil))}function g(b,w){const D=x(w);D&&(D.isCubeTexture||D.mapping===Gr)?(h===void 0&&(h=new ce(new Kn(1,1,1),new Ln({name:"BackgroundCubeMaterial",uniforms:Mi(je.backgroundCube.uniforms),vertexShader:je.backgroundCube.vertexShader,fragmentShader:je.backgroundCube.fragmentShader,side:Ae,depthTest:!1,depthWrite:!1,fog:!1,allowOverride:!1})),h.geometry.deleteAttribute("normal"),h.geometry.deleteAttribute("uv"),h.onBeforeRender=function(L,I,N){this.matrixWorld.copyPosition(N.matrixWorld)},Object.defineProperty(h.material,"envMap",{get:function(){return this.uniforms.envMap.value}}),r.update(h)),kn.copy(w.backgroundRotation),kn.x*=-1,kn.y*=-1,kn.z*=-1,D.isCubeTexture&&D.isRenderTargetTexture===!1&&(kn.y*=-1,kn.z*=-1),h.material.uniforms.envMap.value=D,h.material.uniforms.flipEnvMap.value=D.isCubeTexture&&D.isRenderTargetTexture===!1?-1:1,h.material.uniforms.backgroundBlurriness.value=w.backgroundBlurriness,h.material.uniforms.backgroundIntensity.value=w.backgroundIntensity,h.material.uniforms.backgroundRotation.value.setFromMatrix4(fp.makeRotationFromEuler(kn)),h.material.toneMapped=Gt.getTransfer(D.colorSpace)!==Kt,(p!==D||m!==D.version||_!==n.toneMapping)&&(h.material.needsUpdate=!0,p=D,m=D.version,_=n.toneMapping),h.layers.enableAll(),b.unshift(h,h.geometry,h.material,0,0,null)):D&&D.isTexture&&(u===void 0&&(u=new ce(new Ki(2,2),new Ln({name:"BackgroundMaterial",uniforms:Mi(je.background.uniforms),vertexShader:je.background.vertexShader,fragmentShader:je.background.fragmentShader,side:Pn,depthTest:!1,depthWrite:!1,fog:!1,allowOverride:!1})),u.geometry.deleteAttribute("normal"),Object.defineProperty(u.material,"map",{get:function(){return this.uniforms.t2D.value}}),r.update(u)),u.material.uniforms.t2D.value=D,u.material.uniforms.backgroundIntensity.value=w.backgroundIntensity,u.material.toneMapped=Gt.getTransfer(D.colorSpace)!==Kt,D.matrixAutoUpdate===!0&&D.updateMatrix(),u.material.uniforms.uvTransform.value.copy(D.matrix),(p!==D||m!==D.version||_!==n.toneMapping)&&(u.material.needsUpdate=!0,p=D,m=D.version,_=n.toneMapping),u.layers.enableAll(),b.unshift(u,u.geometry,u.material,0,0,null))}function d(b,w){b.getRGB(yr,Qc(n)),i.buffers.color.setClear(yr.r,yr.g,yr.b,w,o)}function P(){h!==void 0&&(h.geometry.dispose(),h.material.dispose(),h=void 0),u!==void 0&&(u.geometry.dispose(),u.material.dispose(),u=void 0)}return{getClearColor:function(){return a},setClearColor:function(b,w=1){a.set(b),l=w,d(a,l)},getClearAlpha:function(){return l},setClearAlpha:function(b){l=b,d(a,l)},render:y,addToRenderList:g,dispose:P}}function dp(n,t){const e=n.getParameter(n.MAX_VERTEX_ATTRIBS),i={},r=m(null);let s=r,o=!1;function a(T,O,$,W,K){let it=!1;const Z=p(W,$,O);s!==Z&&(s=Z,u(s.object)),it=_(T,W,$,K),it&&x(T,W,$,K),K!==null&&t.update(K,n.ELEMENT_ARRAY_BUFFER),(it||o)&&(o=!1,w(T,O,$,W),K!==null&&n.bindBuffer(n.ELEMENT_ARRAY_BUFFER,t.get(K).buffer))}function l(){return n.createVertexArray()}function u(T){return n.bindVertexArray(T)}function h(T){return n.deleteVertexArray(T)}function p(T,O,$){const W=$.wireframe===!0;let K=i[T.id];K===void 0&&(K={},i[T.id]=K);let it=K[O.id];it===void 0&&(it={},K[O.id]=it);let Z=it[W];return Z===void 0&&(Z=m(l()),it[W]=Z),Z}function m(T){const O=[],$=[],W=[];for(let K=0;K<e;K++)O[K]=0,$[K]=0,W[K]=0;return{geometry:null,program:null,wireframe:!1,newAttributes:O,enabledAttributes:$,attributeDivisors:W,object:T,attributes:{},index:null}}function _(T,O,$,W){const K=s.attributes,it=O.attributes;let Z=0;const at=$.getAttributes();for(const Y in at)if(at[Y].location>=0){const gt=K[Y];let ht=it[Y];if(ht===void 0&&(Y==="instanceMatrix"&&T.instanceMatrix&&(ht=T.instanceMatrix),Y==="instanceColor"&&T.instanceColor&&(ht=T.instanceColor)),gt===void 0||gt.attribute!==ht||ht&&gt.data!==ht.data)return!0;Z++}return s.attributesNum!==Z||s.index!==W}function x(T,O,$,W){const K={},it=O.attributes;let Z=0;const at=$.getAttributes();for(const Y in at)if(at[Y].location>=0){let gt=it[Y];gt===void 0&&(Y==="instanceMatrix"&&T.instanceMatrix&&(gt=T.instanceMatrix),Y==="instanceColor"&&T.instanceColor&&(gt=T.instanceColor));const ht={};ht.attribute=gt,gt&&gt.data&&(ht.data=gt.data),K[Y]=ht,Z++}s.attributes=K,s.attributesNum=Z,s.index=W}function y(){const T=s.newAttributes;for(let O=0,$=T.length;O<$;O++)T[O]=0}function g(T){d(T,0)}function d(T,O){const $=s.newAttributes,W=s.enabledAttributes,K=s.attributeDivisors;$[T]=1,W[T]===0&&(n.enableVertexAttribArray(T),W[T]=1),K[T]!==O&&(n.vertexAttribDivisor(T,O),K[T]=O)}function P(){const T=s.newAttributes,O=s.enabledAttributes;for(let $=0,W=O.length;$<W;$++)O[$]!==T[$]&&(n.disableVertexAttribArray($),O[$]=0)}function b(T,O,$,W,K,it,Z){Z===!0?n.vertexAttribIPointer(T,O,$,K,it):n.vertexAttribPointer(T,O,$,W,K,it)}function w(T,O,$,W){y();const K=W.attributes,it=$.getAttributes(),Z=O.defaultAttributeValues;for(const at in it){const Y=it[at];if(Y.location>=0){let ft=K[at];if(ft===void 0&&(at==="instanceMatrix"&&T.instanceMatrix&&(ft=T.instanceMatrix),at==="instanceColor"&&T.instanceColor&&(ft=T.instanceColor)),ft!==void 0){const gt=ft.normalized,ht=ft.itemSize,bt=t.get(ft);if(bt===void 0)continue;const qt=bt.buffer,Q=bt.type,ct=bt.bytesPerElement,St=Q===n.INT||Q===n.UNSIGNED_INT||ft.gpuType===Da;if(ft.isInterleavedBufferAttribute){const ut=ft.data,Tt=ut.stride,Bt=ft.offset;if(ut.isInstancedInterleavedBuffer){for(let Pt=0;Pt<Y.locationSize;Pt++)d(Y.location+Pt,ut.meshPerAttribute);T.isInstancedMesh!==!0&&W._maxInstanceCount===void 0&&(W._maxInstanceCount=ut.meshPerAttribute*ut.count)}else for(let Pt=0;Pt<Y.locationSize;Pt++)g(Y.location+Pt);n.bindBuffer(n.ARRAY_BUFFER,qt);for(let Pt=0;Pt<Y.locationSize;Pt++)b(Y.location+Pt,ht/Y.locationSize,Q,gt,Tt*ct,(Bt+ht/Y.locationSize*Pt)*ct,St)}else{if(ft.isInstancedBufferAttribute){for(let ut=0;ut<Y.locationSize;ut++)d(Y.location+ut,ft.meshPerAttribute);T.isInstancedMesh!==!0&&W._maxInstanceCount===void 0&&(W._maxInstanceCount=ft.meshPerAttribute*ft.count)}else for(let ut=0;ut<Y.locationSize;ut++)g(Y.location+ut);n.bindBuffer(n.ARRAY_BUFFER,qt);for(let ut=0;ut<Y.locationSize;ut++)b(Y.location+ut,ht/Y.locationSize,Q,gt,ht*ct,ht/Y.locationSize*ut*ct,St)}}else if(Z!==void 0){const gt=Z[at];if(gt!==void 0)switch(gt.length){case 2:n.vertexAttrib2fv(Y.location,gt);break;case 3:n.vertexAttrib3fv(Y.location,gt);break;case 4:n.vertexAttrib4fv(Y.location,gt);break;default:n.vertexAttrib1fv(Y.location,gt)}}}}P()}function D(){N();for(const T in i){const O=i[T];for(const $ in O){const W=O[$];for(const K in W)h(W[K].object),delete W[K];delete O[$]}delete i[T]}}function L(T){if(i[T.id]===void 0)return;const O=i[T.id];for(const $ in O){const W=O[$];for(const K in W)h(W[K].object),delete W[K];delete O[$]}delete i[T.id]}function I(T){for(const O in i){const $=i[O];if($[T.id]===void 0)continue;const W=$[T.id];for(const K in W)h(W[K].object),delete W[K];delete $[T.id]}}function N(){A(),o=!0,s!==r&&(s=r,u(s.object))}function A(){r.geometry=null,r.program=null,r.wireframe=!1}return{setup:a,reset:N,resetDefaultState:A,dispose:D,releaseStatesOfGeometry:L,releaseStatesOfProgram:I,initAttributes:y,enableAttribute:g,disableUnusedAttributes:P}}function pp(n,t,e){let i;function r(u){i=u}function s(u,h){n.drawArrays(i,u,h),e.update(h,i,1)}function o(u,h,p){p!==0&&(n.drawArraysInstanced(i,u,h,p),e.update(h,i,p))}function a(u,h,p){if(p===0)return;t.get("WEBGL_multi_draw").multiDrawArraysWEBGL(i,u,0,h,0,p);let _=0;for(let x=0;x<p;x++)_+=h[x];e.update(_,i,1)}function l(u,h,p,m){if(p===0)return;const _=t.get("WEBGL_multi_draw");if(_===null)for(let x=0;x<u.length;x++)o(u[x],h[x],m[x]);else{_.multiDrawArraysInstancedWEBGL(i,u,0,h,0,m,0,p);let x=0;for(let y=0;y<p;y++)x+=h[y]*m[y];e.update(x,i,1)}}this.setMode=r,this.render=s,this.renderInstances=o,this.renderMultiDraw=a,this.renderMultiDrawInstances=l}function mp(n,t,e,i){let r;function s(){if(r!==void 0)return r;if(t.has("EXT_texture_filter_anisotropic")===!0){const I=t.get("EXT_texture_filter_anisotropic");r=n.getParameter(I.MAX_TEXTURE_MAX_ANISOTROPY_EXT)}else r=0;return r}function o(I){return!(I!==We&&i.convert(I)!==n.getParameter(n.IMPLEMENTATION_COLOR_READ_FORMAT))}function a(I){const N=I===qi&&(t.has("EXT_color_buffer_half_float")||t.has("EXT_color_buffer_float"));return!(I!==Je&&i.convert(I)!==n.getParameter(n.IMPLEMENTATION_COLOR_READ_TYPE)&&I!==cn&&!N)}function l(I){if(I==="highp"){if(n.getShaderPrecisionFormat(n.VERTEX_SHADER,n.HIGH_FLOAT).precision>0&&n.getShaderPrecisionFormat(n.FRAGMENT_SHADER,n.HIGH_FLOAT).precision>0)return"highp";I="mediump"}return I==="mediump"&&n.getShaderPrecisionFormat(n.VERTEX_SHADER,n.MEDIUM_FLOAT).precision>0&&n.getShaderPrecisionFormat(n.FRAGMENT_SHADER,n.MEDIUM_FLOAT).precision>0?"mediump":"lowp"}let u=e.precision!==void 0?e.precision:"highp";const h=l(u);h!==u&&(console.warn("THREE.WebGLRenderer:",u,"not supported, using",h,"instead."),u=h);const p=e.logarithmicDepthBuffer===!0,m=e.reverseDepthBuffer===!0&&t.has("EXT_clip_control"),_=n.getParameter(n.MAX_TEXTURE_IMAGE_UNITS),x=n.getParameter(n.MAX_VERTEX_TEXTURE_IMAGE_UNITS),y=n.getParameter(n.MAX_TEXTURE_SIZE),g=n.getParameter(n.MAX_CUBE_MAP_TEXTURE_SIZE),d=n.getParameter(n.MAX_VERTEX_ATTRIBS),P=n.getParameter(n.MAX_VERTEX_UNIFORM_VECTORS),b=n.getParameter(n.MAX_VARYING_VECTORS),w=n.getParameter(n.MAX_FRAGMENT_UNIFORM_VECTORS),D=x>0,L=n.getParameter(n.MAX_SAMPLES);return{isWebGL2:!0,getMaxAnisotropy:s,getMaxPrecision:l,textureFormatReadable:o,textureTypeReadable:a,precision:u,logarithmicDepthBuffer:p,reverseDepthBuffer:m,maxTextures:_,maxVertexTextures:x,maxTextureSize:y,maxCubemapSize:g,maxAttributes:d,maxVertexUniforms:P,maxVaryings:b,maxFragmentUniforms:w,vertexTextures:D,maxSamples:L}}function _p(n){const t=this;let e=null,i=0,r=!1,s=!1;const o=new Vn,a=new Ft,l={value:null,needsUpdate:!1};this.uniform=l,this.numPlanes=0,this.numIntersection=0,this.init=function(p,m){const _=p.length!==0||m||i!==0||r;return r=m,i=p.length,_},this.beginShadows=function(){s=!0,h(null)},this.endShadows=function(){s=!1},this.setGlobalState=function(p,m){e=h(p,m,0)},this.setState=function(p,m,_){const x=p.clippingPlanes,y=p.clipIntersection,g=p.clipShadows,d=n.get(p);if(!r||x===null||x.length===0||s&&!g)s?h(null):u();else{const P=s?0:i,b=P*4;let w=d.clippingState||null;l.value=w,w=h(x,m,b,_);for(let D=0;D!==b;++D)w[D]=e[D];d.clippingState=w,this.numIntersection=y?this.numPlanes:0,this.numPlanes+=P}};function u(){l.value!==e&&(l.value=e,l.needsUpdate=i>0),t.numPlanes=i,t.numIntersection=0}function h(p,m,_,x){const y=p!==null?p.length:0;let g=null;if(y!==0){if(g=l.value,x!==!0||g===null){const d=_+y*4,P=m.matrixWorldInverse;a.getNormalMatrix(P),(g===null||g.length<d)&&(g=new Float32Array(d));for(let b=0,w=_;b!==y;++b,w+=4)o.copy(p[b]).applyMatrix4(P,a),o.normal.toArray(g,w),g[w+3]=o.constant}l.value=g,l.needsUpdate=!0}return t.numPlanes=y,t.numIntersection=0,g}}function gp(n){let t=new WeakMap;function e(o,a){return a===Ys?o.mapping=vi:a===js&&(o.mapping=yi),o}function i(o){if(o&&o.isTexture){const a=o.mapping;if(a===Ys||a===js)if(t.has(o)){const l=t.get(o).texture;return e(l,o.mapping)}else{const l=o.image;if(l&&l.height>0){const u=new gf(l.height);return u.fromEquirectangularTexture(n,o),t.set(o,u),o.addEventListener("dispose",r),e(u.texture,o.mapping)}else return null}}return o}function r(o){const a=o.target;a.removeEventListener("dispose",r);const l=t.get(a);l!==void 0&&(t.delete(a),l.dispose())}function s(){t=new WeakMap}return{get:i,dispose:s}}const mi=4,No=[.125,.215,.35,.446,.526,.582],Xn=20,bs=new rl,Oo=new Wt;let Cs=null,Rs=0,Ps=0,Ls=!1;const Gn=(1+Math.sqrt(5))/2,pi=1/Gn,Bo=[new H(-Gn,pi,0),new H(Gn,pi,0),new H(-pi,0,Gn),new H(pi,0,Gn),new H(0,Gn,-pi),new H(0,Gn,pi),new H(-1,1,-1),new H(1,1,-1),new H(-1,1,1),new H(1,1,1)],xp=new H;class zo{constructor(t){this._renderer=t,this._pingPongRenderTarget=null,this._lodMax=0,this._cubeSize=0,this._lodPlanes=[],this._sizeLods=[],this._sigmas=[],this._blurMaterial=null,this._cubemapMaterial=null,this._equirectMaterial=null,this._compileMaterial(this._blurMaterial)}fromScene(t,e=0,i=.1,r=100,s={}){const{size:o=256,position:a=xp}=s;Cs=this._renderer.getRenderTarget(),Rs=this._renderer.getActiveCubeFace(),Ps=this._renderer.getActiveMipmapLevel(),Ls=this._renderer.xr.enabled,this._renderer.xr.enabled=!1,this._setSize(o);const l=this._allocateTargets();return l.depthBuffer=!0,this._sceneToCubeUV(t,i,r,l,a),e>0&&this._blur(l,0,0,e),this._applyPMREM(l),this._cleanup(l),l}fromEquirectangular(t,e=null){return this._fromTexture(t,e)}fromCubemap(t,e=null){return this._fromTexture(t,e)}compileCubemapShader(){this._cubemapMaterial===null&&(this._cubemapMaterial=Vo(),this._compileMaterial(this._cubemapMaterial))}compileEquirectangularShader(){this._equirectMaterial===null&&(this._equirectMaterial=Ho(),this._compileMaterial(this._equirectMaterial))}dispose(){this._dispose(),this._cubemapMaterial!==null&&this._cubemapMaterial.dispose(),this._equirectMaterial!==null&&this._equirectMaterial.dispose()}_setSize(t){this._lodMax=Math.floor(Math.log2(t)),this._cubeSize=Math.pow(2,this._lodMax)}_dispose(){this._blurMaterial!==null&&this._blurMaterial.dispose(),this._pingPongRenderTarget!==null&&this._pingPongRenderTarget.dispose();for(let t=0;t<this._lodPlanes.length;t++)this._lodPlanes[t].dispose()}_cleanup(t){this._renderer.setRenderTarget(Cs,Rs,Ps),this._renderer.xr.enabled=Ls,t.scissorTest=!1,Sr(t,0,0,t.width,t.height)}_fromTexture(t,e){t.mapping===vi||t.mapping===yi?this._setSize(t.image.length===0?16:t.image[0].width||t.image[0].image.width):this._setSize(t.image.width/4),Cs=this._renderer.getRenderTarget(),Rs=this._renderer.getActiveCubeFace(),Ps=this._renderer.getActiveMipmapLevel(),Ls=this._renderer.xr.enabled,this._renderer.xr.enabled=!1;const i=e||this._allocateTargets();return this._textureToCubeUV(t,i),this._applyPMREM(i),this._cleanup(i),i}_allocateTargets(){const t=3*Math.max(this._cubeSize,112),e=4*this._cubeSize,i={magFilter:Ke,minFilter:Ke,generateMipmaps:!1,type:qi,format:We,colorSpace:Si,depthBuffer:!1},r=ko(t,e,i);if(this._pingPongRenderTarget===null||this._pingPongRenderTarget.width!==t||this._pingPongRenderTarget.height!==e){this._pingPongRenderTarget!==null&&this._dispose(),this._pingPongRenderTarget=ko(t,e,i);const{_lodMax:s}=this;({sizeLods:this._sizeLods,lodPlanes:this._lodPlanes,sigmas:this._sigmas}=vp(s)),this._blurMaterial=yp(s,t,e)}return r}_compileMaterial(t){const e=new ce(this._lodPlanes[0],t);this._renderer.compile(e,bs)}_sceneToCubeUV(t,e,i,r,s){const l=new Ne(90,1,e,i),u=[1,-1,1,1,1,1],h=[1,1,1,-1,-1,-1],p=this._renderer,m=p.autoClear,_=p.toneMapping;p.getClearColor(Oo),p.toneMapping=Rn,p.autoClear=!1;const x=new un({name:"PMREM.Background",side:Ae,depthWrite:!1,depthTest:!1}),y=new ce(new Kn,x);let g=!1;const d=t.background;d?d.isColor&&(x.color.copy(d),t.background=null,g=!0):(x.color.copy(Oo),g=!0);for(let P=0;P<6;P++){const b=P%3;b===0?(l.up.set(0,u[P],0),l.position.set(s.x,s.y,s.z),l.lookAt(s.x+h[P],s.y,s.z)):b===1?(l.up.set(0,0,u[P]),l.position.set(s.x,s.y,s.z),l.lookAt(s.x,s.y+h[P],s.z)):(l.up.set(0,u[P],0),l.position.set(s.x,s.y,s.z),l.lookAt(s.x,s.y,s.z+h[P]));const w=this._cubeSize;Sr(r,b*w,P>2?w:0,w,w),p.setRenderTarget(r),g&&p.render(y,l),p.render(t,l)}y.geometry.dispose(),y.material.dispose(),p.toneMapping=_,p.autoClear=m,t.background=d}_textureToCubeUV(t,e){const i=this._renderer,r=t.mapping===vi||t.mapping===yi;r?(this._cubemapMaterial===null&&(this._cubemapMaterial=Vo()),this._cubemapMaterial.uniforms.flipEnvMap.value=t.isRenderTargetTexture===!1?-1:1):this._equirectMaterial===null&&(this._equirectMaterial=Ho());const s=r?this._cubemapMaterial:this._equirectMaterial,o=new ce(this._lodPlanes[0],s),a=s.uniforms;a.envMap.value=t;const l=this._cubeSize;Sr(e,0,0,3*l,2*l),i.setRenderTarget(e),i.render(o,bs)}_applyPMREM(t){const e=this._renderer,i=e.autoClear;e.autoClear=!1;const r=this._lodPlanes.length;for(let s=1;s<r;s++){const o=Math.sqrt(this._sigmas[s]*this._sigmas[s]-this._sigmas[s-1]*this._sigmas[s-1]),a=Bo[(r-s-1)%Bo.length];this._blur(t,s-1,s,o,a)}e.autoClear=i}_blur(t,e,i,r,s){const o=this._pingPongRenderTarget;this._halfBlur(t,o,e,i,r,"latitudinal",s),this._halfBlur(o,t,i,i,r,"longitudinal",s)}_halfBlur(t,e,i,r,s,o,a){const l=this._renderer,u=this._blurMaterial;o!=="latitudinal"&&o!=="longitudinal"&&console.error("blur direction must be either latitudinal or longitudinal!");const h=3,p=new ce(this._lodPlanes[r],u),m=u.uniforms,_=this._sizeLods[i]-1,x=isFinite(s)?Math.PI/(2*_):2*Math.PI/(2*Xn-1),y=s/x,g=isFinite(s)?1+Math.floor(h*y):Xn;g>Xn&&console.warn(`sigmaRadians, ${s}, is too large and will clip, as it requested ${g} samples when the maximum is set to ${Xn}`);const d=[];let P=0;for(let I=0;I<Xn;++I){const N=I/y,A=Math.exp(-N*N/2);d.push(A),I===0?P+=A:I<g&&(P+=2*A)}for(let I=0;I<d.length;I++)d[I]=d[I]/P;m.envMap.value=t.texture,m.samples.value=g,m.weights.value=d,m.latitudinal.value=o==="latitudinal",a&&(m.poleAxis.value=a);const{_lodMax:b}=this;m.dTheta.value=x,m.mipInt.value=b-i;const w=this._sizeLods[r],D=3*w*(r>b-mi?r-b+mi:0),L=4*(this._cubeSize-w);Sr(e,D,L,3*w,2*w),l.setRenderTarget(e),l.render(p,bs)}}function vp(n){const t=[],e=[],i=[];let r=n;const s=n-mi+1+No.length;for(let o=0;o<s;o++){const a=Math.pow(2,r);e.push(a);let l=1/a;o>n-mi?l=No[o-n+mi-1]:o===0&&(l=0),i.push(l);const u=1/(a-2),h=-u,p=1+u,m=[h,h,p,h,p,p,h,h,p,p,h,p],_=6,x=6,y=3,g=2,d=1,P=new Float32Array(y*x*_),b=new Float32Array(g*x*_),w=new Float32Array(d*x*_);for(let L=0;L<_;L++){const I=L%3*2/3-1,N=L>2?0:-1,A=[I,N,0,I+2/3,N,0,I+2/3,N+1,0,I,N,0,I+2/3,N+1,0,I,N+1,0];P.set(A,y*x*L),b.set(m,g*x*L);const T=[L,L,L,L,L,L];w.set(T,d*x*L)}const D=new Le;D.setAttribute("position",new $e(P,y)),D.setAttribute("uv",new $e(b,g)),D.setAttribute("faceIndex",new $e(w,d)),t.push(D),r>mi&&r--}return{lodPlanes:t,sizeLods:e,sigmas:i}}function ko(n,t,e){const i=new jn(n,t,e);return i.texture.mapping=Gr,i.texture.name="PMREM.cubeUv",i.scissorTest=!0,i}function Sr(n,t,e,i,r){n.viewport.set(t,e,i,r),n.scissor.set(t,e,i,r)}function yp(n,t,e){const i=new Float32Array(Xn),r=new H(0,1,0);return new Ln({name:"SphericalGaussianBlur",defines:{n:Xn,CUBEUV_TEXEL_WIDTH:1/t,CUBEUV_TEXEL_HEIGHT:1/e,CUBEUV_MAX_MIP:`${n}.0`},uniforms:{envMap:{value:null},samples:{value:1},weights:{value:i},latitudinal:{value:!1},dTheta:{value:0},mipInt:{value:0},poleAxis:{value:r}},vertexShader:Ha(),fragmentShader:`

			precision mediump float;
			precision mediump int;

			varying vec3 vOutputDirection;

			uniform sampler2D envMap;
			uniform int samples;
			uniform float weights[ n ];
			uniform bool latitudinal;
			uniform float dTheta;
			uniform float mipInt;
			uniform vec3 poleAxis;

			#define ENVMAP_TYPE_CUBE_UV
			#include <cube_uv_reflection_fragment>

			vec3 getSample( float theta, vec3 axis ) {

				float cosTheta = cos( theta );
				// Rodrigues' axis-angle rotation
				vec3 sampleDirection = vOutputDirection * cosTheta
					+ cross( axis, vOutputDirection ) * sin( theta )
					+ axis * dot( axis, vOutputDirection ) * ( 1.0 - cosTheta );

				return bilinearCubeUV( envMap, sampleDirection, mipInt );

			}

			void main() {

				vec3 axis = latitudinal ? poleAxis : cross( poleAxis, vOutputDirection );

				if ( all( equal( axis, vec3( 0.0 ) ) ) ) {

					axis = vec3( vOutputDirection.z, 0.0, - vOutputDirection.x );

				}

				axis = normalize( axis );

				gl_FragColor = vec4( 0.0, 0.0, 0.0, 1.0 );
				gl_FragColor.rgb += weights[ 0 ] * getSample( 0.0, axis );

				for ( int i = 1; i < n; i++ ) {

					if ( i >= samples ) {

						break;

					}

					float theta = dTheta * float( i );
					gl_FragColor.rgb += weights[ i ] * getSample( -1.0 * theta, axis );
					gl_FragColor.rgb += weights[ i ] * getSample( theta, axis );

				}

			}
		`,blending:Cn,depthTest:!1,depthWrite:!1})}function Ho(){return new Ln({name:"EquirectangularToCubeUV",uniforms:{envMap:{value:null}},vertexShader:Ha(),fragmentShader:`

			precision mediump float;
			precision mediump int;

			varying vec3 vOutputDirection;

			uniform sampler2D envMap;

			#include <common>

			void main() {

				vec3 outputDirection = normalize( vOutputDirection );
				vec2 uv = equirectUv( outputDirection );

				gl_FragColor = vec4( texture2D ( envMap, uv ).rgb, 1.0 );

			}
		`,blending:Cn,depthTest:!1,depthWrite:!1})}function Vo(){return new Ln({name:"CubemapToCubeUV",uniforms:{envMap:{value:null},flipEnvMap:{value:-1}},vertexShader:Ha(),fragmentShader:`

			precision mediump float;
			precision mediump int;

			uniform float flipEnvMap;

			varying vec3 vOutputDirection;

			uniform samplerCube envMap;

			void main() {

				gl_FragColor = textureCube( envMap, vec3( flipEnvMap * vOutputDirection.x, vOutputDirection.yz ) );

			}
		`,blending:Cn,depthTest:!1,depthWrite:!1})}function Ha(){return`

		precision mediump float;
		precision mediump int;

		attribute float faceIndex;

		varying vec3 vOutputDirection;

		// RH coordinate system; PMREM face-indexing convention
		vec3 getDirection( vec2 uv, float face ) {

			uv = 2.0 * uv - 1.0;

			vec3 direction = vec3( uv, 1.0 );

			if ( face == 0.0 ) {

				direction = direction.zyx; // ( 1, v, u ) pos x

			} else if ( face == 1.0 ) {

				direction = direction.xzy;
				direction.xz *= -1.0; // ( -u, 1, -v ) pos y

			} else if ( face == 2.0 ) {

				direction.x *= -1.0; // ( -u, v, 1 ) pos z

			} else if ( face == 3.0 ) {

				direction = direction.zyx;
				direction.xz *= -1.0; // ( -1, v, -u ) neg x

			} else if ( face == 4.0 ) {

				direction = direction.xzy;
				direction.xy *= -1.0; // ( -u, -1, v ) neg y

			} else if ( face == 5.0 ) {

				direction.z *= -1.0; // ( u, v, -1 ) neg z

			}

			return direction;

		}

		void main() {

			vOutputDirection = getDirection( uv, faceIndex );
			gl_Position = vec4( position, 1.0 );

		}
	`}function Sp(n){let t=new WeakMap,e=null;function i(a){if(a&&a.isTexture){const l=a.mapping,u=l===Ys||l===js,h=l===vi||l===yi;if(u||h){let p=t.get(a);const m=p!==void 0?p.texture.pmremVersion:0;if(a.isRenderTargetTexture&&a.pmremVersion!==m)return e===null&&(e=new zo(n)),p=u?e.fromEquirectangular(a,p):e.fromCubemap(a,p),p.texture.pmremVersion=a.pmremVersion,t.set(a,p),p.texture;if(p!==void 0)return p.texture;{const _=a.image;return u&&_&&_.height>0||h&&_&&r(_)?(e===null&&(e=new zo(n)),p=u?e.fromEquirectangular(a):e.fromCubemap(a),p.texture.pmremVersion=a.pmremVersion,t.set(a,p),a.addEventListener("dispose",s),p.texture):null}}}return a}function r(a){let l=0;const u=6;for(let h=0;h<u;h++)a[h]!==void 0&&l++;return l===u}function s(a){const l=a.target;l.removeEventListener("dispose",s);const u=t.get(l);u!==void 0&&(t.delete(l),u.dispose())}function o(){t=new WeakMap,e!==null&&(e.dispose(),e=null)}return{get:i,dispose:o}}function Mp(n){const t={};function e(i){if(t[i]!==void 0)return t[i];let r;switch(i){case"WEBGL_depth_texture":r=n.getExtension("WEBGL_depth_texture")||n.getExtension("MOZ_WEBGL_depth_texture")||n.getExtension("WEBKIT_WEBGL_depth_texture");break;case"EXT_texture_filter_anisotropic":r=n.getExtension("EXT_texture_filter_anisotropic")||n.getExtension("MOZ_EXT_texture_filter_anisotropic")||n.getExtension("WEBKIT_EXT_texture_filter_anisotropic");break;case"WEBGL_compressed_texture_s3tc":r=n.getExtension("WEBGL_compressed_texture_s3tc")||n.getExtension("MOZ_WEBGL_compressed_texture_s3tc")||n.getExtension("WEBKIT_WEBGL_compressed_texture_s3tc");break;case"WEBGL_compressed_texture_pvrtc":r=n.getExtension("WEBGL_compressed_texture_pvrtc")||n.getExtension("WEBKIT_WEBGL_compressed_texture_pvrtc");break;default:r=n.getExtension(i)}return t[i]=r,r}return{has:function(i){return e(i)!==null},init:function(){e("EXT_color_buffer_float"),e("WEBGL_clip_cull_distance"),e("OES_texture_float_linear"),e("EXT_color_buffer_half_float"),e("WEBGL_multisampled_render_to_texture"),e("WEBGL_render_shared_exponent")},get:function(i){const r=e(i);return r===null&&Ir("THREE.WebGLRenderer: "+i+" extension not supported."),r}}}function Ep(n,t,e,i){const r={},s=new WeakMap;function o(p){const m=p.target;m.index!==null&&t.remove(m.index);for(const x in m.attributes)t.remove(m.attributes[x]);m.removeEventListener("dispose",o),delete r[m.id];const _=s.get(m);_&&(t.remove(_),s.delete(m)),i.releaseStatesOfGeometry(m),m.isInstancedBufferGeometry===!0&&delete m._maxInstanceCount,e.memory.geometries--}function a(p,m){return r[m.id]===!0||(m.addEventListener("dispose",o),r[m.id]=!0,e.memory.geometries++),m}function l(p){const m=p.attributes;for(const _ in m)t.update(m[_],n.ARRAY_BUFFER)}function u(p){const m=[],_=p.index,x=p.attributes.position;let y=0;if(_!==null){const P=_.array;y=_.version;for(let b=0,w=P.length;b<w;b+=3){const D=P[b+0],L=P[b+1],I=P[b+2];m.push(D,L,L,I,I,D)}}else if(x!==void 0){const P=x.array;y=x.version;for(let b=0,w=P.length/3-1;b<w;b+=3){const D=b+0,L=b+1,I=b+2;m.push(D,L,L,I,I,D)}}else return;const g=new(qc(m)?Jc:Zc)(m,1);g.version=y;const d=s.get(p);d&&t.remove(d),s.set(p,g)}function h(p){const m=s.get(p);if(m){const _=p.index;_!==null&&m.version<_.version&&u(p)}else u(p);return s.get(p)}return{get:a,update:l,getWireframeAttribute:h}}function Tp(n,t,e){let i;function r(m){i=m}let s,o;function a(m){s=m.type,o=m.bytesPerElement}function l(m,_){n.drawElements(i,_,s,m*o),e.update(_,i,1)}function u(m,_,x){x!==0&&(n.drawElementsInstanced(i,_,s,m*o,x),e.update(_,i,x))}function h(m,_,x){if(x===0)return;t.get("WEBGL_multi_draw").multiDrawElementsWEBGL(i,_,0,s,m,0,x);let g=0;for(let d=0;d<x;d++)g+=_[d];e.update(g,i,1)}function p(m,_,x,y){if(x===0)return;const g=t.get("WEBGL_multi_draw");if(g===null)for(let d=0;d<m.length;d++)u(m[d]/o,_[d],y[d]);else{g.multiDrawElementsInstancedWEBGL(i,_,0,s,m,0,y,0,x);let d=0;for(let P=0;P<x;P++)d+=_[P]*y[P];e.update(d,i,1)}}this.setMode=r,this.setIndex=a,this.render=l,this.renderInstances=u,this.renderMultiDraw=h,this.renderMultiDrawInstances=p}function wp(n){const t={geometries:0,textures:0},e={frame:0,calls:0,triangles:0,points:0,lines:0};function i(s,o,a){switch(e.calls++,o){case n.TRIANGLES:e.triangles+=a*(s/3);break;case n.LINES:e.lines+=a*(s/2);break;case n.LINE_STRIP:e.lines+=a*(s-1);break;case n.LINE_LOOP:e.lines+=a*s;break;case n.POINTS:e.points+=a*s;break;default:console.error("THREE.WebGLInfo: Unknown draw mode:",o);break}}function r(){e.calls=0,e.triangles=0,e.points=0,e.lines=0}return{memory:t,render:e,programs:null,autoReset:!0,reset:r,update:i}}function Ap(n,t,e){const i=new WeakMap,r=new se;function s(o,a,l){const u=o.morphTargetInfluences,h=a.morphAttributes.position||a.morphAttributes.normal||a.morphAttributes.color,p=h!==void 0?h.length:0;let m=i.get(a);if(m===void 0||m.count!==p){let A=function(){I.dispose(),i.delete(a),a.removeEventListener("dispose",A)};m!==void 0&&m.texture.dispose();const _=a.morphAttributes.position!==void 0,x=a.morphAttributes.normal!==void 0,y=a.morphAttributes.color!==void 0,g=a.morphAttributes.position||[],d=a.morphAttributes.normal||[],P=a.morphAttributes.color||[];let b=0;_===!0&&(b=1),x===!0&&(b=2),y===!0&&(b=3);let w=a.attributes.position.count*b,D=1;w>t.maxTextureSize&&(D=Math.ceil(w/t.maxTextureSize),w=t.maxTextureSize);const L=new Float32Array(w*D*4*p),I=new $c(L,w,D,p);I.type=cn,I.needsUpdate=!0;const N=b*4;for(let T=0;T<p;T++){const O=g[T],$=d[T],W=P[T],K=w*D*4*T;for(let it=0;it<O.count;it++){const Z=it*N;_===!0&&(r.fromBufferAttribute(O,it),L[K+Z+0]=r.x,L[K+Z+1]=r.y,L[K+Z+2]=r.z,L[K+Z+3]=0),x===!0&&(r.fromBufferAttribute($,it),L[K+Z+4]=r.x,L[K+Z+5]=r.y,L[K+Z+6]=r.z,L[K+Z+7]=0),y===!0&&(r.fromBufferAttribute(W,it),L[K+Z+8]=r.x,L[K+Z+9]=r.y,L[K+Z+10]=r.z,L[K+Z+11]=W.itemSize===4?r.w:1)}}m={count:p,texture:I,size:new Xt(w,D)},i.set(a,m),a.addEventListener("dispose",A)}if(o.isInstancedMesh===!0&&o.morphTexture!==null)l.getUniforms().setValue(n,"morphTexture",o.morphTexture,e);else{let _=0;for(let y=0;y<u.length;y++)_+=u[y];const x=a.morphTargetsRelative?1:1-_;l.getUniforms().setValue(n,"morphTargetBaseInfluence",x),l.getUniforms().setValue(n,"morphTargetInfluences",u)}l.getUniforms().setValue(n,"morphTargetsTexture",m.texture,e),l.getUniforms().setValue(n,"morphTargetsTextureSize",m.size)}return{update:s}}function bp(n,t,e,i){let r=new WeakMap;function s(l){const u=i.render.frame,h=l.geometry,p=t.get(l,h);if(r.get(p)!==u&&(t.update(p),r.set(p,u)),l.isInstancedMesh&&(l.hasEventListener("dispose",a)===!1&&l.addEventListener("dispose",a),r.get(l)!==u&&(e.update(l.instanceMatrix,n.ARRAY_BUFFER),l.instanceColor!==null&&e.update(l.instanceColor,n.ARRAY_BUFFER),r.set(l,u))),l.isSkinnedMesh){const m=l.skeleton;r.get(m)!==u&&(m.update(),r.set(m,u))}return p}function o(){r=new WeakMap}function a(l){const u=l.target;u.removeEventListener("dispose",a),e.remove(u.instanceMatrix),u.instanceColor!==null&&e.remove(u.instanceColor)}return{update:s,dispose:o}}const al=new be,Go=new il(1,1),ol=new $c,cl=new Qu,ll=new el,Wo=[],Xo=[],qo=new Float32Array(16),$o=new Float32Array(9),Yo=new Float32Array(4);function wi(n,t,e){const i=n[0];if(i<=0||i>0)return n;const r=t*e;let s=Wo[r];if(s===void 0&&(s=new Float32Array(r),Wo[r]=s),t!==0){i.toArray(s,0);for(let o=1,a=0;o!==t;++o)a+=e,n[o].toArray(s,a)}return s}function le(n,t){if(n.length!==t.length)return!1;for(let e=0,i=n.length;e<i;e++)if(n[e]!==t[e])return!1;return!0}function ue(n,t){for(let e=0,i=t.length;e<i;e++)n[e]=t[e]}function qr(n,t){let e=Xo[t];e===void 0&&(e=new Int32Array(t),Xo[t]=e);for(let i=0;i!==t;++i)e[i]=n.allocateTextureUnit();return e}function Cp(n,t){const e=this.cache;e[0]!==t&&(n.uniform1f(this.addr,t),e[0]=t)}function Rp(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y)&&(n.uniform2f(this.addr,t.x,t.y),e[0]=t.x,e[1]=t.y);else{if(le(e,t))return;n.uniform2fv(this.addr,t),ue(e,t)}}function Pp(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z)&&(n.uniform3f(this.addr,t.x,t.y,t.z),e[0]=t.x,e[1]=t.y,e[2]=t.z);else if(t.r!==void 0)(e[0]!==t.r||e[1]!==t.g||e[2]!==t.b)&&(n.uniform3f(this.addr,t.r,t.g,t.b),e[0]=t.r,e[1]=t.g,e[2]=t.b);else{if(le(e,t))return;n.uniform3fv(this.addr,t),ue(e,t)}}function Lp(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z||e[3]!==t.w)&&(n.uniform4f(this.addr,t.x,t.y,t.z,t.w),e[0]=t.x,e[1]=t.y,e[2]=t.z,e[3]=t.w);else{if(le(e,t))return;n.uniform4fv(this.addr,t),ue(e,t)}}function Dp(n,t){const e=this.cache,i=t.elements;if(i===void 0){if(le(e,t))return;n.uniformMatrix2fv(this.addr,!1,t),ue(e,t)}else{if(le(e,i))return;Yo.set(i),n.uniformMatrix2fv(this.addr,!1,Yo),ue(e,i)}}function Ip(n,t){const e=this.cache,i=t.elements;if(i===void 0){if(le(e,t))return;n.uniformMatrix3fv(this.addr,!1,t),ue(e,t)}else{if(le(e,i))return;$o.set(i),n.uniformMatrix3fv(this.addr,!1,$o),ue(e,i)}}function Up(n,t){const e=this.cache,i=t.elements;if(i===void 0){if(le(e,t))return;n.uniformMatrix4fv(this.addr,!1,t),ue(e,t)}else{if(le(e,i))return;qo.set(i),n.uniformMatrix4fv(this.addr,!1,qo),ue(e,i)}}function Fp(n,t){const e=this.cache;e[0]!==t&&(n.uniform1i(this.addr,t),e[0]=t)}function Np(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y)&&(n.uniform2i(this.addr,t.x,t.y),e[0]=t.x,e[1]=t.y);else{if(le(e,t))return;n.uniform2iv(this.addr,t),ue(e,t)}}function Op(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z)&&(n.uniform3i(this.addr,t.x,t.y,t.z),e[0]=t.x,e[1]=t.y,e[2]=t.z);else{if(le(e,t))return;n.uniform3iv(this.addr,t),ue(e,t)}}function Bp(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z||e[3]!==t.w)&&(n.uniform4i(this.addr,t.x,t.y,t.z,t.w),e[0]=t.x,e[1]=t.y,e[2]=t.z,e[3]=t.w);else{if(le(e,t))return;n.uniform4iv(this.addr,t),ue(e,t)}}function zp(n,t){const e=this.cache;e[0]!==t&&(n.uniform1ui(this.addr,t),e[0]=t)}function kp(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y)&&(n.uniform2ui(this.addr,t.x,t.y),e[0]=t.x,e[1]=t.y);else{if(le(e,t))return;n.uniform2uiv(this.addr,t),ue(e,t)}}function Hp(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z)&&(n.uniform3ui(this.addr,t.x,t.y,t.z),e[0]=t.x,e[1]=t.y,e[2]=t.z);else{if(le(e,t))return;n.uniform3uiv(this.addr,t),ue(e,t)}}function Vp(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z||e[3]!==t.w)&&(n.uniform4ui(this.addr,t.x,t.y,t.z,t.w),e[0]=t.x,e[1]=t.y,e[2]=t.z,e[3]=t.w);else{if(le(e,t))return;n.uniform4uiv(this.addr,t),ue(e,t)}}function Gp(n,t,e){const i=this.cache,r=e.allocateTextureUnit();i[0]!==r&&(n.uniform1i(this.addr,r),i[0]=r);let s;this.type===n.SAMPLER_2D_SHADOW?(Go.compareFunction=Xc,s=Go):s=al,e.setTexture2D(t||s,r)}function Wp(n,t,e){const i=this.cache,r=e.allocateTextureUnit();i[0]!==r&&(n.uniform1i(this.addr,r),i[0]=r),e.setTexture3D(t||cl,r)}function Xp(n,t,e){const i=this.cache,r=e.allocateTextureUnit();i[0]!==r&&(n.uniform1i(this.addr,r),i[0]=r),e.setTextureCube(t||ll,r)}function qp(n,t,e){const i=this.cache,r=e.allocateTextureUnit();i[0]!==r&&(n.uniform1i(this.addr,r),i[0]=r),e.setTexture2DArray(t||ol,r)}function $p(n){switch(n){case 5126:return Cp;case 35664:return Rp;case 35665:return Pp;case 35666:return Lp;case 35674:return Dp;case 35675:return Ip;case 35676:return Up;case 5124:case 35670:return Fp;case 35667:case 35671:return Np;case 35668:case 35672:return Op;case 35669:case 35673:return Bp;case 5125:return zp;case 36294:return kp;case 36295:return Hp;case 36296:return Vp;case 35678:case 36198:case 36298:case 36306:case 35682:return Gp;case 35679:case 36299:case 36307:return Wp;case 35680:case 36300:case 36308:case 36293:return Xp;case 36289:case 36303:case 36311:case 36292:return qp}}function Yp(n,t){n.uniform1fv(this.addr,t)}function jp(n,t){const e=wi(t,this.size,2);n.uniform2fv(this.addr,e)}function Kp(n,t){const e=wi(t,this.size,3);n.uniform3fv(this.addr,e)}function Zp(n,t){const e=wi(t,this.size,4);n.uniform4fv(this.addr,e)}function Jp(n,t){const e=wi(t,this.size,4);n.uniformMatrix2fv(this.addr,!1,e)}function Qp(n,t){const e=wi(t,this.size,9);n.uniformMatrix3fv(this.addr,!1,e)}function tm(n,t){const e=wi(t,this.size,16);n.uniformMatrix4fv(this.addr,!1,e)}function em(n,t){n.uniform1iv(this.addr,t)}function nm(n,t){n.uniform2iv(this.addr,t)}function im(n,t){n.uniform3iv(this.addr,t)}function rm(n,t){n.uniform4iv(this.addr,t)}function sm(n,t){n.uniform1uiv(this.addr,t)}function am(n,t){n.uniform2uiv(this.addr,t)}function om(n,t){n.uniform3uiv(this.addr,t)}function cm(n,t){n.uniform4uiv(this.addr,t)}function lm(n,t,e){const i=this.cache,r=t.length,s=qr(e,r);le(i,s)||(n.uniform1iv(this.addr,s),ue(i,s));for(let o=0;o!==r;++o)e.setTexture2D(t[o]||al,s[o])}function um(n,t,e){const i=this.cache,r=t.length,s=qr(e,r);le(i,s)||(n.uniform1iv(this.addr,s),ue(i,s));for(let o=0;o!==r;++o)e.setTexture3D(t[o]||cl,s[o])}function fm(n,t,e){const i=this.cache,r=t.length,s=qr(e,r);le(i,s)||(n.uniform1iv(this.addr,s),ue(i,s));for(let o=0;o!==r;++o)e.setTextureCube(t[o]||ll,s[o])}function hm(n,t,e){const i=this.cache,r=t.length,s=qr(e,r);le(i,s)||(n.uniform1iv(this.addr,s),ue(i,s));for(let o=0;o!==r;++o)e.setTexture2DArray(t[o]||ol,s[o])}function dm(n){switch(n){case 5126:return Yp;case 35664:return jp;case 35665:return Kp;case 35666:return Zp;case 35674:return Jp;case 35675:return Qp;case 35676:return tm;case 5124:case 35670:return em;case 35667:case 35671:return nm;case 35668:case 35672:return im;case 35669:case 35673:return rm;case 5125:return sm;case 36294:return am;case 36295:return om;case 36296:return cm;case 35678:case 36198:case 36298:case 36306:case 35682:return lm;case 35679:case 36299:case 36307:return um;case 35680:case 36300:case 36308:case 36293:return fm;case 36289:case 36303:case 36311:case 36292:return hm}}class pm{constructor(t,e,i){this.id=t,this.addr=i,this.cache=[],this.type=e.type,this.setValue=$p(e.type)}}class mm{constructor(t,e,i){this.id=t,this.addr=i,this.cache=[],this.type=e.type,this.size=e.size,this.setValue=dm(e.type)}}class _m{constructor(t){this.id=t,this.seq=[],this.map={}}setValue(t,e,i){const r=this.seq;for(let s=0,o=r.length;s!==o;++s){const a=r[s];a.setValue(t,e[a.id],i)}}}const Ds=/(\w+)(\])?(\[|\.)?/g;function jo(n,t){n.seq.push(t),n.map[t.id]=t}function gm(n,t,e){const i=n.name,r=i.length;for(Ds.lastIndex=0;;){const s=Ds.exec(i),o=Ds.lastIndex;let a=s[1];const l=s[2]==="]",u=s[3];if(l&&(a=a|0),u===void 0||u==="["&&o+2===r){jo(e,u===void 0?new pm(a,n,t):new mm(a,n,t));break}else{let p=e.map[a];p===void 0&&(p=new _m(a),jo(e,p)),e=p}}}class Ur{constructor(t,e){this.seq=[],this.map={};const i=t.getProgramParameter(e,t.ACTIVE_UNIFORMS);for(let r=0;r<i;++r){const s=t.getActiveUniform(e,r),o=t.getUniformLocation(e,s.name);gm(s,o,this)}}setValue(t,e,i,r){const s=this.map[e];s!==void 0&&s.setValue(t,i,r)}setOptional(t,e,i){const r=e[i];r!==void 0&&this.setValue(t,i,r)}static upload(t,e,i,r){for(let s=0,o=e.length;s!==o;++s){const a=e[s],l=i[a.id];l.needsUpdate!==!1&&a.setValue(t,l.value,r)}}static seqWithValue(t,e){const i=[];for(let r=0,s=t.length;r!==s;++r){const o=t[r];o.id in e&&i.push(o)}return i}}function Ko(n,t,e){const i=n.createShader(t);return n.shaderSource(i,e),n.compileShader(i),i}const xm=37297;let vm=0;function ym(n,t){const e=n.split(`
`),i=[],r=Math.max(t-6,0),s=Math.min(t+6,e.length);for(let o=r;o<s;o++){const a=o+1;i.push(`${a===t?">":" "} ${a}: ${e[o]}`)}return i.join(`
`)}const Zo=new Ft;function Sm(n){Gt._getMatrix(Zo,Gt.workingColorSpace,n);const t=`mat3( ${Zo.elements.map(e=>e.toFixed(4))} )`;switch(Gt.getTransfer(n)){case Nr:return[t,"LinearTransferOETF"];case Kt:return[t,"sRGBTransferOETF"];default:return console.warn("THREE.WebGLProgram: Unsupported color space: ",n),[t,"LinearTransferOETF"]}}function Jo(n,t,e){const i=n.getShaderParameter(t,n.COMPILE_STATUS),r=n.getShaderInfoLog(t).trim();if(i&&r==="")return"";const s=/ERROR: 0:(\d+)/.exec(r);if(s){const o=parseInt(s[1]);return e.toUpperCase()+`

`+r+`

`+ym(n.getShaderSource(t),o)}else return r}function Mm(n,t){const e=Sm(t);return[`vec4 ${n}( vec4 value ) {`,`	return ${e[1]}( vec4( value.rgb * ${e[0]}, value.a ) );`,"}"].join(`
`)}function Em(n,t){let e;switch(t){case wu:e="Linear";break;case Au:e="Reinhard";break;case bu:e="Cineon";break;case Cu:e="ACESFilmic";break;case Pu:e="AgX";break;case Lu:e="Neutral";break;case Ru:e="Custom";break;default:console.warn("THREE.WebGLProgram: Unsupported toneMapping:",t),e="Linear"}return"vec3 "+n+"( vec3 color ) { return "+e+"ToneMapping( color ); }"}const Mr=new H;function Tm(){Gt.getLuminanceCoefficients(Mr);const n=Mr.x.toFixed(4),t=Mr.y.toFixed(4),e=Mr.z.toFixed(4);return["float luminance( const in vec3 rgb ) {",`	const vec3 weights = vec3( ${n}, ${t}, ${e} );`,"	return dot( weights, rgb );","}"].join(`
`)}function wm(n){return[n.extensionClipCullDistance?"#extension GL_ANGLE_clip_cull_distance : require":"",n.extensionMultiDraw?"#extension GL_ANGLE_multi_draw : require":""].filter(Oi).join(`
`)}function Am(n){const t=[];for(const e in n){const i=n[e];i!==!1&&t.push("#define "+e+" "+i)}return t.join(`
`)}function bm(n,t){const e={},i=n.getProgramParameter(t,n.ACTIVE_ATTRIBUTES);for(let r=0;r<i;r++){const s=n.getActiveAttrib(t,r),o=s.name;let a=1;s.type===n.FLOAT_MAT2&&(a=2),s.type===n.FLOAT_MAT3&&(a=3),s.type===n.FLOAT_MAT4&&(a=4),e[o]={type:s.type,location:n.getAttribLocation(t,o),locationSize:a}}return e}function Oi(n){return n!==""}function Qo(n,t){const e=t.numSpotLightShadows+t.numSpotLightMaps-t.numSpotLightShadowsWithMaps;return n.replace(/NUM_DIR_LIGHTS/g,t.numDirLights).replace(/NUM_SPOT_LIGHTS/g,t.numSpotLights).replace(/NUM_SPOT_LIGHT_MAPS/g,t.numSpotLightMaps).replace(/NUM_SPOT_LIGHT_COORDS/g,e).replace(/NUM_RECT_AREA_LIGHTS/g,t.numRectAreaLights).replace(/NUM_POINT_LIGHTS/g,t.numPointLights).replace(/NUM_HEMI_LIGHTS/g,t.numHemiLights).replace(/NUM_DIR_LIGHT_SHADOWS/g,t.numDirLightShadows).replace(/NUM_SPOT_LIGHT_SHADOWS_WITH_MAPS/g,t.numSpotLightShadowsWithMaps).replace(/NUM_SPOT_LIGHT_SHADOWS/g,t.numSpotLightShadows).replace(/NUM_POINT_LIGHT_SHADOWS/g,t.numPointLightShadows)}function tc(n,t){return n.replace(/NUM_CLIPPING_PLANES/g,t.numClippingPlanes).replace(/UNION_CLIPPING_PLANES/g,t.numClippingPlanes-t.numClipIntersection)}const Cm=/^[ \t]*#include +<([\w\d./]+)>/gm;function wa(n){return n.replace(Cm,Pm)}const Rm=new Map;function Pm(n,t){let e=Nt[t];if(e===void 0){const i=Rm.get(t);if(i!==void 0)e=Nt[i],console.warn('THREE.WebGLRenderer: Shader chunk "%s" has been deprecated. Use "%s" instead.',t,i);else throw new Error("Can not resolve #include <"+t+">")}return wa(e)}const Lm=/#pragma unroll_loop_start\s+for\s*\(\s*int\s+i\s*=\s*(\d+)\s*;\s*i\s*<\s*(\d+)\s*;\s*i\s*\+\+\s*\)\s*{([\s\S]+?)}\s+#pragma unroll_loop_end/g;function ec(n){return n.replace(Lm,Dm)}function Dm(n,t,e,i){let r="";for(let s=parseInt(t);s<parseInt(e);s++)r+=i.replace(/\[\s*i\s*\]/g,"[ "+s+" ]").replace(/UNROLLED_LOOP_INDEX/g,s);return r}function nc(n){let t=`precision ${n.precision} float;
	precision ${n.precision} int;
	precision ${n.precision} sampler2D;
	precision ${n.precision} samplerCube;
	precision ${n.precision} sampler3D;
	precision ${n.precision} sampler2DArray;
	precision ${n.precision} sampler2DShadow;
	precision ${n.precision} samplerCubeShadow;
	precision ${n.precision} sampler2DArrayShadow;
	precision ${n.precision} isampler2D;
	precision ${n.precision} isampler3D;
	precision ${n.precision} isamplerCube;
	precision ${n.precision} isampler2DArray;
	precision ${n.precision} usampler2D;
	precision ${n.precision} usampler3D;
	precision ${n.precision} usamplerCube;
	precision ${n.precision} usampler2DArray;
	`;return n.precision==="highp"?t+=`
#define HIGH_PRECISION`:n.precision==="mediump"?t+=`
#define MEDIUM_PRECISION`:n.precision==="lowp"&&(t+=`
#define LOW_PRECISION`),t}function Im(n){let t="SHADOWMAP_TYPE_BASIC";return n.shadowMapType===Uc?t="SHADOWMAP_TYPE_PCF":n.shadowMapType===ru?t="SHADOWMAP_TYPE_PCF_SOFT":n.shadowMapType===an&&(t="SHADOWMAP_TYPE_VSM"),t}function Um(n){let t="ENVMAP_TYPE_CUBE";if(n.envMap)switch(n.envMapMode){case vi:case yi:t="ENVMAP_TYPE_CUBE";break;case Gr:t="ENVMAP_TYPE_CUBE_UV";break}return t}function Fm(n){let t="ENVMAP_MODE_REFLECTION";if(n.envMap)switch(n.envMapMode){case yi:t="ENVMAP_MODE_REFRACTION";break}return t}function Nm(n){let t="ENVMAP_BLENDING_NONE";if(n.envMap)switch(n.combine){case Fc:t="ENVMAP_BLENDING_MULTIPLY";break;case Eu:t="ENVMAP_BLENDING_MIX";break;case Tu:t="ENVMAP_BLENDING_ADD";break}return t}function Om(n){const t=n.envMapCubeUVHeight;if(t===null)return null;const e=Math.log2(t)-2,i=1/t;return{texelWidth:1/(3*Math.max(Math.pow(2,e),7*16)),texelHeight:i,maxMip:e}}function Bm(n,t,e,i){const r=n.getContext(),s=e.defines;let o=e.vertexShader,a=e.fragmentShader;const l=Im(e),u=Um(e),h=Fm(e),p=Nm(e),m=Om(e),_=wm(e),x=Am(s),y=r.createProgram();let g,d,P=e.glslVersion?"#version "+e.glslVersion+`
`:"";e.isRawShaderMaterial?(g=["#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,x].filter(Oi).join(`
`),g.length>0&&(g+=`
`),d=["#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,x].filter(Oi).join(`
`),d.length>0&&(d+=`
`)):(g=[nc(e),"#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,x,e.extensionClipCullDistance?"#define USE_CLIP_DISTANCE":"",e.batching?"#define USE_BATCHING":"",e.batchingColor?"#define USE_BATCHING_COLOR":"",e.instancing?"#define USE_INSTANCING":"",e.instancingColor?"#define USE_INSTANCING_COLOR":"",e.instancingMorph?"#define USE_INSTANCING_MORPH":"",e.useFog&&e.fog?"#define USE_FOG":"",e.useFog&&e.fogExp2?"#define FOG_EXP2":"",e.map?"#define USE_MAP":"",e.envMap?"#define USE_ENVMAP":"",e.envMap?"#define "+h:"",e.lightMap?"#define USE_LIGHTMAP":"",e.aoMap?"#define USE_AOMAP":"",e.bumpMap?"#define USE_BUMPMAP":"",e.normalMap?"#define USE_NORMALMAP":"",e.normalMapObjectSpace?"#define USE_NORMALMAP_OBJECTSPACE":"",e.normalMapTangentSpace?"#define USE_NORMALMAP_TANGENTSPACE":"",e.displacementMap?"#define USE_DISPLACEMENTMAP":"",e.emissiveMap?"#define USE_EMISSIVEMAP":"",e.anisotropy?"#define USE_ANISOTROPY":"",e.anisotropyMap?"#define USE_ANISOTROPYMAP":"",e.clearcoatMap?"#define USE_CLEARCOATMAP":"",e.clearcoatRoughnessMap?"#define USE_CLEARCOAT_ROUGHNESSMAP":"",e.clearcoatNormalMap?"#define USE_CLEARCOAT_NORMALMAP":"",e.iridescenceMap?"#define USE_IRIDESCENCEMAP":"",e.iridescenceThicknessMap?"#define USE_IRIDESCENCE_THICKNESSMAP":"",e.specularMap?"#define USE_SPECULARMAP":"",e.specularColorMap?"#define USE_SPECULAR_COLORMAP":"",e.specularIntensityMap?"#define USE_SPECULAR_INTENSITYMAP":"",e.roughnessMap?"#define USE_ROUGHNESSMAP":"",e.metalnessMap?"#define USE_METALNESSMAP":"",e.alphaMap?"#define USE_ALPHAMAP":"",e.alphaHash?"#define USE_ALPHAHASH":"",e.transmission?"#define USE_TRANSMISSION":"",e.transmissionMap?"#define USE_TRANSMISSIONMAP":"",e.thicknessMap?"#define USE_THICKNESSMAP":"",e.sheenColorMap?"#define USE_SHEEN_COLORMAP":"",e.sheenRoughnessMap?"#define USE_SHEEN_ROUGHNESSMAP":"",e.mapUv?"#define MAP_UV "+e.mapUv:"",e.alphaMapUv?"#define ALPHAMAP_UV "+e.alphaMapUv:"",e.lightMapUv?"#define LIGHTMAP_UV "+e.lightMapUv:"",e.aoMapUv?"#define AOMAP_UV "+e.aoMapUv:"",e.emissiveMapUv?"#define EMISSIVEMAP_UV "+e.emissiveMapUv:"",e.bumpMapUv?"#define BUMPMAP_UV "+e.bumpMapUv:"",e.normalMapUv?"#define NORMALMAP_UV "+e.normalMapUv:"",e.displacementMapUv?"#define DISPLACEMENTMAP_UV "+e.displacementMapUv:"",e.metalnessMapUv?"#define METALNESSMAP_UV "+e.metalnessMapUv:"",e.roughnessMapUv?"#define ROUGHNESSMAP_UV "+e.roughnessMapUv:"",e.anisotropyMapUv?"#define ANISOTROPYMAP_UV "+e.anisotropyMapUv:"",e.clearcoatMapUv?"#define CLEARCOATMAP_UV "+e.clearcoatMapUv:"",e.clearcoatNormalMapUv?"#define CLEARCOAT_NORMALMAP_UV "+e.clearcoatNormalMapUv:"",e.clearcoatRoughnessMapUv?"#define CLEARCOAT_ROUGHNESSMAP_UV "+e.clearcoatRoughnessMapUv:"",e.iridescenceMapUv?"#define IRIDESCENCEMAP_UV "+e.iridescenceMapUv:"",e.iridescenceThicknessMapUv?"#define IRIDESCENCE_THICKNESSMAP_UV "+e.iridescenceThicknessMapUv:"",e.sheenColorMapUv?"#define SHEEN_COLORMAP_UV "+e.sheenColorMapUv:"",e.sheenRoughnessMapUv?"#define SHEEN_ROUGHNESSMAP_UV "+e.sheenRoughnessMapUv:"",e.specularMapUv?"#define SPECULARMAP_UV "+e.specularMapUv:"",e.specularColorMapUv?"#define SPECULAR_COLORMAP_UV "+e.specularColorMapUv:"",e.specularIntensityMapUv?"#define SPECULAR_INTENSITYMAP_UV "+e.specularIntensityMapUv:"",e.transmissionMapUv?"#define TRANSMISSIONMAP_UV "+e.transmissionMapUv:"",e.thicknessMapUv?"#define THICKNESSMAP_UV "+e.thicknessMapUv:"",e.vertexTangents&&e.flatShading===!1?"#define USE_TANGENT":"",e.vertexColors?"#define USE_COLOR":"",e.vertexAlphas?"#define USE_COLOR_ALPHA":"",e.vertexUv1s?"#define USE_UV1":"",e.vertexUv2s?"#define USE_UV2":"",e.vertexUv3s?"#define USE_UV3":"",e.pointsUvs?"#define USE_POINTS_UV":"",e.flatShading?"#define FLAT_SHADED":"",e.skinning?"#define USE_SKINNING":"",e.morphTargets?"#define USE_MORPHTARGETS":"",e.morphNormals&&e.flatShading===!1?"#define USE_MORPHNORMALS":"",e.morphColors?"#define USE_MORPHCOLORS":"",e.morphTargetsCount>0?"#define MORPHTARGETS_TEXTURE_STRIDE "+e.morphTextureStride:"",e.morphTargetsCount>0?"#define MORPHTARGETS_COUNT "+e.morphTargetsCount:"",e.doubleSided?"#define DOUBLE_SIDED":"",e.flipSided?"#define FLIP_SIDED":"",e.shadowMapEnabled?"#define USE_SHADOWMAP":"",e.shadowMapEnabled?"#define "+l:"",e.sizeAttenuation?"#define USE_SIZEATTENUATION":"",e.numLightProbes>0?"#define USE_LIGHT_PROBES":"",e.logarithmicDepthBuffer?"#define USE_LOGDEPTHBUF":"",e.reverseDepthBuffer?"#define USE_REVERSEDEPTHBUF":"","uniform mat4 modelMatrix;","uniform mat4 modelViewMatrix;","uniform mat4 projectionMatrix;","uniform mat4 viewMatrix;","uniform mat3 normalMatrix;","uniform vec3 cameraPosition;","uniform bool isOrthographic;","#ifdef USE_INSTANCING","	attribute mat4 instanceMatrix;","#endif","#ifdef USE_INSTANCING_COLOR","	attribute vec3 instanceColor;","#endif","#ifdef USE_INSTANCING_MORPH","	uniform sampler2D morphTexture;","#endif","attribute vec3 position;","attribute vec3 normal;","attribute vec2 uv;","#ifdef USE_UV1","	attribute vec2 uv1;","#endif","#ifdef USE_UV2","	attribute vec2 uv2;","#endif","#ifdef USE_UV3","	attribute vec2 uv3;","#endif","#ifdef USE_TANGENT","	attribute vec4 tangent;","#endif","#if defined( USE_COLOR_ALPHA )","	attribute vec4 color;","#elif defined( USE_COLOR )","	attribute vec3 color;","#endif","#ifdef USE_SKINNING","	attribute vec4 skinIndex;","	attribute vec4 skinWeight;","#endif",`
`].filter(Oi).join(`
`),d=[nc(e),"#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,x,e.useFog&&e.fog?"#define USE_FOG":"",e.useFog&&e.fogExp2?"#define FOG_EXP2":"",e.alphaToCoverage?"#define ALPHA_TO_COVERAGE":"",e.map?"#define USE_MAP":"",e.matcap?"#define USE_MATCAP":"",e.envMap?"#define USE_ENVMAP":"",e.envMap?"#define "+u:"",e.envMap?"#define "+h:"",e.envMap?"#define "+p:"",m?"#define CUBEUV_TEXEL_WIDTH "+m.texelWidth:"",m?"#define CUBEUV_TEXEL_HEIGHT "+m.texelHeight:"",m?"#define CUBEUV_MAX_MIP "+m.maxMip+".0":"",e.lightMap?"#define USE_LIGHTMAP":"",e.aoMap?"#define USE_AOMAP":"",e.bumpMap?"#define USE_BUMPMAP":"",e.normalMap?"#define USE_NORMALMAP":"",e.normalMapObjectSpace?"#define USE_NORMALMAP_OBJECTSPACE":"",e.normalMapTangentSpace?"#define USE_NORMALMAP_TANGENTSPACE":"",e.emissiveMap?"#define USE_EMISSIVEMAP":"",e.anisotropy?"#define USE_ANISOTROPY":"",e.anisotropyMap?"#define USE_ANISOTROPYMAP":"",e.clearcoat?"#define USE_CLEARCOAT":"",e.clearcoatMap?"#define USE_CLEARCOATMAP":"",e.clearcoatRoughnessMap?"#define USE_CLEARCOAT_ROUGHNESSMAP":"",e.clearcoatNormalMap?"#define USE_CLEARCOAT_NORMALMAP":"",e.dispersion?"#define USE_DISPERSION":"",e.iridescence?"#define USE_IRIDESCENCE":"",e.iridescenceMap?"#define USE_IRIDESCENCEMAP":"",e.iridescenceThicknessMap?"#define USE_IRIDESCENCE_THICKNESSMAP":"",e.specularMap?"#define USE_SPECULARMAP":"",e.specularColorMap?"#define USE_SPECULAR_COLORMAP":"",e.specularIntensityMap?"#define USE_SPECULAR_INTENSITYMAP":"",e.roughnessMap?"#define USE_ROUGHNESSMAP":"",e.metalnessMap?"#define USE_METALNESSMAP":"",e.alphaMap?"#define USE_ALPHAMAP":"",e.alphaTest?"#define USE_ALPHATEST":"",e.alphaHash?"#define USE_ALPHAHASH":"",e.sheen?"#define USE_SHEEN":"",e.sheenColorMap?"#define USE_SHEEN_COLORMAP":"",e.sheenRoughnessMap?"#define USE_SHEEN_ROUGHNESSMAP":"",e.transmission?"#define USE_TRANSMISSION":"",e.transmissionMap?"#define USE_TRANSMISSIONMAP":"",e.thicknessMap?"#define USE_THICKNESSMAP":"",e.vertexTangents&&e.flatShading===!1?"#define USE_TANGENT":"",e.vertexColors||e.instancingColor||e.batchingColor?"#define USE_COLOR":"",e.vertexAlphas?"#define USE_COLOR_ALPHA":"",e.vertexUv1s?"#define USE_UV1":"",e.vertexUv2s?"#define USE_UV2":"",e.vertexUv3s?"#define USE_UV3":"",e.pointsUvs?"#define USE_POINTS_UV":"",e.gradientMap?"#define USE_GRADIENTMAP":"",e.flatShading?"#define FLAT_SHADED":"",e.doubleSided?"#define DOUBLE_SIDED":"",e.flipSided?"#define FLIP_SIDED":"",e.shadowMapEnabled?"#define USE_SHADOWMAP":"",e.shadowMapEnabled?"#define "+l:"",e.premultipliedAlpha?"#define PREMULTIPLIED_ALPHA":"",e.numLightProbes>0?"#define USE_LIGHT_PROBES":"",e.decodeVideoTexture?"#define DECODE_VIDEO_TEXTURE":"",e.decodeVideoTextureEmissive?"#define DECODE_VIDEO_TEXTURE_EMISSIVE":"",e.logarithmicDepthBuffer?"#define USE_LOGDEPTHBUF":"",e.reverseDepthBuffer?"#define USE_REVERSEDEPTHBUF":"","uniform mat4 viewMatrix;","uniform vec3 cameraPosition;","uniform bool isOrthographic;",e.toneMapping!==Rn?"#define TONE_MAPPING":"",e.toneMapping!==Rn?Nt.tonemapping_pars_fragment:"",e.toneMapping!==Rn?Em("toneMapping",e.toneMapping):"",e.dithering?"#define DITHERING":"",e.opaque?"#define OPAQUE":"",Nt.colorspace_pars_fragment,Mm("linearToOutputTexel",e.outputColorSpace),Tm(),e.useDepthPacking?"#define DEPTH_PACKING "+e.depthPacking:"",`
`].filter(Oi).join(`
`)),o=wa(o),o=Qo(o,e),o=tc(o,e),a=wa(a),a=Qo(a,e),a=tc(a,e),o=ec(o),a=ec(a),e.isRawShaderMaterial!==!0&&(P=`#version 300 es
`,g=[_,"#define attribute in","#define varying out","#define texture2D texture"].join(`
`)+`
`+g,d=["#define varying in",e.glslVersion===uo?"":"layout(location = 0) out highp vec4 pc_fragColor;",e.glslVersion===uo?"":"#define gl_FragColor pc_fragColor","#define gl_FragDepthEXT gl_FragDepth","#define texture2D texture","#define textureCube texture","#define texture2DProj textureProj","#define texture2DLodEXT textureLod","#define texture2DProjLodEXT textureProjLod","#define textureCubeLodEXT textureLod","#define texture2DGradEXT textureGrad","#define texture2DProjGradEXT textureProjGrad","#define textureCubeGradEXT textureGrad"].join(`
`)+`
`+d);const b=P+g+o,w=P+d+a,D=Ko(r,r.VERTEX_SHADER,b),L=Ko(r,r.FRAGMENT_SHADER,w);r.attachShader(y,D),r.attachShader(y,L),e.index0AttributeName!==void 0?r.bindAttribLocation(y,0,e.index0AttributeName):e.morphTargets===!0&&r.bindAttribLocation(y,0,"position"),r.linkProgram(y);function I(O){if(n.debug.checkShaderErrors){const $=r.getProgramInfoLog(y).trim(),W=r.getShaderInfoLog(D).trim(),K=r.getShaderInfoLog(L).trim();let it=!0,Z=!0;if(r.getProgramParameter(y,r.LINK_STATUS)===!1)if(it=!1,typeof n.debug.onShaderError=="function")n.debug.onShaderError(r,y,D,L);else{const at=Jo(r,D,"vertex"),Y=Jo(r,L,"fragment");console.error("THREE.WebGLProgram: Shader Error "+r.getError()+" - VALIDATE_STATUS "+r.getProgramParameter(y,r.VALIDATE_STATUS)+`

Material Name: `+O.name+`
Material Type: `+O.type+`

Program Info Log: `+$+`
`+at+`
`+Y)}else $!==""?console.warn("THREE.WebGLProgram: Program Info Log:",$):(W===""||K==="")&&(Z=!1);Z&&(O.diagnostics={runnable:it,programLog:$,vertexShader:{log:W,prefix:g},fragmentShader:{log:K,prefix:d}})}r.deleteShader(D),r.deleteShader(L),N=new Ur(r,y),A=bm(r,y)}let N;this.getUniforms=function(){return N===void 0&&I(this),N};let A;this.getAttributes=function(){return A===void 0&&I(this),A};let T=e.rendererExtensionParallelShaderCompile===!1;return this.isReady=function(){return T===!1&&(T=r.getProgramParameter(y,xm)),T},this.destroy=function(){i.releaseStatesOfProgram(this),r.deleteProgram(y),this.program=void 0},this.type=e.shaderType,this.name=e.shaderName,this.id=vm++,this.cacheKey=t,this.usedTimes=1,this.program=y,this.vertexShader=D,this.fragmentShader=L,this}let zm=0;class km{constructor(){this.shaderCache=new Map,this.materialCache=new Map}update(t){const e=t.vertexShader,i=t.fragmentShader,r=this._getShaderStage(e),s=this._getShaderStage(i),o=this._getShaderCacheForMaterial(t);return o.has(r)===!1&&(o.add(r),r.usedTimes++),o.has(s)===!1&&(o.add(s),s.usedTimes++),this}remove(t){const e=this.materialCache.get(t);for(const i of e)i.usedTimes--,i.usedTimes===0&&this.shaderCache.delete(i.code);return this.materialCache.delete(t),this}getVertexShaderID(t){return this._getShaderStage(t.vertexShader).id}getFragmentShaderID(t){return this._getShaderStage(t.fragmentShader).id}dispose(){this.shaderCache.clear(),this.materialCache.clear()}_getShaderCacheForMaterial(t){const e=this.materialCache;let i=e.get(t);return i===void 0&&(i=new Set,e.set(t,i)),i}_getShaderStage(t){const e=this.shaderCache;let i=e.get(t);return i===void 0&&(i=new Hm(t),e.set(t,i)),i}}class Hm{constructor(t){this.id=zm++,this.code=t,this.usedTimes=0}}function Vm(n,t,e,i,r,s,o){const a=new jc,l=new km,u=new Set,h=[],p=r.logarithmicDepthBuffer,m=r.vertexTextures;let _=r.precision;const x={MeshDepthMaterial:"depth",MeshDistanceMaterial:"distanceRGBA",MeshNormalMaterial:"normal",MeshBasicMaterial:"basic",MeshLambertMaterial:"lambert",MeshPhongMaterial:"phong",MeshToonMaterial:"toon",MeshStandardMaterial:"physical",MeshPhysicalMaterial:"physical",MeshMatcapMaterial:"matcap",LineBasicMaterial:"basic",LineDashedMaterial:"dashed",PointsMaterial:"points",ShadowMaterial:"shadow",SpriteMaterial:"sprite"};function y(A){return u.add(A),A===0?"uv":`uv${A}`}function g(A,T,O,$,W){const K=$.fog,it=W.geometry,Z=A.isMeshStandardMaterial?$.environment:null,at=(A.isMeshStandardMaterial?e:t).get(A.envMap||Z),Y=at&&at.mapping===Gr?at.image.height:null,ft=x[A.type];A.precision!==null&&(_=r.getMaxPrecision(A.precision),_!==A.precision&&console.warn("THREE.WebGLProgram.getParameters:",A.precision,"not supported, using",_,"instead."));const gt=it.morphAttributes.position||it.morphAttributes.normal||it.morphAttributes.color,ht=gt!==void 0?gt.length:0;let bt=0;it.morphAttributes.position!==void 0&&(bt=1),it.morphAttributes.normal!==void 0&&(bt=2),it.morphAttributes.color!==void 0&&(bt=3);let qt,Q,ct,St;if(ft){const jt=je[ft];qt=jt.vertexShader,Q=jt.fragmentShader}else qt=A.vertexShader,Q=A.fragmentShader,l.update(A),ct=l.getVertexShaderID(A),St=l.getFragmentShaderID(A);const ut=n.getRenderTarget(),Tt=n.state.buffers.depth.getReversed(),Bt=W.isInstancedMesh===!0,Pt=W.isBatchedMesh===!0,ne=!!A.map,Jt=!!A.matcap,It=!!at,U=!!A.aoMap,Ee=!!A.lightMap,zt=!!A.bumpMap,Ot=!!A.normalMap,wt=!!A.displacementMap,Zt=!!A.emissiveMap,Et=!!A.metalnessMap,R=!!A.roughnessMap,S=A.anisotropy>0,G=A.clearcoat>0,tt=A.dispersion>0,et=A.iridescence>0,J=A.sheen>0,v=A.transmission>0,c=S&&!!A.anisotropyMap,f=G&&!!A.clearcoatMap,E=G&&!!A.clearcoatNormalMap,C=G&&!!A.clearcoatRoughnessMap,F=et&&!!A.iridescenceMap,V=et&&!!A.iridescenceThicknessMap,ot=J&&!!A.sheenColorMap,st=J&&!!A.sheenRoughnessMap,Mt=!!A.specularMap,vt=!!A.specularColorMap,At=!!A.specularIntensityMap,B=v&&!!A.transmissionMap,pt=v&&!!A.thicknessMap,j=!!A.gradientMap,nt=!!A.alphaMap,_t=A.alphaTest>0,mt=!!A.alphaHash,Ut=!!A.extensions;let ie=Rn;A.toneMapped&&(ut===null||ut.isXRRenderTarget===!0)&&(ie=n.toneMapping);const pe={shaderID:ft,shaderType:A.type,shaderName:A.name,vertexShader:qt,fragmentShader:Q,defines:A.defines,customVertexShaderID:ct,customFragmentShaderID:St,isRawShaderMaterial:A.isRawShaderMaterial===!0,glslVersion:A.glslVersion,precision:_,batching:Pt,batchingColor:Pt&&W._colorsTexture!==null,instancing:Bt,instancingColor:Bt&&W.instanceColor!==null,instancingMorph:Bt&&W.morphTexture!==null,supportsVertexTextures:m,outputColorSpace:ut===null?n.outputColorSpace:ut.isXRRenderTarget===!0?ut.texture.colorSpace:Si,alphaToCoverage:!!A.alphaToCoverage,map:ne,matcap:Jt,envMap:It,envMapMode:It&&at.mapping,envMapCubeUVHeight:Y,aoMap:U,lightMap:Ee,bumpMap:zt,normalMap:Ot,displacementMap:m&&wt,emissiveMap:Zt,normalMapObjectSpace:Ot&&A.normalMapType===Fu,normalMapTangentSpace:Ot&&A.normalMapType===Ba,metalnessMap:Et,roughnessMap:R,anisotropy:S,anisotropyMap:c,clearcoat:G,clearcoatMap:f,clearcoatNormalMap:E,clearcoatRoughnessMap:C,dispersion:tt,iridescence:et,iridescenceMap:F,iridescenceThicknessMap:V,sheen:J,sheenColorMap:ot,sheenRoughnessMap:st,specularMap:Mt,specularColorMap:vt,specularIntensityMap:At,transmission:v,transmissionMap:B,thicknessMap:pt,gradientMap:j,opaque:A.transparent===!1&&A.blending===_i&&A.alphaToCoverage===!1,alphaMap:nt,alphaTest:_t,alphaHash:mt,combine:A.combine,mapUv:ne&&y(A.map.channel),aoMapUv:U&&y(A.aoMap.channel),lightMapUv:Ee&&y(A.lightMap.channel),bumpMapUv:zt&&y(A.bumpMap.channel),normalMapUv:Ot&&y(A.normalMap.channel),displacementMapUv:wt&&y(A.displacementMap.channel),emissiveMapUv:Zt&&y(A.emissiveMap.channel),metalnessMapUv:Et&&y(A.metalnessMap.channel),roughnessMapUv:R&&y(A.roughnessMap.channel),anisotropyMapUv:c&&y(A.anisotropyMap.channel),clearcoatMapUv:f&&y(A.clearcoatMap.channel),clearcoatNormalMapUv:E&&y(A.clearcoatNormalMap.channel),clearcoatRoughnessMapUv:C&&y(A.clearcoatRoughnessMap.channel),iridescenceMapUv:F&&y(A.iridescenceMap.channel),iridescenceThicknessMapUv:V&&y(A.iridescenceThicknessMap.channel),sheenColorMapUv:ot&&y(A.sheenColorMap.channel),sheenRoughnessMapUv:st&&y(A.sheenRoughnessMap.channel),specularMapUv:Mt&&y(A.specularMap.channel),specularColorMapUv:vt&&y(A.specularColorMap.channel),specularIntensityMapUv:At&&y(A.specularIntensityMap.channel),transmissionMapUv:B&&y(A.transmissionMap.channel),thicknessMapUv:pt&&y(A.thicknessMap.channel),alphaMapUv:nt&&y(A.alphaMap.channel),vertexTangents:!!it.attributes.tangent&&(Ot||S),vertexColors:A.vertexColors,vertexAlphas:A.vertexColors===!0&&!!it.attributes.color&&it.attributes.color.itemSize===4,pointsUvs:W.isPoints===!0&&!!it.attributes.uv&&(ne||nt),fog:!!K,useFog:A.fog===!0,fogExp2:!!K&&K.isFogExp2,flatShading:A.flatShading===!0,sizeAttenuation:A.sizeAttenuation===!0,logarithmicDepthBuffer:p,reverseDepthBuffer:Tt,skinning:W.isSkinnedMesh===!0,morphTargets:it.morphAttributes.position!==void 0,morphNormals:it.morphAttributes.normal!==void 0,morphColors:it.morphAttributes.color!==void 0,morphTargetsCount:ht,morphTextureStride:bt,numDirLights:T.directional.length,numPointLights:T.point.length,numSpotLights:T.spot.length,numSpotLightMaps:T.spotLightMap.length,numRectAreaLights:T.rectArea.length,numHemiLights:T.hemi.length,numDirLightShadows:T.directionalShadowMap.length,numPointLightShadows:T.pointShadowMap.length,numSpotLightShadows:T.spotShadowMap.length,numSpotLightShadowsWithMaps:T.numSpotLightShadowsWithMaps,numLightProbes:T.numLightProbes,numClippingPlanes:o.numPlanes,numClipIntersection:o.numIntersection,dithering:A.dithering,shadowMapEnabled:n.shadowMap.enabled&&O.length>0,shadowMapType:n.shadowMap.type,toneMapping:ie,decodeVideoTexture:ne&&A.map.isVideoTexture===!0&&Gt.getTransfer(A.map.colorSpace)===Kt,decodeVideoTextureEmissive:Zt&&A.emissiveMap.isVideoTexture===!0&&Gt.getTransfer(A.emissiveMap.colorSpace)===Kt,premultipliedAlpha:A.premultipliedAlpha,doubleSided:A.side===ve,flipSided:A.side===Ae,useDepthPacking:A.depthPacking>=0,depthPacking:A.depthPacking||0,index0AttributeName:A.index0AttributeName,extensionClipCullDistance:Ut&&A.extensions.clipCullDistance===!0&&i.has("WEBGL_clip_cull_distance"),extensionMultiDraw:(Ut&&A.extensions.multiDraw===!0||Pt)&&i.has("WEBGL_multi_draw"),rendererExtensionParallelShaderCompile:i.has("KHR_parallel_shader_compile"),customProgramCacheKey:A.customProgramCacheKey()};return pe.vertexUv1s=u.has(1),pe.vertexUv2s=u.has(2),pe.vertexUv3s=u.has(3),u.clear(),pe}function d(A){const T=[];if(A.shaderID?T.push(A.shaderID):(T.push(A.customVertexShaderID),T.push(A.customFragmentShaderID)),A.defines!==void 0)for(const O in A.defines)T.push(O),T.push(A.defines[O]);return A.isRawShaderMaterial===!1&&(P(T,A),b(T,A),T.push(n.outputColorSpace)),T.push(A.customProgramCacheKey),T.join()}function P(A,T){A.push(T.precision),A.push(T.outputColorSpace),A.push(T.envMapMode),A.push(T.envMapCubeUVHeight),A.push(T.mapUv),A.push(T.alphaMapUv),A.push(T.lightMapUv),A.push(T.aoMapUv),A.push(T.bumpMapUv),A.push(T.normalMapUv),A.push(T.displacementMapUv),A.push(T.emissiveMapUv),A.push(T.metalnessMapUv),A.push(T.roughnessMapUv),A.push(T.anisotropyMapUv),A.push(T.clearcoatMapUv),A.push(T.clearcoatNormalMapUv),A.push(T.clearcoatRoughnessMapUv),A.push(T.iridescenceMapUv),A.push(T.iridescenceThicknessMapUv),A.push(T.sheenColorMapUv),A.push(T.sheenRoughnessMapUv),A.push(T.specularMapUv),A.push(T.specularColorMapUv),A.push(T.specularIntensityMapUv),A.push(T.transmissionMapUv),A.push(T.thicknessMapUv),A.push(T.combine),A.push(T.fogExp2),A.push(T.sizeAttenuation),A.push(T.morphTargetsCount),A.push(T.morphAttributeCount),A.push(T.numDirLights),A.push(T.numPointLights),A.push(T.numSpotLights),A.push(T.numSpotLightMaps),A.push(T.numHemiLights),A.push(T.numRectAreaLights),A.push(T.numDirLightShadows),A.push(T.numPointLightShadows),A.push(T.numSpotLightShadows),A.push(T.numSpotLightShadowsWithMaps),A.push(T.numLightProbes),A.push(T.shadowMapType),A.push(T.toneMapping),A.push(T.numClippingPlanes),A.push(T.numClipIntersection),A.push(T.depthPacking)}function b(A,T){a.disableAll(),T.supportsVertexTextures&&a.enable(0),T.instancing&&a.enable(1),T.instancingColor&&a.enable(2),T.instancingMorph&&a.enable(3),T.matcap&&a.enable(4),T.envMap&&a.enable(5),T.normalMapObjectSpace&&a.enable(6),T.normalMapTangentSpace&&a.enable(7),T.clearcoat&&a.enable(8),T.iridescence&&a.enable(9),T.alphaTest&&a.enable(10),T.vertexColors&&a.enable(11),T.vertexAlphas&&a.enable(12),T.vertexUv1s&&a.enable(13),T.vertexUv2s&&a.enable(14),T.vertexUv3s&&a.enable(15),T.vertexTangents&&a.enable(16),T.anisotropy&&a.enable(17),T.alphaHash&&a.enable(18),T.batching&&a.enable(19),T.dispersion&&a.enable(20),T.batchingColor&&a.enable(21),A.push(a.mask),a.disableAll(),T.fog&&a.enable(0),T.useFog&&a.enable(1),T.flatShading&&a.enable(2),T.logarithmicDepthBuffer&&a.enable(3),T.reverseDepthBuffer&&a.enable(4),T.skinning&&a.enable(5),T.morphTargets&&a.enable(6),T.morphNormals&&a.enable(7),T.morphColors&&a.enable(8),T.premultipliedAlpha&&a.enable(9),T.shadowMapEnabled&&a.enable(10),T.doubleSided&&a.enable(11),T.flipSided&&a.enable(12),T.useDepthPacking&&a.enable(13),T.dithering&&a.enable(14),T.transmission&&a.enable(15),T.sheen&&a.enable(16),T.opaque&&a.enable(17),T.pointsUvs&&a.enable(18),T.decodeVideoTexture&&a.enable(19),T.decodeVideoTextureEmissive&&a.enable(20),T.alphaToCoverage&&a.enable(21),A.push(a.mask)}function w(A){const T=x[A.type];let O;if(T){const $=je[T];O=df.clone($.uniforms)}else O=A.uniforms;return O}function D(A,T){let O;for(let $=0,W=h.length;$<W;$++){const K=h[$];if(K.cacheKey===T){O=K,++O.usedTimes;break}}return O===void 0&&(O=new Bm(n,T,A,s),h.push(O)),O}function L(A){if(--A.usedTimes===0){const T=h.indexOf(A);h[T]=h[h.length-1],h.pop(),A.destroy()}}function I(A){l.remove(A)}function N(){l.dispose()}return{getParameters:g,getProgramCacheKey:d,getUniforms:w,acquireProgram:D,releaseProgram:L,releaseShaderCache:I,programs:h,dispose:N}}function Gm(){let n=new WeakMap;function t(o){return n.has(o)}function e(o){let a=n.get(o);return a===void 0&&(a={},n.set(o,a)),a}function i(o){n.delete(o)}function r(o,a,l){n.get(o)[a]=l}function s(){n=new WeakMap}return{has:t,get:e,remove:i,update:r,dispose:s}}function Wm(n,t){return n.groupOrder!==t.groupOrder?n.groupOrder-t.groupOrder:n.renderOrder!==t.renderOrder?n.renderOrder-t.renderOrder:n.material.id!==t.material.id?n.material.id-t.material.id:n.z!==t.z?n.z-t.z:n.id-t.id}function ic(n,t){return n.groupOrder!==t.groupOrder?n.groupOrder-t.groupOrder:n.renderOrder!==t.renderOrder?n.renderOrder-t.renderOrder:n.z!==t.z?t.z-n.z:n.id-t.id}function rc(){const n=[];let t=0;const e=[],i=[],r=[];function s(){t=0,e.length=0,i.length=0,r.length=0}function o(p,m,_,x,y,g){let d=n[t];return d===void 0?(d={id:p.id,object:p,geometry:m,material:_,groupOrder:x,renderOrder:p.renderOrder,z:y,group:g},n[t]=d):(d.id=p.id,d.object=p,d.geometry=m,d.material=_,d.groupOrder=x,d.renderOrder=p.renderOrder,d.z=y,d.group=g),t++,d}function a(p,m,_,x,y,g){const d=o(p,m,_,x,y,g);_.transmission>0?i.push(d):_.transparent===!0?r.push(d):e.push(d)}function l(p,m,_,x,y,g){const d=o(p,m,_,x,y,g);_.transmission>0?i.unshift(d):_.transparent===!0?r.unshift(d):e.unshift(d)}function u(p,m){e.length>1&&e.sort(p||Wm),i.length>1&&i.sort(m||ic),r.length>1&&r.sort(m||ic)}function h(){for(let p=t,m=n.length;p<m;p++){const _=n[p];if(_.id===null)break;_.id=null,_.object=null,_.geometry=null,_.material=null,_.group=null}}return{opaque:e,transmissive:i,transparent:r,init:s,push:a,unshift:l,finish:h,sort:u}}function Xm(){let n=new WeakMap;function t(i,r){const s=n.get(i);let o;return s===void 0?(o=new rc,n.set(i,[o])):r>=s.length?(o=new rc,s.push(o)):o=s[r],o}function e(){n=new WeakMap}return{get:t,dispose:e}}function qm(){const n={};return{get:function(t){if(n[t.id]!==void 0)return n[t.id];let e;switch(t.type){case"DirectionalLight":e={direction:new H,color:new Wt};break;case"SpotLight":e={position:new H,direction:new H,color:new Wt,distance:0,coneCos:0,penumbraCos:0,decay:0};break;case"PointLight":e={position:new H,color:new Wt,distance:0,decay:0};break;case"HemisphereLight":e={direction:new H,skyColor:new Wt,groundColor:new Wt};break;case"RectAreaLight":e={color:new Wt,position:new H,halfWidth:new H,halfHeight:new H};break}return n[t.id]=e,e}}}function $m(){const n={};return{get:function(t){if(n[t.id]!==void 0)return n[t.id];let e;switch(t.type){case"DirectionalLight":e={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new Xt};break;case"SpotLight":e={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new Xt};break;case"PointLight":e={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new Xt,shadowCameraNear:1,shadowCameraFar:1e3};break}return n[t.id]=e,e}}}let Ym=0;function jm(n,t){return(t.castShadow?2:0)-(n.castShadow?2:0)+(t.map?1:0)-(n.map?1:0)}function Km(n){const t=new qm,e=$m(),i={version:0,hash:{directionalLength:-1,pointLength:-1,spotLength:-1,rectAreaLength:-1,hemiLength:-1,numDirectionalShadows:-1,numPointShadows:-1,numSpotShadows:-1,numSpotMaps:-1,numLightProbes:-1},ambient:[0,0,0],probe:[],directional:[],directionalShadow:[],directionalShadowMap:[],directionalShadowMatrix:[],spot:[],spotLightMap:[],spotShadow:[],spotShadowMap:[],spotLightMatrix:[],rectArea:[],rectAreaLTC1:null,rectAreaLTC2:null,point:[],pointShadow:[],pointShadowMap:[],pointShadowMatrix:[],hemi:[],numSpotLightShadowsWithMaps:0,numLightProbes:0};for(let u=0;u<9;u++)i.probe.push(new H);const r=new H,s=new te,o=new te;function a(u){let h=0,p=0,m=0;for(let A=0;A<9;A++)i.probe[A].set(0,0,0);let _=0,x=0,y=0,g=0,d=0,P=0,b=0,w=0,D=0,L=0,I=0;u.sort(jm);for(let A=0,T=u.length;A<T;A++){const O=u[A],$=O.color,W=O.intensity,K=O.distance,it=O.shadow&&O.shadow.map?O.shadow.map.texture:null;if(O.isAmbientLight)h+=$.r*W,p+=$.g*W,m+=$.b*W;else if(O.isLightProbe){for(let Z=0;Z<9;Z++)i.probe[Z].addScaledVector(O.sh.coefficients[Z],W);I++}else if(O.isDirectionalLight){const Z=t.get(O);if(Z.color.copy(O.color).multiplyScalar(O.intensity),O.castShadow){const at=O.shadow,Y=e.get(O);Y.shadowIntensity=at.intensity,Y.shadowBias=at.bias,Y.shadowNormalBias=at.normalBias,Y.shadowRadius=at.radius,Y.shadowMapSize=at.mapSize,i.directionalShadow[_]=Y,i.directionalShadowMap[_]=it,i.directionalShadowMatrix[_]=O.shadow.matrix,P++}i.directional[_]=Z,_++}else if(O.isSpotLight){const Z=t.get(O);Z.position.setFromMatrixPosition(O.matrixWorld),Z.color.copy($).multiplyScalar(W),Z.distance=K,Z.coneCos=Math.cos(O.angle),Z.penumbraCos=Math.cos(O.angle*(1-O.penumbra)),Z.decay=O.decay,i.spot[y]=Z;const at=O.shadow;if(O.map&&(i.spotLightMap[D]=O.map,D++,at.updateMatrices(O),O.castShadow&&L++),i.spotLightMatrix[y]=at.matrix,O.castShadow){const Y=e.get(O);Y.shadowIntensity=at.intensity,Y.shadowBias=at.bias,Y.shadowNormalBias=at.normalBias,Y.shadowRadius=at.radius,Y.shadowMapSize=at.mapSize,i.spotShadow[y]=Y,i.spotShadowMap[y]=it,w++}y++}else if(O.isRectAreaLight){const Z=t.get(O);Z.color.copy($).multiplyScalar(W),Z.halfWidth.set(O.width*.5,0,0),Z.halfHeight.set(0,O.height*.5,0),i.rectArea[g]=Z,g++}else if(O.isPointLight){const Z=t.get(O);if(Z.color.copy(O.color).multiplyScalar(O.intensity),Z.distance=O.distance,Z.decay=O.decay,O.castShadow){const at=O.shadow,Y=e.get(O);Y.shadowIntensity=at.intensity,Y.shadowBias=at.bias,Y.shadowNormalBias=at.normalBias,Y.shadowRadius=at.radius,Y.shadowMapSize=at.mapSize,Y.shadowCameraNear=at.camera.near,Y.shadowCameraFar=at.camera.far,i.pointShadow[x]=Y,i.pointShadowMap[x]=it,i.pointShadowMatrix[x]=O.shadow.matrix,b++}i.point[x]=Z,x++}else if(O.isHemisphereLight){const Z=t.get(O);Z.skyColor.copy(O.color).multiplyScalar(W),Z.groundColor.copy(O.groundColor).multiplyScalar(W),i.hemi[d]=Z,d++}}g>0&&(n.has("OES_texture_float_linear")===!0?(i.rectAreaLTC1=lt.LTC_FLOAT_1,i.rectAreaLTC2=lt.LTC_FLOAT_2):(i.rectAreaLTC1=lt.LTC_HALF_1,i.rectAreaLTC2=lt.LTC_HALF_2)),i.ambient[0]=h,i.ambient[1]=p,i.ambient[2]=m;const N=i.hash;(N.directionalLength!==_||N.pointLength!==x||N.spotLength!==y||N.rectAreaLength!==g||N.hemiLength!==d||N.numDirectionalShadows!==P||N.numPointShadows!==b||N.numSpotShadows!==w||N.numSpotMaps!==D||N.numLightProbes!==I)&&(i.directional.length=_,i.spot.length=y,i.rectArea.length=g,i.point.length=x,i.hemi.length=d,i.directionalShadow.length=P,i.directionalShadowMap.length=P,i.pointShadow.length=b,i.pointShadowMap.length=b,i.spotShadow.length=w,i.spotShadowMap.length=w,i.directionalShadowMatrix.length=P,i.pointShadowMatrix.length=b,i.spotLightMatrix.length=w+D-L,i.spotLightMap.length=D,i.numSpotLightShadowsWithMaps=L,i.numLightProbes=I,N.directionalLength=_,N.pointLength=x,N.spotLength=y,N.rectAreaLength=g,N.hemiLength=d,N.numDirectionalShadows=P,N.numPointShadows=b,N.numSpotShadows=w,N.numSpotMaps=D,N.numLightProbes=I,i.version=Ym++)}function l(u,h){let p=0,m=0,_=0,x=0,y=0;const g=h.matrixWorldInverse;for(let d=0,P=u.length;d<P;d++){const b=u[d];if(b.isDirectionalLight){const w=i.directional[p];w.direction.setFromMatrixPosition(b.matrixWorld),r.setFromMatrixPosition(b.target.matrixWorld),w.direction.sub(r),w.direction.transformDirection(g),p++}else if(b.isSpotLight){const w=i.spot[_];w.position.setFromMatrixPosition(b.matrixWorld),w.position.applyMatrix4(g),w.direction.setFromMatrixPosition(b.matrixWorld),r.setFromMatrixPosition(b.target.matrixWorld),w.direction.sub(r),w.direction.transformDirection(g),_++}else if(b.isRectAreaLight){const w=i.rectArea[x];w.position.setFromMatrixPosition(b.matrixWorld),w.position.applyMatrix4(g),o.identity(),s.copy(b.matrixWorld),s.premultiply(g),o.extractRotation(s),w.halfWidth.set(b.width*.5,0,0),w.halfHeight.set(0,b.height*.5,0),w.halfWidth.applyMatrix4(o),w.halfHeight.applyMatrix4(o),x++}else if(b.isPointLight){const w=i.point[m];w.position.setFromMatrixPosition(b.matrixWorld),w.position.applyMatrix4(g),m++}else if(b.isHemisphereLight){const w=i.hemi[y];w.direction.setFromMatrixPosition(b.matrixWorld),w.direction.transformDirection(g),y++}}}return{setup:a,setupView:l,state:i}}function sc(n){const t=new Km(n),e=[],i=[];function r(h){u.camera=h,e.length=0,i.length=0}function s(h){e.push(h)}function o(h){i.push(h)}function a(){t.setup(e)}function l(h){t.setupView(e,h)}const u={lightsArray:e,shadowsArray:i,camera:null,lights:t,transmissionRenderTarget:{}};return{init:r,state:u,setupLights:a,setupLightsView:l,pushLight:s,pushShadow:o}}function Zm(n){let t=new WeakMap;function e(r,s=0){const o=t.get(r);let a;return o===void 0?(a=new sc(n),t.set(r,[a])):s>=o.length?(a=new sc(n),o.push(a)):a=o[s],a}function i(){t=new WeakMap}return{get:e,dispose:i}}const Jm=`void main() {
	gl_Position = vec4( position, 1.0 );
}`,Qm=`uniform sampler2D shadow_pass;
uniform vec2 resolution;
uniform float radius;
#include <packing>
void main() {
	const float samples = float( VSM_SAMPLES );
	float mean = 0.0;
	float squared_mean = 0.0;
	float uvStride = samples <= 1.0 ? 0.0 : 2.0 / ( samples - 1.0 );
	float uvStart = samples <= 1.0 ? 0.0 : - 1.0;
	for ( float i = 0.0; i < samples; i ++ ) {
		float uvOffset = uvStart + i * uvStride;
		#ifdef HORIZONTAL_PASS
			vec2 distribution = unpackRGBATo2Half( texture2D( shadow_pass, ( gl_FragCoord.xy + vec2( uvOffset, 0.0 ) * radius ) / resolution ) );
			mean += distribution.x;
			squared_mean += distribution.y * distribution.y + distribution.x * distribution.x;
		#else
			float depth = unpackRGBAToDepth( texture2D( shadow_pass, ( gl_FragCoord.xy + vec2( 0.0, uvOffset ) * radius ) / resolution ) );
			mean += depth;
			squared_mean += depth * depth;
		#endif
	}
	mean = mean / samples;
	squared_mean = squared_mean / samples;
	float std_dev = sqrt( squared_mean - mean * mean );
	gl_FragColor = pack2HalfToRGBA( vec2( mean, std_dev ) );
}`;function t0(n,t,e){let i=new ka;const r=new Xt,s=new Xt,o=new se,a=new Ef({depthPacking:Uu}),l=new Tf,u={},h=e.maxTextureSize,p={[Pn]:Ae,[Ae]:Pn,[ve]:ve},m=new Ln({defines:{VSM_SAMPLES:8},uniforms:{shadow_pass:{value:null},resolution:{value:new Xt},radius:{value:4}},vertexShader:Jm,fragmentShader:Qm}),_=m.clone();_.defines.HORIZONTAL_PASS=1;const x=new Le;x.setAttribute("position",new $e(new Float32Array([-1,-1,.5,3,-1,.5,-1,3,.5]),3));const y=new ce(x,m),g=this;this.enabled=!1,this.autoUpdate=!0,this.needsUpdate=!1,this.type=Uc;let d=this.type;this.render=function(L,I,N){if(g.enabled===!1||g.autoUpdate===!1&&g.needsUpdate===!1||L.length===0)return;const A=n.getRenderTarget(),T=n.getActiveCubeFace(),O=n.getActiveMipmapLevel(),$=n.state;$.setBlending(Cn),$.buffers.color.setClear(1,1,1,1),$.buffers.depth.setTest(!0),$.setScissorTest(!1);const W=d!==an&&this.type===an,K=d===an&&this.type!==an;for(let it=0,Z=L.length;it<Z;it++){const at=L[it],Y=at.shadow;if(Y===void 0){console.warn("THREE.WebGLShadowMap:",at,"has no shadow.");continue}if(Y.autoUpdate===!1&&Y.needsUpdate===!1)continue;r.copy(Y.mapSize);const ft=Y.getFrameExtents();if(r.multiply(ft),s.copy(Y.mapSize),(r.x>h||r.y>h)&&(r.x>h&&(s.x=Math.floor(h/ft.x),r.x=s.x*ft.x,Y.mapSize.x=s.x),r.y>h&&(s.y=Math.floor(h/ft.y),r.y=s.y*ft.y,Y.mapSize.y=s.y)),Y.map===null||W===!0||K===!0){const ht=this.type!==an?{minFilter:qe,magFilter:qe}:{};Y.map!==null&&Y.map.dispose(),Y.map=new jn(r.x,r.y,ht),Y.map.texture.name=at.name+".shadowMap",Y.camera.updateProjectionMatrix()}n.setRenderTarget(Y.map),n.clear();const gt=Y.getViewportCount();for(let ht=0;ht<gt;ht++){const bt=Y.getViewport(ht);o.set(s.x*bt.x,s.y*bt.y,s.x*bt.z,s.y*bt.w),$.viewport(o),Y.updateMatrices(at,ht),i=Y.getFrustum(),w(I,N,Y.camera,at,this.type)}Y.isPointLightShadow!==!0&&this.type===an&&P(Y,N),Y.needsUpdate=!1}d=this.type,g.needsUpdate=!1,n.setRenderTarget(A,T,O)};function P(L,I){const N=t.update(y);m.defines.VSM_SAMPLES!==L.blurSamples&&(m.defines.VSM_SAMPLES=L.blurSamples,_.defines.VSM_SAMPLES=L.blurSamples,m.needsUpdate=!0,_.needsUpdate=!0),L.mapPass===null&&(L.mapPass=new jn(r.x,r.y)),m.uniforms.shadow_pass.value=L.map.texture,m.uniforms.resolution.value=L.mapSize,m.uniforms.radius.value=L.radius,n.setRenderTarget(L.mapPass),n.clear(),n.renderBufferDirect(I,null,N,m,y,null),_.uniforms.shadow_pass.value=L.mapPass.texture,_.uniforms.resolution.value=L.mapSize,_.uniforms.radius.value=L.radius,n.setRenderTarget(L.map),n.clear(),n.renderBufferDirect(I,null,N,_,y,null)}function b(L,I,N,A){let T=null;const O=N.isPointLight===!0?L.customDistanceMaterial:L.customDepthMaterial;if(O!==void 0)T=O;else if(T=N.isPointLight===!0?l:a,n.localClippingEnabled&&I.clipShadows===!0&&Array.isArray(I.clippingPlanes)&&I.clippingPlanes.length!==0||I.displacementMap&&I.displacementScale!==0||I.alphaMap&&I.alphaTest>0||I.map&&I.alphaTest>0||I.alphaToCoverage===!0){const $=T.uuid,W=I.uuid;let K=u[$];K===void 0&&(K={},u[$]=K);let it=K[W];it===void 0&&(it=T.clone(),K[W]=it,I.addEventListener("dispose",D)),T=it}if(T.visible=I.visible,T.wireframe=I.wireframe,A===an?T.side=I.shadowSide!==null?I.shadowSide:I.side:T.side=I.shadowSide!==null?I.shadowSide:p[I.side],T.alphaMap=I.alphaMap,T.alphaTest=I.alphaToCoverage===!0?.5:I.alphaTest,T.map=I.map,T.clipShadows=I.clipShadows,T.clippingPlanes=I.clippingPlanes,T.clipIntersection=I.clipIntersection,T.displacementMap=I.displacementMap,T.displacementScale=I.displacementScale,T.displacementBias=I.displacementBias,T.wireframeLinewidth=I.wireframeLinewidth,T.linewidth=I.linewidth,N.isPointLight===!0&&T.isMeshDistanceMaterial===!0){const $=n.properties.get(T);$.light=N}return T}function w(L,I,N,A,T){if(L.visible===!1)return;if(L.layers.test(I.layers)&&(L.isMesh||L.isLine||L.isPoints)&&(L.castShadow||L.receiveShadow&&T===an)&&(!L.frustumCulled||i.intersectsObject(L))){L.modelViewMatrix.multiplyMatrices(N.matrixWorldInverse,L.matrixWorld);const W=t.update(L),K=L.material;if(Array.isArray(K)){const it=W.groups;for(let Z=0,at=it.length;Z<at;Z++){const Y=it[Z],ft=K[Y.materialIndex];if(ft&&ft.visible){const gt=b(L,ft,A,T);L.onBeforeShadow(n,L,I,N,W,gt,Y),n.renderBufferDirect(N,null,W,gt,L,Y),L.onAfterShadow(n,L,I,N,W,gt,Y)}}}else if(K.visible){const it=b(L,K,A,T);L.onBeforeShadow(n,L,I,N,W,it,null),n.renderBufferDirect(N,null,W,it,L,null),L.onAfterShadow(n,L,I,N,W,it,null)}}const $=L.children;for(let W=0,K=$.length;W<K;W++)w($[W],I,N,A,T)}function D(L){L.target.removeEventListener("dispose",D);for(const N in u){const A=u[N],T=L.target.uuid;T in A&&(A[T].dispose(),delete A[T])}}}const e0={[Hs]:Vs,[Gs]:qs,[Ws]:$s,[xi]:Xs,[Vs]:Hs,[qs]:Gs,[$s]:Ws,[Xs]:xi};function n0(n,t){function e(){let B=!1;const pt=new se;let j=null;const nt=new se(0,0,0,0);return{setMask:function(_t){j!==_t&&!B&&(n.colorMask(_t,_t,_t,_t),j=_t)},setLocked:function(_t){B=_t},setClear:function(_t,mt,Ut,ie,pe){pe===!0&&(_t*=ie,mt*=ie,Ut*=ie),pt.set(_t,mt,Ut,ie),nt.equals(pt)===!1&&(n.clearColor(_t,mt,Ut,ie),nt.copy(pt))},reset:function(){B=!1,j=null,nt.set(-1,0,0,0)}}}function i(){let B=!1,pt=!1,j=null,nt=null,_t=null;return{setReversed:function(mt){if(pt!==mt){const Ut=t.get("EXT_clip_control");mt?Ut.clipControlEXT(Ut.LOWER_LEFT_EXT,Ut.ZERO_TO_ONE_EXT):Ut.clipControlEXT(Ut.LOWER_LEFT_EXT,Ut.NEGATIVE_ONE_TO_ONE_EXT),pt=mt;const ie=_t;_t=null,this.setClear(ie)}},getReversed:function(){return pt},setTest:function(mt){mt?ut(n.DEPTH_TEST):Tt(n.DEPTH_TEST)},setMask:function(mt){j!==mt&&!B&&(n.depthMask(mt),j=mt)},setFunc:function(mt){if(pt&&(mt=e0[mt]),nt!==mt){switch(mt){case Hs:n.depthFunc(n.NEVER);break;case Vs:n.depthFunc(n.ALWAYS);break;case Gs:n.depthFunc(n.LESS);break;case xi:n.depthFunc(n.LEQUAL);break;case Ws:n.depthFunc(n.EQUAL);break;case Xs:n.depthFunc(n.GEQUAL);break;case qs:n.depthFunc(n.GREATER);break;case $s:n.depthFunc(n.NOTEQUAL);break;default:n.depthFunc(n.LEQUAL)}nt=mt}},setLocked:function(mt){B=mt},setClear:function(mt){_t!==mt&&(pt&&(mt=1-mt),n.clearDepth(mt),_t=mt)},reset:function(){B=!1,j=null,nt=null,_t=null,pt=!1}}}function r(){let B=!1,pt=null,j=null,nt=null,_t=null,mt=null,Ut=null,ie=null,pe=null;return{setTest:function(jt){B||(jt?ut(n.STENCIL_TEST):Tt(n.STENCIL_TEST))},setMask:function(jt){pt!==jt&&!B&&(n.stencilMask(jt),pt=jt)},setFunc:function(jt,Oe,Qe){(j!==jt||nt!==Oe||_t!==Qe)&&(n.stencilFunc(jt,Oe,Qe),j=jt,nt=Oe,_t=Qe)},setOp:function(jt,Oe,Qe){(mt!==jt||Ut!==Oe||ie!==Qe)&&(n.stencilOp(jt,Oe,Qe),mt=jt,Ut=Oe,ie=Qe)},setLocked:function(jt){B=jt},setClear:function(jt){pe!==jt&&(n.clearStencil(jt),pe=jt)},reset:function(){B=!1,pt=null,j=null,nt=null,_t=null,mt=null,Ut=null,ie=null,pe=null}}}const s=new e,o=new i,a=new r,l=new WeakMap,u=new WeakMap;let h={},p={},m=new WeakMap,_=[],x=null,y=!1,g=null,d=null,P=null,b=null,w=null,D=null,L=null,I=new Wt(0,0,0),N=0,A=!1,T=null,O=null,$=null,W=null,K=null;const it=n.getParameter(n.MAX_COMBINED_TEXTURE_IMAGE_UNITS);let Z=!1,at=0;const Y=n.getParameter(n.VERSION);Y.indexOf("WebGL")!==-1?(at=parseFloat(/^WebGL (\d)/.exec(Y)[1]),Z=at>=1):Y.indexOf("OpenGL ES")!==-1&&(at=parseFloat(/^OpenGL ES (\d)/.exec(Y)[1]),Z=at>=2);let ft=null,gt={};const ht=n.getParameter(n.SCISSOR_BOX),bt=n.getParameter(n.VIEWPORT),qt=new se().fromArray(ht),Q=new se().fromArray(bt);function ct(B,pt,j,nt){const _t=new Uint8Array(4),mt=n.createTexture();n.bindTexture(B,mt),n.texParameteri(B,n.TEXTURE_MIN_FILTER,n.NEAREST),n.texParameteri(B,n.TEXTURE_MAG_FILTER,n.NEAREST);for(let Ut=0;Ut<j;Ut++)B===n.TEXTURE_3D||B===n.TEXTURE_2D_ARRAY?n.texImage3D(pt,0,n.RGBA,1,1,nt,0,n.RGBA,n.UNSIGNED_BYTE,_t):n.texImage2D(pt+Ut,0,n.RGBA,1,1,0,n.RGBA,n.UNSIGNED_BYTE,_t);return mt}const St={};St[n.TEXTURE_2D]=ct(n.TEXTURE_2D,n.TEXTURE_2D,1),St[n.TEXTURE_CUBE_MAP]=ct(n.TEXTURE_CUBE_MAP,n.TEXTURE_CUBE_MAP_POSITIVE_X,6),St[n.TEXTURE_2D_ARRAY]=ct(n.TEXTURE_2D_ARRAY,n.TEXTURE_2D_ARRAY,1,1),St[n.TEXTURE_3D]=ct(n.TEXTURE_3D,n.TEXTURE_3D,1,1),s.setClear(0,0,0,1),o.setClear(1),a.setClear(0),ut(n.DEPTH_TEST),o.setFunc(xi),zt(!1),Ot(ro),ut(n.CULL_FACE),U(Cn);function ut(B){h[B]!==!0&&(n.enable(B),h[B]=!0)}function Tt(B){h[B]!==!1&&(n.disable(B),h[B]=!1)}function Bt(B,pt){return p[B]!==pt?(n.bindFramebuffer(B,pt),p[B]=pt,B===n.DRAW_FRAMEBUFFER&&(p[n.FRAMEBUFFER]=pt),B===n.FRAMEBUFFER&&(p[n.DRAW_FRAMEBUFFER]=pt),!0):!1}function Pt(B,pt){let j=_,nt=!1;if(B){j=m.get(pt),j===void 0&&(j=[],m.set(pt,j));const _t=B.textures;if(j.length!==_t.length||j[0]!==n.COLOR_ATTACHMENT0){for(let mt=0,Ut=_t.length;mt<Ut;mt++)j[mt]=n.COLOR_ATTACHMENT0+mt;j.length=_t.length,nt=!0}}else j[0]!==n.BACK&&(j[0]=n.BACK,nt=!0);nt&&n.drawBuffers(j)}function ne(B){return x!==B?(n.useProgram(B),x=B,!0):!1}const Jt={[Wn]:n.FUNC_ADD,[au]:n.FUNC_SUBTRACT,[ou]:n.FUNC_REVERSE_SUBTRACT};Jt[cu]=n.MIN,Jt[lu]=n.MAX;const It={[uu]:n.ZERO,[fu]:n.ONE,[hu]:n.SRC_COLOR,[zs]:n.SRC_ALPHA,[xu]:n.SRC_ALPHA_SATURATE,[_u]:n.DST_COLOR,[pu]:n.DST_ALPHA,[du]:n.ONE_MINUS_SRC_COLOR,[ks]:n.ONE_MINUS_SRC_ALPHA,[gu]:n.ONE_MINUS_DST_COLOR,[mu]:n.ONE_MINUS_DST_ALPHA,[vu]:n.CONSTANT_COLOR,[yu]:n.ONE_MINUS_CONSTANT_COLOR,[Su]:n.CONSTANT_ALPHA,[Mu]:n.ONE_MINUS_CONSTANT_ALPHA};function U(B,pt,j,nt,_t,mt,Ut,ie,pe,jt){if(B===Cn){y===!0&&(Tt(n.BLEND),y=!1);return}if(y===!1&&(ut(n.BLEND),y=!0),B!==su){if(B!==g||jt!==A){if((d!==Wn||w!==Wn)&&(n.blendEquation(n.FUNC_ADD),d=Wn,w=Wn),jt)switch(B){case _i:n.blendFuncSeparate(n.ONE,n.ONE_MINUS_SRC_ALPHA,n.ONE,n.ONE_MINUS_SRC_ALPHA);break;case so:n.blendFunc(n.ONE,n.ONE);break;case ao:n.blendFuncSeparate(n.ZERO,n.ONE_MINUS_SRC_COLOR,n.ZERO,n.ONE);break;case oo:n.blendFuncSeparate(n.ZERO,n.SRC_COLOR,n.ZERO,n.SRC_ALPHA);break;default:console.error("THREE.WebGLState: Invalid blending: ",B);break}else switch(B){case _i:n.blendFuncSeparate(n.SRC_ALPHA,n.ONE_MINUS_SRC_ALPHA,n.ONE,n.ONE_MINUS_SRC_ALPHA);break;case so:n.blendFunc(n.SRC_ALPHA,n.ONE);break;case ao:n.blendFuncSeparate(n.ZERO,n.ONE_MINUS_SRC_COLOR,n.ZERO,n.ONE);break;case oo:n.blendFunc(n.ZERO,n.SRC_COLOR);break;default:console.error("THREE.WebGLState: Invalid blending: ",B);break}P=null,b=null,D=null,L=null,I.set(0,0,0),N=0,g=B,A=jt}return}_t=_t||pt,mt=mt||j,Ut=Ut||nt,(pt!==d||_t!==w)&&(n.blendEquationSeparate(Jt[pt],Jt[_t]),d=pt,w=_t),(j!==P||nt!==b||mt!==D||Ut!==L)&&(n.blendFuncSeparate(It[j],It[nt],It[mt],It[Ut]),P=j,b=nt,D=mt,L=Ut),(ie.equals(I)===!1||pe!==N)&&(n.blendColor(ie.r,ie.g,ie.b,pe),I.copy(ie),N=pe),g=B,A=!1}function Ee(B,pt){B.side===ve?Tt(n.CULL_FACE):ut(n.CULL_FACE);let j=B.side===Ae;pt&&(j=!j),zt(j),B.blending===_i&&B.transparent===!1?U(Cn):U(B.blending,B.blendEquation,B.blendSrc,B.blendDst,B.blendEquationAlpha,B.blendSrcAlpha,B.blendDstAlpha,B.blendColor,B.blendAlpha,B.premultipliedAlpha),o.setFunc(B.depthFunc),o.setTest(B.depthTest),o.setMask(B.depthWrite),s.setMask(B.colorWrite);const nt=B.stencilWrite;a.setTest(nt),nt&&(a.setMask(B.stencilWriteMask),a.setFunc(B.stencilFunc,B.stencilRef,B.stencilFuncMask),a.setOp(B.stencilFail,B.stencilZFail,B.stencilZPass)),Zt(B.polygonOffset,B.polygonOffsetFactor,B.polygonOffsetUnits),B.alphaToCoverage===!0?ut(n.SAMPLE_ALPHA_TO_COVERAGE):Tt(n.SAMPLE_ALPHA_TO_COVERAGE)}function zt(B){T!==B&&(B?n.frontFace(n.CW):n.frontFace(n.CCW),T=B)}function Ot(B){B!==nu?(ut(n.CULL_FACE),B!==O&&(B===ro?n.cullFace(n.BACK):B===iu?n.cullFace(n.FRONT):n.cullFace(n.FRONT_AND_BACK))):Tt(n.CULL_FACE),O=B}function wt(B){B!==$&&(Z&&n.lineWidth(B),$=B)}function Zt(B,pt,j){B?(ut(n.POLYGON_OFFSET_FILL),(W!==pt||K!==j)&&(n.polygonOffset(pt,j),W=pt,K=j)):Tt(n.POLYGON_OFFSET_FILL)}function Et(B){B?ut(n.SCISSOR_TEST):Tt(n.SCISSOR_TEST)}function R(B){B===void 0&&(B=n.TEXTURE0+it-1),ft!==B&&(n.activeTexture(B),ft=B)}function S(B,pt,j){j===void 0&&(ft===null?j=n.TEXTURE0+it-1:j=ft);let nt=gt[j];nt===void 0&&(nt={type:void 0,texture:void 0},gt[j]=nt),(nt.type!==B||nt.texture!==pt)&&(ft!==j&&(n.activeTexture(j),ft=j),n.bindTexture(B,pt||St[B]),nt.type=B,nt.texture=pt)}function G(){const B=gt[ft];B!==void 0&&B.type!==void 0&&(n.bindTexture(B.type,null),B.type=void 0,B.texture=void 0)}function tt(){try{n.compressedTexImage2D(...arguments)}catch(B){console.error("THREE.WebGLState:",B)}}function et(){try{n.compressedTexImage3D(...arguments)}catch(B){console.error("THREE.WebGLState:",B)}}function J(){try{n.texSubImage2D(...arguments)}catch(B){console.error("THREE.WebGLState:",B)}}function v(){try{n.texSubImage3D(...arguments)}catch(B){console.error("THREE.WebGLState:",B)}}function c(){try{n.compressedTexSubImage2D(...arguments)}catch(B){console.error("THREE.WebGLState:",B)}}function f(){try{n.compressedTexSubImage3D(...arguments)}catch(B){console.error("THREE.WebGLState:",B)}}function E(){try{n.texStorage2D(...arguments)}catch(B){console.error("THREE.WebGLState:",B)}}function C(){try{n.texStorage3D(...arguments)}catch(B){console.error("THREE.WebGLState:",B)}}function F(){try{n.texImage2D(...arguments)}catch(B){console.error("THREE.WebGLState:",B)}}function V(){try{n.texImage3D(...arguments)}catch(B){console.error("THREE.WebGLState:",B)}}function ot(B){qt.equals(B)===!1&&(n.scissor(B.x,B.y,B.z,B.w),qt.copy(B))}function st(B){Q.equals(B)===!1&&(n.viewport(B.x,B.y,B.z,B.w),Q.copy(B))}function Mt(B,pt){let j=u.get(pt);j===void 0&&(j=new WeakMap,u.set(pt,j));let nt=j.get(B);nt===void 0&&(nt=n.getUniformBlockIndex(pt,B.name),j.set(B,nt))}function vt(B,pt){const nt=u.get(pt).get(B);l.get(pt)!==nt&&(n.uniformBlockBinding(pt,nt,B.__bindingPointIndex),l.set(pt,nt))}function At(){n.disable(n.BLEND),n.disable(n.CULL_FACE),n.disable(n.DEPTH_TEST),n.disable(n.POLYGON_OFFSET_FILL),n.disable(n.SCISSOR_TEST),n.disable(n.STENCIL_TEST),n.disable(n.SAMPLE_ALPHA_TO_COVERAGE),n.blendEquation(n.FUNC_ADD),n.blendFunc(n.ONE,n.ZERO),n.blendFuncSeparate(n.ONE,n.ZERO,n.ONE,n.ZERO),n.blendColor(0,0,0,0),n.colorMask(!0,!0,!0,!0),n.clearColor(0,0,0,0),n.depthMask(!0),n.depthFunc(n.LESS),o.setReversed(!1),n.clearDepth(1),n.stencilMask(4294967295),n.stencilFunc(n.ALWAYS,0,4294967295),n.stencilOp(n.KEEP,n.KEEP,n.KEEP),n.clearStencil(0),n.cullFace(n.BACK),n.frontFace(n.CCW),n.polygonOffset(0,0),n.activeTexture(n.TEXTURE0),n.bindFramebuffer(n.FRAMEBUFFER,null),n.bindFramebuffer(n.DRAW_FRAMEBUFFER,null),n.bindFramebuffer(n.READ_FRAMEBUFFER,null),n.useProgram(null),n.lineWidth(1),n.scissor(0,0,n.canvas.width,n.canvas.height),n.viewport(0,0,n.canvas.width,n.canvas.height),h={},ft=null,gt={},p={},m=new WeakMap,_=[],x=null,y=!1,g=null,d=null,P=null,b=null,w=null,D=null,L=null,I=new Wt(0,0,0),N=0,A=!1,T=null,O=null,$=null,W=null,K=null,qt.set(0,0,n.canvas.width,n.canvas.height),Q.set(0,0,n.canvas.width,n.canvas.height),s.reset(),o.reset(),a.reset()}return{buffers:{color:s,depth:o,stencil:a},enable:ut,disable:Tt,bindFramebuffer:Bt,drawBuffers:Pt,useProgram:ne,setBlending:U,setMaterial:Ee,setFlipSided:zt,setCullFace:Ot,setLineWidth:wt,setPolygonOffset:Zt,setScissorTest:Et,activeTexture:R,bindTexture:S,unbindTexture:G,compressedTexImage2D:tt,compressedTexImage3D:et,texImage2D:F,texImage3D:V,updateUBOMapping:Mt,uniformBlockBinding:vt,texStorage2D:E,texStorage3D:C,texSubImage2D:J,texSubImage3D:v,compressedTexSubImage2D:c,compressedTexSubImage3D:f,scissor:ot,viewport:st,reset:At}}function i0(n,t,e,i,r,s,o){const a=t.has("WEBGL_multisampled_render_to_texture")?t.get("WEBGL_multisampled_render_to_texture"):null,l=typeof navigator>"u"?!1:/OculusBrowser/g.test(navigator.userAgent),u=new Xt,h=new WeakMap;let p;const m=new WeakMap;let _=!1;try{_=typeof OffscreenCanvas<"u"&&new OffscreenCanvas(1,1).getContext("2d")!==null}catch{}function x(R,S){return _?new OffscreenCanvas(R,S):Br("canvas")}function y(R,S,G){let tt=1;const et=Et(R);if((et.width>G||et.height>G)&&(tt=G/Math.max(et.width,et.height)),tt<1)if(typeof HTMLImageElement<"u"&&R instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&R instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&R instanceof ImageBitmap||typeof VideoFrame<"u"&&R instanceof VideoFrame){const J=Math.floor(tt*et.width),v=Math.floor(tt*et.height);p===void 0&&(p=x(J,v));const c=S?x(J,v):p;return c.width=J,c.height=v,c.getContext("2d").drawImage(R,0,0,J,v),console.warn("THREE.WebGLRenderer: Texture has been resized from ("+et.width+"x"+et.height+") to ("+J+"x"+v+")."),c}else return"data"in R&&console.warn("THREE.WebGLRenderer: Image in DataTexture is too big ("+et.width+"x"+et.height+")."),R;return R}function g(R){return R.generateMipmaps}function d(R){n.generateMipmap(R)}function P(R){return R.isWebGLCubeRenderTarget?n.TEXTURE_CUBE_MAP:R.isWebGL3DRenderTarget?n.TEXTURE_3D:R.isWebGLArrayRenderTarget||R.isCompressedArrayTexture?n.TEXTURE_2D_ARRAY:n.TEXTURE_2D}function b(R,S,G,tt,et=!1){if(R!==null){if(n[R]!==void 0)return n[R];console.warn("THREE.WebGLRenderer: Attempt to use non-existing WebGL internal format '"+R+"'")}let J=S;if(S===n.RED&&(G===n.FLOAT&&(J=n.R32F),G===n.HALF_FLOAT&&(J=n.R16F),G===n.UNSIGNED_BYTE&&(J=n.R8)),S===n.RED_INTEGER&&(G===n.UNSIGNED_BYTE&&(J=n.R8UI),G===n.UNSIGNED_SHORT&&(J=n.R16UI),G===n.UNSIGNED_INT&&(J=n.R32UI),G===n.BYTE&&(J=n.R8I),G===n.SHORT&&(J=n.R16I),G===n.INT&&(J=n.R32I)),S===n.RG&&(G===n.FLOAT&&(J=n.RG32F),G===n.HALF_FLOAT&&(J=n.RG16F),G===n.UNSIGNED_BYTE&&(J=n.RG8)),S===n.RG_INTEGER&&(G===n.UNSIGNED_BYTE&&(J=n.RG8UI),G===n.UNSIGNED_SHORT&&(J=n.RG16UI),G===n.UNSIGNED_INT&&(J=n.RG32UI),G===n.BYTE&&(J=n.RG8I),G===n.SHORT&&(J=n.RG16I),G===n.INT&&(J=n.RG32I)),S===n.RGB_INTEGER&&(G===n.UNSIGNED_BYTE&&(J=n.RGB8UI),G===n.UNSIGNED_SHORT&&(J=n.RGB16UI),G===n.UNSIGNED_INT&&(J=n.RGB32UI),G===n.BYTE&&(J=n.RGB8I),G===n.SHORT&&(J=n.RGB16I),G===n.INT&&(J=n.RGB32I)),S===n.RGBA_INTEGER&&(G===n.UNSIGNED_BYTE&&(J=n.RGBA8UI),G===n.UNSIGNED_SHORT&&(J=n.RGBA16UI),G===n.UNSIGNED_INT&&(J=n.RGBA32UI),G===n.BYTE&&(J=n.RGBA8I),G===n.SHORT&&(J=n.RGBA16I),G===n.INT&&(J=n.RGBA32I)),S===n.RGB&&G===n.UNSIGNED_INT_5_9_9_9_REV&&(J=n.RGB9_E5),S===n.RGBA){const v=et?Nr:Gt.getTransfer(tt);G===n.FLOAT&&(J=n.RGBA32F),G===n.HALF_FLOAT&&(J=n.RGBA16F),G===n.UNSIGNED_BYTE&&(J=v===Kt?n.SRGB8_ALPHA8:n.RGBA8),G===n.UNSIGNED_SHORT_4_4_4_4&&(J=n.RGBA4),G===n.UNSIGNED_SHORT_5_5_5_1&&(J=n.RGB5_A1)}return(J===n.R16F||J===n.R32F||J===n.RG16F||J===n.RG32F||J===n.RGBA16F||J===n.RGBA32F)&&t.get("EXT_color_buffer_float"),J}function w(R,S){let G;return R?S===null||S===Yn||S===Hi?G=n.DEPTH24_STENCIL8:S===cn?G=n.DEPTH32F_STENCIL8:S===ki&&(G=n.DEPTH24_STENCIL8,console.warn("DepthTexture: 16 bit depth attachment is not supported with stencil. Using 24-bit attachment.")):S===null||S===Yn||S===Hi?G=n.DEPTH_COMPONENT24:S===cn?G=n.DEPTH_COMPONENT32F:S===ki&&(G=n.DEPTH_COMPONENT16),G}function D(R,S){return g(R)===!0||R.isFramebufferTexture&&R.minFilter!==qe&&R.minFilter!==Ke?Math.log2(Math.max(S.width,S.height))+1:R.mipmaps!==void 0&&R.mipmaps.length>0?R.mipmaps.length:R.isCompressedTexture&&Array.isArray(R.image)?S.mipmaps.length:1}function L(R){const S=R.target;S.removeEventListener("dispose",L),N(S),S.isVideoTexture&&h.delete(S)}function I(R){const S=R.target;S.removeEventListener("dispose",I),T(S)}function N(R){const S=i.get(R);if(S.__webglInit===void 0)return;const G=R.source,tt=m.get(G);if(tt){const et=tt[S.__cacheKey];et.usedTimes--,et.usedTimes===0&&A(R),Object.keys(tt).length===0&&m.delete(G)}i.remove(R)}function A(R){const S=i.get(R);n.deleteTexture(S.__webglTexture);const G=R.source,tt=m.get(G);delete tt[S.__cacheKey],o.memory.textures--}function T(R){const S=i.get(R);if(R.depthTexture&&(R.depthTexture.dispose(),i.remove(R.depthTexture)),R.isWebGLCubeRenderTarget)for(let tt=0;tt<6;tt++){if(Array.isArray(S.__webglFramebuffer[tt]))for(let et=0;et<S.__webglFramebuffer[tt].length;et++)n.deleteFramebuffer(S.__webglFramebuffer[tt][et]);else n.deleteFramebuffer(S.__webglFramebuffer[tt]);S.__webglDepthbuffer&&n.deleteRenderbuffer(S.__webglDepthbuffer[tt])}else{if(Array.isArray(S.__webglFramebuffer))for(let tt=0;tt<S.__webglFramebuffer.length;tt++)n.deleteFramebuffer(S.__webglFramebuffer[tt]);else n.deleteFramebuffer(S.__webglFramebuffer);if(S.__webglDepthbuffer&&n.deleteRenderbuffer(S.__webglDepthbuffer),S.__webglMultisampledFramebuffer&&n.deleteFramebuffer(S.__webglMultisampledFramebuffer),S.__webglColorRenderbuffer)for(let tt=0;tt<S.__webglColorRenderbuffer.length;tt++)S.__webglColorRenderbuffer[tt]&&n.deleteRenderbuffer(S.__webglColorRenderbuffer[tt]);S.__webglDepthRenderbuffer&&n.deleteRenderbuffer(S.__webglDepthRenderbuffer)}const G=R.textures;for(let tt=0,et=G.length;tt<et;tt++){const J=i.get(G[tt]);J.__webglTexture&&(n.deleteTexture(J.__webglTexture),o.memory.textures--),i.remove(G[tt])}i.remove(R)}let O=0;function $(){O=0}function W(){const R=O;return R>=r.maxTextures&&console.warn("THREE.WebGLTextures: Trying to use "+R+" texture units while this GPU supports only "+r.maxTextures),O+=1,R}function K(R){const S=[];return S.push(R.wrapS),S.push(R.wrapT),S.push(R.wrapR||0),S.push(R.magFilter),S.push(R.minFilter),S.push(R.anisotropy),S.push(R.internalFormat),S.push(R.format),S.push(R.type),S.push(R.generateMipmaps),S.push(R.premultiplyAlpha),S.push(R.flipY),S.push(R.unpackAlignment),S.push(R.colorSpace),S.join()}function it(R,S){const G=i.get(R);if(R.isVideoTexture&&wt(R),R.isRenderTargetTexture===!1&&R.version>0&&G.__version!==R.version){const tt=R.image;if(tt===null)console.warn("THREE.WebGLRenderer: Texture marked for update but no image data found.");else if(tt.complete===!1)console.warn("THREE.WebGLRenderer: Texture marked for update but image is incomplete");else{Q(G,R,S);return}}e.bindTexture(n.TEXTURE_2D,G.__webglTexture,n.TEXTURE0+S)}function Z(R,S){const G=i.get(R);if(R.version>0&&G.__version!==R.version){Q(G,R,S);return}e.bindTexture(n.TEXTURE_2D_ARRAY,G.__webglTexture,n.TEXTURE0+S)}function at(R,S){const G=i.get(R);if(R.version>0&&G.__version!==R.version){Q(G,R,S);return}e.bindTexture(n.TEXTURE_3D,G.__webglTexture,n.TEXTURE0+S)}function Y(R,S){const G=i.get(R);if(R.version>0&&G.__version!==R.version){ct(G,R,S);return}e.bindTexture(n.TEXTURE_CUBE_MAP,G.__webglTexture,n.TEXTURE0+S)}const ft={[Ks]:n.REPEAT,[qn]:n.CLAMP_TO_EDGE,[Zs]:n.MIRRORED_REPEAT},gt={[qe]:n.NEAREST,[Du]:n.NEAREST_MIPMAP_NEAREST,[tr]:n.NEAREST_MIPMAP_LINEAR,[Ke]:n.LINEAR,[es]:n.LINEAR_MIPMAP_NEAREST,[$n]:n.LINEAR_MIPMAP_LINEAR},ht={[Nu]:n.NEVER,[Vu]:n.ALWAYS,[Ou]:n.LESS,[Xc]:n.LEQUAL,[Bu]:n.EQUAL,[Hu]:n.GEQUAL,[zu]:n.GREATER,[ku]:n.NOTEQUAL};function bt(R,S){if(S.type===cn&&t.has("OES_texture_float_linear")===!1&&(S.magFilter===Ke||S.magFilter===es||S.magFilter===tr||S.magFilter===$n||S.minFilter===Ke||S.minFilter===es||S.minFilter===tr||S.minFilter===$n)&&console.warn("THREE.WebGLRenderer: Unable to use linear filtering with floating point textures. OES_texture_float_linear not supported on this device."),n.texParameteri(R,n.TEXTURE_WRAP_S,ft[S.wrapS]),n.texParameteri(R,n.TEXTURE_WRAP_T,ft[S.wrapT]),(R===n.TEXTURE_3D||R===n.TEXTURE_2D_ARRAY)&&n.texParameteri(R,n.TEXTURE_WRAP_R,ft[S.wrapR]),n.texParameteri(R,n.TEXTURE_MAG_FILTER,gt[S.magFilter]),n.texParameteri(R,n.TEXTURE_MIN_FILTER,gt[S.minFilter]),S.compareFunction&&(n.texParameteri(R,n.TEXTURE_COMPARE_MODE,n.COMPARE_REF_TO_TEXTURE),n.texParameteri(R,n.TEXTURE_COMPARE_FUNC,ht[S.compareFunction])),t.has("EXT_texture_filter_anisotropic")===!0){if(S.magFilter===qe||S.minFilter!==tr&&S.minFilter!==$n||S.type===cn&&t.has("OES_texture_float_linear")===!1)return;if(S.anisotropy>1||i.get(S).__currentAnisotropy){const G=t.get("EXT_texture_filter_anisotropic");n.texParameterf(R,G.TEXTURE_MAX_ANISOTROPY_EXT,Math.min(S.anisotropy,r.getMaxAnisotropy())),i.get(S).__currentAnisotropy=S.anisotropy}}}function qt(R,S){let G=!1;R.__webglInit===void 0&&(R.__webglInit=!0,S.addEventListener("dispose",L));const tt=S.source;let et=m.get(tt);et===void 0&&(et={},m.set(tt,et));const J=K(S);if(J!==R.__cacheKey){et[J]===void 0&&(et[J]={texture:n.createTexture(),usedTimes:0},o.memory.textures++,G=!0),et[J].usedTimes++;const v=et[R.__cacheKey];v!==void 0&&(et[R.__cacheKey].usedTimes--,v.usedTimes===0&&A(S)),R.__cacheKey=J,R.__webglTexture=et[J].texture}return G}function Q(R,S,G){let tt=n.TEXTURE_2D;(S.isDataArrayTexture||S.isCompressedArrayTexture)&&(tt=n.TEXTURE_2D_ARRAY),S.isData3DTexture&&(tt=n.TEXTURE_3D);const et=qt(R,S),J=S.source;e.bindTexture(tt,R.__webglTexture,n.TEXTURE0+G);const v=i.get(J);if(J.version!==v.__version||et===!0){e.activeTexture(n.TEXTURE0+G);const c=Gt.getPrimaries(Gt.workingColorSpace),f=S.colorSpace===An?null:Gt.getPrimaries(S.colorSpace),E=S.colorSpace===An||c===f?n.NONE:n.BROWSER_DEFAULT_WEBGL;n.pixelStorei(n.UNPACK_FLIP_Y_WEBGL,S.flipY),n.pixelStorei(n.UNPACK_PREMULTIPLY_ALPHA_WEBGL,S.premultiplyAlpha),n.pixelStorei(n.UNPACK_ALIGNMENT,S.unpackAlignment),n.pixelStorei(n.UNPACK_COLORSPACE_CONVERSION_WEBGL,E);let C=y(S.image,!1,r.maxTextureSize);C=Zt(S,C);const F=s.convert(S.format,S.colorSpace),V=s.convert(S.type);let ot=b(S.internalFormat,F,V,S.colorSpace,S.isVideoTexture);bt(tt,S);let st;const Mt=S.mipmaps,vt=S.isVideoTexture!==!0,At=v.__version===void 0||et===!0,B=J.dataReady,pt=D(S,C);if(S.isDepthTexture)ot=w(S.format===Gi,S.type),At&&(vt?e.texStorage2D(n.TEXTURE_2D,1,ot,C.width,C.height):e.texImage2D(n.TEXTURE_2D,0,ot,C.width,C.height,0,F,V,null));else if(S.isDataTexture)if(Mt.length>0){vt&&At&&e.texStorage2D(n.TEXTURE_2D,pt,ot,Mt[0].width,Mt[0].height);for(let j=0,nt=Mt.length;j<nt;j++)st=Mt[j],vt?B&&e.texSubImage2D(n.TEXTURE_2D,j,0,0,st.width,st.height,F,V,st.data):e.texImage2D(n.TEXTURE_2D,j,ot,st.width,st.height,0,F,V,st.data);S.generateMipmaps=!1}else vt?(At&&e.texStorage2D(n.TEXTURE_2D,pt,ot,C.width,C.height),B&&e.texSubImage2D(n.TEXTURE_2D,0,0,0,C.width,C.height,F,V,C.data)):e.texImage2D(n.TEXTURE_2D,0,ot,C.width,C.height,0,F,V,C.data);else if(S.isCompressedTexture)if(S.isCompressedArrayTexture){vt&&At&&e.texStorage3D(n.TEXTURE_2D_ARRAY,pt,ot,Mt[0].width,Mt[0].height,C.depth);for(let j=0,nt=Mt.length;j<nt;j++)if(st=Mt[j],S.format!==We)if(F!==null)if(vt){if(B)if(S.layerUpdates.size>0){const _t=Fo(st.width,st.height,S.format,S.type);for(const mt of S.layerUpdates){const Ut=st.data.subarray(mt*_t/st.data.BYTES_PER_ELEMENT,(mt+1)*_t/st.data.BYTES_PER_ELEMENT);e.compressedTexSubImage3D(n.TEXTURE_2D_ARRAY,j,0,0,mt,st.width,st.height,1,F,Ut)}S.clearLayerUpdates()}else e.compressedTexSubImage3D(n.TEXTURE_2D_ARRAY,j,0,0,0,st.width,st.height,C.depth,F,st.data)}else e.compressedTexImage3D(n.TEXTURE_2D_ARRAY,j,ot,st.width,st.height,C.depth,0,st.data,0,0);else console.warn("THREE.WebGLRenderer: Attempt to load unsupported compressed texture format in .uploadTexture()");else vt?B&&e.texSubImage3D(n.TEXTURE_2D_ARRAY,j,0,0,0,st.width,st.height,C.depth,F,V,st.data):e.texImage3D(n.TEXTURE_2D_ARRAY,j,ot,st.width,st.height,C.depth,0,F,V,st.data)}else{vt&&At&&e.texStorage2D(n.TEXTURE_2D,pt,ot,Mt[0].width,Mt[0].height);for(let j=0,nt=Mt.length;j<nt;j++)st=Mt[j],S.format!==We?F!==null?vt?B&&e.compressedTexSubImage2D(n.TEXTURE_2D,j,0,0,st.width,st.height,F,st.data):e.compressedTexImage2D(n.TEXTURE_2D,j,ot,st.width,st.height,0,st.data):console.warn("THREE.WebGLRenderer: Attempt to load unsupported compressed texture format in .uploadTexture()"):vt?B&&e.texSubImage2D(n.TEXTURE_2D,j,0,0,st.width,st.height,F,V,st.data):e.texImage2D(n.TEXTURE_2D,j,ot,st.width,st.height,0,F,V,st.data)}else if(S.isDataArrayTexture)if(vt){if(At&&e.texStorage3D(n.TEXTURE_2D_ARRAY,pt,ot,C.width,C.height,C.depth),B)if(S.layerUpdates.size>0){const j=Fo(C.width,C.height,S.format,S.type);for(const nt of S.layerUpdates){const _t=C.data.subarray(nt*j/C.data.BYTES_PER_ELEMENT,(nt+1)*j/C.data.BYTES_PER_ELEMENT);e.texSubImage3D(n.TEXTURE_2D_ARRAY,0,0,0,nt,C.width,C.height,1,F,V,_t)}S.clearLayerUpdates()}else e.texSubImage3D(n.TEXTURE_2D_ARRAY,0,0,0,0,C.width,C.height,C.depth,F,V,C.data)}else e.texImage3D(n.TEXTURE_2D_ARRAY,0,ot,C.width,C.height,C.depth,0,F,V,C.data);else if(S.isData3DTexture)vt?(At&&e.texStorage3D(n.TEXTURE_3D,pt,ot,C.width,C.height,C.depth),B&&e.texSubImage3D(n.TEXTURE_3D,0,0,0,0,C.width,C.height,C.depth,F,V,C.data)):e.texImage3D(n.TEXTURE_3D,0,ot,C.width,C.height,C.depth,0,F,V,C.data);else if(S.isFramebufferTexture){if(At)if(vt)e.texStorage2D(n.TEXTURE_2D,pt,ot,C.width,C.height);else{let j=C.width,nt=C.height;for(let _t=0;_t<pt;_t++)e.texImage2D(n.TEXTURE_2D,_t,ot,j,nt,0,F,V,null),j>>=1,nt>>=1}}else if(Mt.length>0){if(vt&&At){const j=Et(Mt[0]);e.texStorage2D(n.TEXTURE_2D,pt,ot,j.width,j.height)}for(let j=0,nt=Mt.length;j<nt;j++)st=Mt[j],vt?B&&e.texSubImage2D(n.TEXTURE_2D,j,0,0,F,V,st):e.texImage2D(n.TEXTURE_2D,j,ot,F,V,st);S.generateMipmaps=!1}else if(vt){if(At){const j=Et(C);e.texStorage2D(n.TEXTURE_2D,pt,ot,j.width,j.height)}B&&e.texSubImage2D(n.TEXTURE_2D,0,0,0,F,V,C)}else e.texImage2D(n.TEXTURE_2D,0,ot,F,V,C);g(S)&&d(tt),v.__version=J.version,S.onUpdate&&S.onUpdate(S)}R.__version=S.version}function ct(R,S,G){if(S.image.length!==6)return;const tt=qt(R,S),et=S.source;e.bindTexture(n.TEXTURE_CUBE_MAP,R.__webglTexture,n.TEXTURE0+G);const J=i.get(et);if(et.version!==J.__version||tt===!0){e.activeTexture(n.TEXTURE0+G);const v=Gt.getPrimaries(Gt.workingColorSpace),c=S.colorSpace===An?null:Gt.getPrimaries(S.colorSpace),f=S.colorSpace===An||v===c?n.NONE:n.BROWSER_DEFAULT_WEBGL;n.pixelStorei(n.UNPACK_FLIP_Y_WEBGL,S.flipY),n.pixelStorei(n.UNPACK_PREMULTIPLY_ALPHA_WEBGL,S.premultiplyAlpha),n.pixelStorei(n.UNPACK_ALIGNMENT,S.unpackAlignment),n.pixelStorei(n.UNPACK_COLORSPACE_CONVERSION_WEBGL,f);const E=S.isCompressedTexture||S.image[0].isCompressedTexture,C=S.image[0]&&S.image[0].isDataTexture,F=[];for(let nt=0;nt<6;nt++)!E&&!C?F[nt]=y(S.image[nt],!0,r.maxCubemapSize):F[nt]=C?S.image[nt].image:S.image[nt],F[nt]=Zt(S,F[nt]);const V=F[0],ot=s.convert(S.format,S.colorSpace),st=s.convert(S.type),Mt=b(S.internalFormat,ot,st,S.colorSpace),vt=S.isVideoTexture!==!0,At=J.__version===void 0||tt===!0,B=et.dataReady;let pt=D(S,V);bt(n.TEXTURE_CUBE_MAP,S);let j;if(E){vt&&At&&e.texStorage2D(n.TEXTURE_CUBE_MAP,pt,Mt,V.width,V.height);for(let nt=0;nt<6;nt++){j=F[nt].mipmaps;for(let _t=0;_t<j.length;_t++){const mt=j[_t];S.format!==We?ot!==null?vt?B&&e.compressedTexSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+nt,_t,0,0,mt.width,mt.height,ot,mt.data):e.compressedTexImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+nt,_t,Mt,mt.width,mt.height,0,mt.data):console.warn("THREE.WebGLRenderer: Attempt to load unsupported compressed texture format in .setTextureCube()"):vt?B&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+nt,_t,0,0,mt.width,mt.height,ot,st,mt.data):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+nt,_t,Mt,mt.width,mt.height,0,ot,st,mt.data)}}}else{if(j=S.mipmaps,vt&&At){j.length>0&&pt++;const nt=Et(F[0]);e.texStorage2D(n.TEXTURE_CUBE_MAP,pt,Mt,nt.width,nt.height)}for(let nt=0;nt<6;nt++)if(C){vt?B&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+nt,0,0,0,F[nt].width,F[nt].height,ot,st,F[nt].data):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+nt,0,Mt,F[nt].width,F[nt].height,0,ot,st,F[nt].data);for(let _t=0;_t<j.length;_t++){const Ut=j[_t].image[nt].image;vt?B&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+nt,_t+1,0,0,Ut.width,Ut.height,ot,st,Ut.data):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+nt,_t+1,Mt,Ut.width,Ut.height,0,ot,st,Ut.data)}}else{vt?B&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+nt,0,0,0,ot,st,F[nt]):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+nt,0,Mt,ot,st,F[nt]);for(let _t=0;_t<j.length;_t++){const mt=j[_t];vt?B&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+nt,_t+1,0,0,ot,st,mt.image[nt]):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+nt,_t+1,Mt,ot,st,mt.image[nt])}}}g(S)&&d(n.TEXTURE_CUBE_MAP),J.__version=et.version,S.onUpdate&&S.onUpdate(S)}R.__version=S.version}function St(R,S,G,tt,et,J){const v=s.convert(G.format,G.colorSpace),c=s.convert(G.type),f=b(G.internalFormat,v,c,G.colorSpace),E=i.get(S),C=i.get(G);if(C.__renderTarget=S,!E.__hasExternalTextures){const F=Math.max(1,S.width>>J),V=Math.max(1,S.height>>J);et===n.TEXTURE_3D||et===n.TEXTURE_2D_ARRAY?e.texImage3D(et,J,f,F,V,S.depth,0,v,c,null):e.texImage2D(et,J,f,F,V,0,v,c,null)}e.bindFramebuffer(n.FRAMEBUFFER,R),Ot(S)?a.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,tt,et,C.__webglTexture,0,zt(S)):(et===n.TEXTURE_2D||et>=n.TEXTURE_CUBE_MAP_POSITIVE_X&&et<=n.TEXTURE_CUBE_MAP_NEGATIVE_Z)&&n.framebufferTexture2D(n.FRAMEBUFFER,tt,et,C.__webglTexture,J),e.bindFramebuffer(n.FRAMEBUFFER,null)}function ut(R,S,G){if(n.bindRenderbuffer(n.RENDERBUFFER,R),S.depthBuffer){const tt=S.depthTexture,et=tt&&tt.isDepthTexture?tt.type:null,J=w(S.stencilBuffer,et),v=S.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT,c=zt(S);Ot(S)?a.renderbufferStorageMultisampleEXT(n.RENDERBUFFER,c,J,S.width,S.height):G?n.renderbufferStorageMultisample(n.RENDERBUFFER,c,J,S.width,S.height):n.renderbufferStorage(n.RENDERBUFFER,J,S.width,S.height),n.framebufferRenderbuffer(n.FRAMEBUFFER,v,n.RENDERBUFFER,R)}else{const tt=S.textures;for(let et=0;et<tt.length;et++){const J=tt[et],v=s.convert(J.format,J.colorSpace),c=s.convert(J.type),f=b(J.internalFormat,v,c,J.colorSpace),E=zt(S);G&&Ot(S)===!1?n.renderbufferStorageMultisample(n.RENDERBUFFER,E,f,S.width,S.height):Ot(S)?a.renderbufferStorageMultisampleEXT(n.RENDERBUFFER,E,f,S.width,S.height):n.renderbufferStorage(n.RENDERBUFFER,f,S.width,S.height)}}n.bindRenderbuffer(n.RENDERBUFFER,null)}function Tt(R,S){if(S&&S.isWebGLCubeRenderTarget)throw new Error("Depth Texture with cube render targets is not supported");if(e.bindFramebuffer(n.FRAMEBUFFER,R),!(S.depthTexture&&S.depthTexture.isDepthTexture))throw new Error("renderTarget.depthTexture must be an instance of THREE.DepthTexture");const tt=i.get(S.depthTexture);tt.__renderTarget=S,(!tt.__webglTexture||S.depthTexture.image.width!==S.width||S.depthTexture.image.height!==S.height)&&(S.depthTexture.image.width=S.width,S.depthTexture.image.height=S.height,S.depthTexture.needsUpdate=!0),it(S.depthTexture,0);const et=tt.__webglTexture,J=zt(S);if(S.depthTexture.format===Vi)Ot(S)?a.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,n.DEPTH_ATTACHMENT,n.TEXTURE_2D,et,0,J):n.framebufferTexture2D(n.FRAMEBUFFER,n.DEPTH_ATTACHMENT,n.TEXTURE_2D,et,0);else if(S.depthTexture.format===Gi)Ot(S)?a.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,n.DEPTH_STENCIL_ATTACHMENT,n.TEXTURE_2D,et,0,J):n.framebufferTexture2D(n.FRAMEBUFFER,n.DEPTH_STENCIL_ATTACHMENT,n.TEXTURE_2D,et,0);else throw new Error("Unknown depthTexture format")}function Bt(R){const S=i.get(R),G=R.isWebGLCubeRenderTarget===!0;if(S.__boundDepthTexture!==R.depthTexture){const tt=R.depthTexture;if(S.__depthDisposeCallback&&S.__depthDisposeCallback(),tt){const et=()=>{delete S.__boundDepthTexture,delete S.__depthDisposeCallback,tt.removeEventListener("dispose",et)};tt.addEventListener("dispose",et),S.__depthDisposeCallback=et}S.__boundDepthTexture=tt}if(R.depthTexture&&!S.__autoAllocateDepthBuffer){if(G)throw new Error("target.depthTexture not supported in Cube render targets");const tt=R.texture.mipmaps;tt&&tt.length>0?Tt(S.__webglFramebuffer[0],R):Tt(S.__webglFramebuffer,R)}else if(G){S.__webglDepthbuffer=[];for(let tt=0;tt<6;tt++)if(e.bindFramebuffer(n.FRAMEBUFFER,S.__webglFramebuffer[tt]),S.__webglDepthbuffer[tt]===void 0)S.__webglDepthbuffer[tt]=n.createRenderbuffer(),ut(S.__webglDepthbuffer[tt],R,!1);else{const et=R.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT,J=S.__webglDepthbuffer[tt];n.bindRenderbuffer(n.RENDERBUFFER,J),n.framebufferRenderbuffer(n.FRAMEBUFFER,et,n.RENDERBUFFER,J)}}else{const tt=R.texture.mipmaps;if(tt&&tt.length>0?e.bindFramebuffer(n.FRAMEBUFFER,S.__webglFramebuffer[0]):e.bindFramebuffer(n.FRAMEBUFFER,S.__webglFramebuffer),S.__webglDepthbuffer===void 0)S.__webglDepthbuffer=n.createRenderbuffer(),ut(S.__webglDepthbuffer,R,!1);else{const et=R.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT,J=S.__webglDepthbuffer;n.bindRenderbuffer(n.RENDERBUFFER,J),n.framebufferRenderbuffer(n.FRAMEBUFFER,et,n.RENDERBUFFER,J)}}e.bindFramebuffer(n.FRAMEBUFFER,null)}function Pt(R,S,G){const tt=i.get(R);S!==void 0&&St(tt.__webglFramebuffer,R,R.texture,n.COLOR_ATTACHMENT0,n.TEXTURE_2D,0),G!==void 0&&Bt(R)}function ne(R){const S=R.texture,G=i.get(R),tt=i.get(S);R.addEventListener("dispose",I);const et=R.textures,J=R.isWebGLCubeRenderTarget===!0,v=et.length>1;if(v||(tt.__webglTexture===void 0&&(tt.__webglTexture=n.createTexture()),tt.__version=S.version,o.memory.textures++),J){G.__webglFramebuffer=[];for(let c=0;c<6;c++)if(S.mipmaps&&S.mipmaps.length>0){G.__webglFramebuffer[c]=[];for(let f=0;f<S.mipmaps.length;f++)G.__webglFramebuffer[c][f]=n.createFramebuffer()}else G.__webglFramebuffer[c]=n.createFramebuffer()}else{if(S.mipmaps&&S.mipmaps.length>0){G.__webglFramebuffer=[];for(let c=0;c<S.mipmaps.length;c++)G.__webglFramebuffer[c]=n.createFramebuffer()}else G.__webglFramebuffer=n.createFramebuffer();if(v)for(let c=0,f=et.length;c<f;c++){const E=i.get(et[c]);E.__webglTexture===void 0&&(E.__webglTexture=n.createTexture(),o.memory.textures++)}if(R.samples>0&&Ot(R)===!1){G.__webglMultisampledFramebuffer=n.createFramebuffer(),G.__webglColorRenderbuffer=[],e.bindFramebuffer(n.FRAMEBUFFER,G.__webglMultisampledFramebuffer);for(let c=0;c<et.length;c++){const f=et[c];G.__webglColorRenderbuffer[c]=n.createRenderbuffer(),n.bindRenderbuffer(n.RENDERBUFFER,G.__webglColorRenderbuffer[c]);const E=s.convert(f.format,f.colorSpace),C=s.convert(f.type),F=b(f.internalFormat,E,C,f.colorSpace,R.isXRRenderTarget===!0),V=zt(R);n.renderbufferStorageMultisample(n.RENDERBUFFER,V,F,R.width,R.height),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+c,n.RENDERBUFFER,G.__webglColorRenderbuffer[c])}n.bindRenderbuffer(n.RENDERBUFFER,null),R.depthBuffer&&(G.__webglDepthRenderbuffer=n.createRenderbuffer(),ut(G.__webglDepthRenderbuffer,R,!0)),e.bindFramebuffer(n.FRAMEBUFFER,null)}}if(J){e.bindTexture(n.TEXTURE_CUBE_MAP,tt.__webglTexture),bt(n.TEXTURE_CUBE_MAP,S);for(let c=0;c<6;c++)if(S.mipmaps&&S.mipmaps.length>0)for(let f=0;f<S.mipmaps.length;f++)St(G.__webglFramebuffer[c][f],R,S,n.COLOR_ATTACHMENT0,n.TEXTURE_CUBE_MAP_POSITIVE_X+c,f);else St(G.__webglFramebuffer[c],R,S,n.COLOR_ATTACHMENT0,n.TEXTURE_CUBE_MAP_POSITIVE_X+c,0);g(S)&&d(n.TEXTURE_CUBE_MAP),e.unbindTexture()}else if(v){for(let c=0,f=et.length;c<f;c++){const E=et[c],C=i.get(E);e.bindTexture(n.TEXTURE_2D,C.__webglTexture),bt(n.TEXTURE_2D,E),St(G.__webglFramebuffer,R,E,n.COLOR_ATTACHMENT0+c,n.TEXTURE_2D,0),g(E)&&d(n.TEXTURE_2D)}e.unbindTexture()}else{let c=n.TEXTURE_2D;if((R.isWebGL3DRenderTarget||R.isWebGLArrayRenderTarget)&&(c=R.isWebGL3DRenderTarget?n.TEXTURE_3D:n.TEXTURE_2D_ARRAY),e.bindTexture(c,tt.__webglTexture),bt(c,S),S.mipmaps&&S.mipmaps.length>0)for(let f=0;f<S.mipmaps.length;f++)St(G.__webglFramebuffer[f],R,S,n.COLOR_ATTACHMENT0,c,f);else St(G.__webglFramebuffer,R,S,n.COLOR_ATTACHMENT0,c,0);g(S)&&d(c),e.unbindTexture()}R.depthBuffer&&Bt(R)}function Jt(R){const S=R.textures;for(let G=0,tt=S.length;G<tt;G++){const et=S[G];if(g(et)){const J=P(R),v=i.get(et).__webglTexture;e.bindTexture(J,v),d(J),e.unbindTexture()}}}const It=[],U=[];function Ee(R){if(R.samples>0){if(Ot(R)===!1){const S=R.textures,G=R.width,tt=R.height;let et=n.COLOR_BUFFER_BIT;const J=R.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT,v=i.get(R),c=S.length>1;if(c)for(let E=0;E<S.length;E++)e.bindFramebuffer(n.FRAMEBUFFER,v.__webglMultisampledFramebuffer),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+E,n.RENDERBUFFER,null),e.bindFramebuffer(n.FRAMEBUFFER,v.__webglFramebuffer),n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0+E,n.TEXTURE_2D,null,0);e.bindFramebuffer(n.READ_FRAMEBUFFER,v.__webglMultisampledFramebuffer);const f=R.texture.mipmaps;f&&f.length>0?e.bindFramebuffer(n.DRAW_FRAMEBUFFER,v.__webglFramebuffer[0]):e.bindFramebuffer(n.DRAW_FRAMEBUFFER,v.__webglFramebuffer);for(let E=0;E<S.length;E++){if(R.resolveDepthBuffer&&(R.depthBuffer&&(et|=n.DEPTH_BUFFER_BIT),R.stencilBuffer&&R.resolveStencilBuffer&&(et|=n.STENCIL_BUFFER_BIT)),c){n.framebufferRenderbuffer(n.READ_FRAMEBUFFER,n.COLOR_ATTACHMENT0,n.RENDERBUFFER,v.__webglColorRenderbuffer[E]);const C=i.get(S[E]).__webglTexture;n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0,n.TEXTURE_2D,C,0)}n.blitFramebuffer(0,0,G,tt,0,0,G,tt,et,n.NEAREST),l===!0&&(It.length=0,U.length=0,It.push(n.COLOR_ATTACHMENT0+E),R.depthBuffer&&R.resolveDepthBuffer===!1&&(It.push(J),U.push(J),n.invalidateFramebuffer(n.DRAW_FRAMEBUFFER,U)),n.invalidateFramebuffer(n.READ_FRAMEBUFFER,It))}if(e.bindFramebuffer(n.READ_FRAMEBUFFER,null),e.bindFramebuffer(n.DRAW_FRAMEBUFFER,null),c)for(let E=0;E<S.length;E++){e.bindFramebuffer(n.FRAMEBUFFER,v.__webglMultisampledFramebuffer),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+E,n.RENDERBUFFER,v.__webglColorRenderbuffer[E]);const C=i.get(S[E]).__webglTexture;e.bindFramebuffer(n.FRAMEBUFFER,v.__webglFramebuffer),n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0+E,n.TEXTURE_2D,C,0)}e.bindFramebuffer(n.DRAW_FRAMEBUFFER,v.__webglMultisampledFramebuffer)}else if(R.depthBuffer&&R.resolveDepthBuffer===!1&&l){const S=R.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT;n.invalidateFramebuffer(n.DRAW_FRAMEBUFFER,[S])}}}function zt(R){return Math.min(r.maxSamples,R.samples)}function Ot(R){const S=i.get(R);return R.samples>0&&t.has("WEBGL_multisampled_render_to_texture")===!0&&S.__useRenderToTexture!==!1}function wt(R){const S=o.render.frame;h.get(R)!==S&&(h.set(R,S),R.update())}function Zt(R,S){const G=R.colorSpace,tt=R.format,et=R.type;return R.isCompressedTexture===!0||R.isVideoTexture===!0||G!==Si&&G!==An&&(Gt.getTransfer(G)===Kt?(tt!==We||et!==Je)&&console.warn("THREE.WebGLTextures: sRGB encoded textures have to use RGBAFormat and UnsignedByteType."):console.error("THREE.WebGLTextures: Unsupported texture color space:",G)),S}function Et(R){return typeof HTMLImageElement<"u"&&R instanceof HTMLImageElement?(u.width=R.naturalWidth||R.width,u.height=R.naturalHeight||R.height):typeof VideoFrame<"u"&&R instanceof VideoFrame?(u.width=R.displayWidth,u.height=R.displayHeight):(u.width=R.width,u.height=R.height),u}this.allocateTextureUnit=W,this.resetTextureUnits=$,this.setTexture2D=it,this.setTexture2DArray=Z,this.setTexture3D=at,this.setTextureCube=Y,this.rebindTextures=Pt,this.setupRenderTarget=ne,this.updateRenderTargetMipmap=Jt,this.updateMultisampleRenderTarget=Ee,this.setupDepthRenderbuffer=Bt,this.setupFrameBufferTexture=St,this.useMultisampledRTT=Ot}function r0(n,t){function e(i,r=An){let s;const o=Gt.getTransfer(r);if(i===Je)return n.UNSIGNED_BYTE;if(i===Ia)return n.UNSIGNED_SHORT_4_4_4_4;if(i===Ua)return n.UNSIGNED_SHORT_5_5_5_1;if(i===zc)return n.UNSIGNED_INT_5_9_9_9_REV;if(i===Oc)return n.BYTE;if(i===Bc)return n.SHORT;if(i===ki)return n.UNSIGNED_SHORT;if(i===Da)return n.INT;if(i===Yn)return n.UNSIGNED_INT;if(i===cn)return n.FLOAT;if(i===qi)return n.HALF_FLOAT;if(i===kc)return n.ALPHA;if(i===Hc)return n.RGB;if(i===We)return n.RGBA;if(i===Vi)return n.DEPTH_COMPONENT;if(i===Gi)return n.DEPTH_STENCIL;if(i===Vc)return n.RED;if(i===Fa)return n.RED_INTEGER;if(i===Gc)return n.RG;if(i===Na)return n.RG_INTEGER;if(i===Oa)return n.RGBA_INTEGER;if(i===Cr||i===Rr||i===Pr||i===Lr)if(o===Kt)if(s=t.get("WEBGL_compressed_texture_s3tc_srgb"),s!==null){if(i===Cr)return s.COMPRESSED_SRGB_S3TC_DXT1_EXT;if(i===Rr)return s.COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT;if(i===Pr)return s.COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT;if(i===Lr)return s.COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT}else return null;else if(s=t.get("WEBGL_compressed_texture_s3tc"),s!==null){if(i===Cr)return s.COMPRESSED_RGB_S3TC_DXT1_EXT;if(i===Rr)return s.COMPRESSED_RGBA_S3TC_DXT1_EXT;if(i===Pr)return s.COMPRESSED_RGBA_S3TC_DXT3_EXT;if(i===Lr)return s.COMPRESSED_RGBA_S3TC_DXT5_EXT}else return null;if(i===Js||i===Qs||i===ta||i===ea)if(s=t.get("WEBGL_compressed_texture_pvrtc"),s!==null){if(i===Js)return s.COMPRESSED_RGB_PVRTC_4BPPV1_IMG;if(i===Qs)return s.COMPRESSED_RGB_PVRTC_2BPPV1_IMG;if(i===ta)return s.COMPRESSED_RGBA_PVRTC_4BPPV1_IMG;if(i===ea)return s.COMPRESSED_RGBA_PVRTC_2BPPV1_IMG}else return null;if(i===na||i===ia||i===ra)if(s=t.get("WEBGL_compressed_texture_etc"),s!==null){if(i===na||i===ia)return o===Kt?s.COMPRESSED_SRGB8_ETC2:s.COMPRESSED_RGB8_ETC2;if(i===ra)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ETC2_EAC:s.COMPRESSED_RGBA8_ETC2_EAC}else return null;if(i===sa||i===aa||i===oa||i===ca||i===la||i===ua||i===fa||i===ha||i===da||i===pa||i===ma||i===_a||i===ga||i===xa)if(s=t.get("WEBGL_compressed_texture_astc"),s!==null){if(i===sa)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR:s.COMPRESSED_RGBA_ASTC_4x4_KHR;if(i===aa)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR:s.COMPRESSED_RGBA_ASTC_5x4_KHR;if(i===oa)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR:s.COMPRESSED_RGBA_ASTC_5x5_KHR;if(i===ca)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR:s.COMPRESSED_RGBA_ASTC_6x5_KHR;if(i===la)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR:s.COMPRESSED_RGBA_ASTC_6x6_KHR;if(i===ua)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR:s.COMPRESSED_RGBA_ASTC_8x5_KHR;if(i===fa)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR:s.COMPRESSED_RGBA_ASTC_8x6_KHR;if(i===ha)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR:s.COMPRESSED_RGBA_ASTC_8x8_KHR;if(i===da)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR:s.COMPRESSED_RGBA_ASTC_10x5_KHR;if(i===pa)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR:s.COMPRESSED_RGBA_ASTC_10x6_KHR;if(i===ma)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR:s.COMPRESSED_RGBA_ASTC_10x8_KHR;if(i===_a)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR:s.COMPRESSED_RGBA_ASTC_10x10_KHR;if(i===ga)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR:s.COMPRESSED_RGBA_ASTC_12x10_KHR;if(i===xa)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR:s.COMPRESSED_RGBA_ASTC_12x12_KHR}else return null;if(i===Dr||i===va||i===ya)if(s=t.get("EXT_texture_compression_bptc"),s!==null){if(i===Dr)return o===Kt?s.COMPRESSED_SRGB_ALPHA_BPTC_UNORM_EXT:s.COMPRESSED_RGBA_BPTC_UNORM_EXT;if(i===va)return s.COMPRESSED_RGB_BPTC_SIGNED_FLOAT_EXT;if(i===ya)return s.COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_EXT}else return null;if(i===Wc||i===Sa||i===Ma||i===Ea)if(s=t.get("EXT_texture_compression_rgtc"),s!==null){if(i===Dr)return s.COMPRESSED_RED_RGTC1_EXT;if(i===Sa)return s.COMPRESSED_SIGNED_RED_RGTC1_EXT;if(i===Ma)return s.COMPRESSED_RED_GREEN_RGTC2_EXT;if(i===Ea)return s.COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT}else return null;return i===Hi?n.UNSIGNED_INT_24_8:n[i]!==void 0?n[i]:null}return{convert:e}}const s0=`
void main() {

	gl_Position = vec4( position, 1.0 );

}`,a0=`
uniform sampler2DArray depthColor;
uniform float depthWidth;
uniform float depthHeight;

void main() {

	vec2 coord = vec2( gl_FragCoord.x / depthWidth, gl_FragCoord.y / depthHeight );

	if ( coord.x >= 1.0 ) {

		gl_FragDepth = texture( depthColor, vec3( coord.x - 1.0, coord.y, 1 ) ).r;

	} else {

		gl_FragDepth = texture( depthColor, vec3( coord.x, coord.y, 0 ) ).r;

	}

}`;class o0{constructor(){this.texture=null,this.mesh=null,this.depthNear=0,this.depthFar=0}init(t,e,i){if(this.texture===null){const r=new be,s=t.properties.get(r);s.__webglTexture=e.texture,(e.depthNear!==i.depthNear||e.depthFar!==i.depthFar)&&(this.depthNear=e.depthNear,this.depthFar=e.depthFar),this.texture=r}}getMesh(t){if(this.texture!==null&&this.mesh===null){const e=t.cameras[0].viewport,i=new Ln({vertexShader:s0,fragmentShader:a0,uniforms:{depthColor:{value:this.texture},depthWidth:{value:e.z},depthHeight:{value:e.w}}});this.mesh=new ce(new Ki(20,20),i)}return this.mesh}reset(){this.texture=null,this.mesh=null}getDepthTexture(){return this.texture}}class c0 extends Ti{constructor(t,e){super();const i=this;let r=null,s=1,o=null,a="local-floor",l=1,u=null,h=null,p=null,m=null,_=null,x=null;const y=new o0,g=e.getContextAttributes();let d=null,P=null;const b=[],w=[],D=new Xt;let L=null;const I=new Ne;I.viewport=new se;const N=new Ne;N.viewport=new se;const A=[I,N],T=new Rf;let O=null,$=null;this.cameraAutoUpdate=!0,this.enabled=!1,this.isPresenting=!1,this.getController=function(Q){let ct=b[Q];return ct===void 0&&(ct=new Es,b[Q]=ct),ct.getTargetRaySpace()},this.getControllerGrip=function(Q){let ct=b[Q];return ct===void 0&&(ct=new Es,b[Q]=ct),ct.getGripSpace()},this.getHand=function(Q){let ct=b[Q];return ct===void 0&&(ct=new Es,b[Q]=ct),ct.getHandSpace()};function W(Q){const ct=w.indexOf(Q.inputSource);if(ct===-1)return;const St=b[ct];St!==void 0&&(St.update(Q.inputSource,Q.frame,u||o),St.dispatchEvent({type:Q.type,data:Q.inputSource}))}function K(){r.removeEventListener("select",W),r.removeEventListener("selectstart",W),r.removeEventListener("selectend",W),r.removeEventListener("squeeze",W),r.removeEventListener("squeezestart",W),r.removeEventListener("squeezeend",W),r.removeEventListener("end",K),r.removeEventListener("inputsourceschange",it);for(let Q=0;Q<b.length;Q++){const ct=w[Q];ct!==null&&(w[Q]=null,b[Q].disconnect(ct))}O=null,$=null,y.reset(),t.setRenderTarget(d),_=null,m=null,p=null,r=null,P=null,qt.stop(),i.isPresenting=!1,t.setPixelRatio(L),t.setSize(D.width,D.height,!1),i.dispatchEvent({type:"sessionend"})}this.setFramebufferScaleFactor=function(Q){s=Q,i.isPresenting===!0&&console.warn("THREE.WebXRManager: Cannot change framebuffer scale while presenting.")},this.setReferenceSpaceType=function(Q){a=Q,i.isPresenting===!0&&console.warn("THREE.WebXRManager: Cannot change reference space type while presenting.")},this.getReferenceSpace=function(){return u||o},this.setReferenceSpace=function(Q){u=Q},this.getBaseLayer=function(){return m!==null?m:_},this.getBinding=function(){return p},this.getFrame=function(){return x},this.getSession=function(){return r},this.setSession=async function(Q){if(r=Q,r!==null){if(d=t.getRenderTarget(),r.addEventListener("select",W),r.addEventListener("selectstart",W),r.addEventListener("selectend",W),r.addEventListener("squeeze",W),r.addEventListener("squeezestart",W),r.addEventListener("squeezeend",W),r.addEventListener("end",K),r.addEventListener("inputsourceschange",it),g.xrCompatible!==!0&&await e.makeXRCompatible(),L=t.getPixelRatio(),t.getSize(D),typeof XRWebGLBinding<"u"&&"createProjectionLayer"in XRWebGLBinding.prototype){let St=null,ut=null,Tt=null;g.depth&&(Tt=g.stencil?e.DEPTH24_STENCIL8:e.DEPTH_COMPONENT24,St=g.stencil?Gi:Vi,ut=g.stencil?Hi:Yn);const Bt={colorFormat:e.RGBA8,depthFormat:Tt,scaleFactor:s};p=new XRWebGLBinding(r,e),m=p.createProjectionLayer(Bt),r.updateRenderState({layers:[m]}),t.setPixelRatio(1),t.setSize(m.textureWidth,m.textureHeight,!1),P=new jn(m.textureWidth,m.textureHeight,{format:We,type:Je,depthTexture:new il(m.textureWidth,m.textureHeight,ut,void 0,void 0,void 0,void 0,void 0,void 0,St),stencilBuffer:g.stencil,colorSpace:t.outputColorSpace,samples:g.antialias?4:0,resolveDepthBuffer:m.ignoreDepthValues===!1,resolveStencilBuffer:m.ignoreDepthValues===!1})}else{const St={antialias:g.antialias,alpha:!0,depth:g.depth,stencil:g.stencil,framebufferScaleFactor:s};_=new XRWebGLLayer(r,e,St),r.updateRenderState({baseLayer:_}),t.setPixelRatio(1),t.setSize(_.framebufferWidth,_.framebufferHeight,!1),P=new jn(_.framebufferWidth,_.framebufferHeight,{format:We,type:Je,colorSpace:t.outputColorSpace,stencilBuffer:g.stencil,resolveDepthBuffer:_.ignoreDepthValues===!1,resolveStencilBuffer:_.ignoreDepthValues===!1})}P.isXRRenderTarget=!0,this.setFoveation(l),u=null,o=await r.requestReferenceSpace(a),qt.setContext(r),qt.start(),i.isPresenting=!0,i.dispatchEvent({type:"sessionstart"})}},this.getEnvironmentBlendMode=function(){if(r!==null)return r.environmentBlendMode},this.getDepthTexture=function(){return y.getDepthTexture()};function it(Q){for(let ct=0;ct<Q.removed.length;ct++){const St=Q.removed[ct],ut=w.indexOf(St);ut>=0&&(w[ut]=null,b[ut].disconnect(St))}for(let ct=0;ct<Q.added.length;ct++){const St=Q.added[ct];let ut=w.indexOf(St);if(ut===-1){for(let Bt=0;Bt<b.length;Bt++)if(Bt>=w.length){w.push(St),ut=Bt;break}else if(w[Bt]===null){w[Bt]=St,ut=Bt;break}if(ut===-1)break}const Tt=b[ut];Tt&&Tt.connect(St)}}const Z=new H,at=new H;function Y(Q,ct,St){Z.setFromMatrixPosition(ct.matrixWorld),at.setFromMatrixPosition(St.matrixWorld);const ut=Z.distanceTo(at),Tt=ct.projectionMatrix.elements,Bt=St.projectionMatrix.elements,Pt=Tt[14]/(Tt[10]-1),ne=Tt[14]/(Tt[10]+1),Jt=(Tt[9]+1)/Tt[5],It=(Tt[9]-1)/Tt[5],U=(Tt[8]-1)/Tt[0],Ee=(Bt[8]+1)/Bt[0],zt=Pt*U,Ot=Pt*Ee,wt=ut/(-U+Ee),Zt=wt*-U;if(ct.matrixWorld.decompose(Q.position,Q.quaternion,Q.scale),Q.translateX(Zt),Q.translateZ(wt),Q.matrixWorld.compose(Q.position,Q.quaternion,Q.scale),Q.matrixWorldInverse.copy(Q.matrixWorld).invert(),Tt[10]===-1)Q.projectionMatrix.copy(ct.projectionMatrix),Q.projectionMatrixInverse.copy(ct.projectionMatrixInverse);else{const Et=Pt+wt,R=ne+wt,S=zt-Zt,G=Ot+(ut-Zt),tt=Jt*ne/R*Et,et=It*ne/R*Et;Q.projectionMatrix.makePerspective(S,G,tt,et,Et,R),Q.projectionMatrixInverse.copy(Q.projectionMatrix).invert()}}function ft(Q,ct){ct===null?Q.matrixWorld.copy(Q.matrix):Q.matrixWorld.multiplyMatrices(ct.matrixWorld,Q.matrix),Q.matrixWorldInverse.copy(Q.matrixWorld).invert()}this.updateCamera=function(Q){if(r===null)return;let ct=Q.near,St=Q.far;y.texture!==null&&(y.depthNear>0&&(ct=y.depthNear),y.depthFar>0&&(St=y.depthFar)),T.near=N.near=I.near=ct,T.far=N.far=I.far=St,(O!==T.near||$!==T.far)&&(r.updateRenderState({depthNear:T.near,depthFar:T.far}),O=T.near,$=T.far),I.layers.mask=Q.layers.mask|2,N.layers.mask=Q.layers.mask|4,T.layers.mask=I.layers.mask|N.layers.mask;const ut=Q.parent,Tt=T.cameras;ft(T,ut);for(let Bt=0;Bt<Tt.length;Bt++)ft(Tt[Bt],ut);Tt.length===2?Y(T,I,N):T.projectionMatrix.copy(I.projectionMatrix),gt(Q,T,ut)};function gt(Q,ct,St){St===null?Q.matrix.copy(ct.matrixWorld):(Q.matrix.copy(St.matrixWorld),Q.matrix.invert(),Q.matrix.multiply(ct.matrixWorld)),Q.matrix.decompose(Q.position,Q.quaternion,Q.scale),Q.updateMatrixWorld(!0),Q.projectionMatrix.copy(ct.projectionMatrix),Q.projectionMatrixInverse.copy(ct.projectionMatrixInverse),Q.isPerspectiveCamera&&(Q.fov=Ta*2*Math.atan(1/Q.projectionMatrix.elements[5]),Q.zoom=1)}this.getCamera=function(){return T},this.getFoveation=function(){if(!(m===null&&_===null))return l},this.setFoveation=function(Q){l=Q,m!==null&&(m.fixedFoveation=Q),_!==null&&_.fixedFoveation!==void 0&&(_.fixedFoveation=Q)},this.hasDepthSensing=function(){return y.texture!==null},this.getDepthSensingMesh=function(){return y.getMesh(T)};let ht=null;function bt(Q,ct){if(h=ct.getViewerPose(u||o),x=ct,h!==null){const St=h.views;_!==null&&(t.setRenderTargetFramebuffer(P,_.framebuffer),t.setRenderTarget(P));let ut=!1;St.length!==T.cameras.length&&(T.cameras.length=0,ut=!0);for(let Pt=0;Pt<St.length;Pt++){const ne=St[Pt];let Jt=null;if(_!==null)Jt=_.getViewport(ne);else{const U=p.getViewSubImage(m,ne);Jt=U.viewport,Pt===0&&(t.setRenderTargetTextures(P,U.colorTexture,U.depthStencilTexture),t.setRenderTarget(P))}let It=A[Pt];It===void 0&&(It=new Ne,It.layers.enable(Pt),It.viewport=new se,A[Pt]=It),It.matrix.fromArray(ne.transform.matrix),It.matrix.decompose(It.position,It.quaternion,It.scale),It.projectionMatrix.fromArray(ne.projectionMatrix),It.projectionMatrixInverse.copy(It.projectionMatrix).invert(),It.viewport.set(Jt.x,Jt.y,Jt.width,Jt.height),Pt===0&&(T.matrix.copy(It.matrix),T.matrix.decompose(T.position,T.quaternion,T.scale)),ut===!0&&T.cameras.push(It)}const Tt=r.enabledFeatures;if(Tt&&Tt.includes("depth-sensing")&&r.depthUsage=="gpu-optimized"&&p){const Pt=p.getDepthInformation(St[0]);Pt&&Pt.isValid&&Pt.texture&&y.init(t,Pt,r.renderState)}}for(let St=0;St<b.length;St++){const ut=w[St],Tt=b[St];ut!==null&&Tt!==void 0&&Tt.update(ut,ct,u||o)}ht&&ht(Q,ct),ct.detectedPlanes&&i.dispatchEvent({type:"planesdetected",data:ct}),x=null}const qt=new sl;qt.setAnimationLoop(bt),this.setAnimationLoop=function(Q){ht=Q},this.dispose=function(){}}}const Hn=new Ye,l0=new te;function u0(n,t){function e(g,d){g.matrixAutoUpdate===!0&&g.updateMatrix(),d.value.copy(g.matrix)}function i(g,d){d.color.getRGB(g.fogColor.value,Qc(n)),d.isFog?(g.fogNear.value=d.near,g.fogFar.value=d.far):d.isFogExp2&&(g.fogDensity.value=d.density)}function r(g,d,P,b,w){d.isMeshBasicMaterial||d.isMeshLambertMaterial?s(g,d):d.isMeshToonMaterial?(s(g,d),p(g,d)):d.isMeshPhongMaterial?(s(g,d),h(g,d)):d.isMeshStandardMaterial?(s(g,d),m(g,d),d.isMeshPhysicalMaterial&&_(g,d,w)):d.isMeshMatcapMaterial?(s(g,d),x(g,d)):d.isMeshDepthMaterial?s(g,d):d.isMeshDistanceMaterial?(s(g,d),y(g,d)):d.isMeshNormalMaterial?s(g,d):d.isLineBasicMaterial?(o(g,d),d.isLineDashedMaterial&&a(g,d)):d.isPointsMaterial?l(g,d,P,b):d.isSpriteMaterial?u(g,d):d.isShadowMaterial?(g.color.value.copy(d.color),g.opacity.value=d.opacity):d.isShaderMaterial&&(d.uniformsNeedUpdate=!1)}function s(g,d){g.opacity.value=d.opacity,d.color&&g.diffuse.value.copy(d.color),d.emissive&&g.emissive.value.copy(d.emissive).multiplyScalar(d.emissiveIntensity),d.map&&(g.map.value=d.map,e(d.map,g.mapTransform)),d.alphaMap&&(g.alphaMap.value=d.alphaMap,e(d.alphaMap,g.alphaMapTransform)),d.bumpMap&&(g.bumpMap.value=d.bumpMap,e(d.bumpMap,g.bumpMapTransform),g.bumpScale.value=d.bumpScale,d.side===Ae&&(g.bumpScale.value*=-1)),d.normalMap&&(g.normalMap.value=d.normalMap,e(d.normalMap,g.normalMapTransform),g.normalScale.value.copy(d.normalScale),d.side===Ae&&g.normalScale.value.negate()),d.displacementMap&&(g.displacementMap.value=d.displacementMap,e(d.displacementMap,g.displacementMapTransform),g.displacementScale.value=d.displacementScale,g.displacementBias.value=d.displacementBias),d.emissiveMap&&(g.emissiveMap.value=d.emissiveMap,e(d.emissiveMap,g.emissiveMapTransform)),d.specularMap&&(g.specularMap.value=d.specularMap,e(d.specularMap,g.specularMapTransform)),d.alphaTest>0&&(g.alphaTest.value=d.alphaTest);const P=t.get(d),b=P.envMap,w=P.envMapRotation;b&&(g.envMap.value=b,Hn.copy(w),Hn.x*=-1,Hn.y*=-1,Hn.z*=-1,b.isCubeTexture&&b.isRenderTargetTexture===!1&&(Hn.y*=-1,Hn.z*=-1),g.envMapRotation.value.setFromMatrix4(l0.makeRotationFromEuler(Hn)),g.flipEnvMap.value=b.isCubeTexture&&b.isRenderTargetTexture===!1?-1:1,g.reflectivity.value=d.reflectivity,g.ior.value=d.ior,g.refractionRatio.value=d.refractionRatio),d.lightMap&&(g.lightMap.value=d.lightMap,g.lightMapIntensity.value=d.lightMapIntensity,e(d.lightMap,g.lightMapTransform)),d.aoMap&&(g.aoMap.value=d.aoMap,g.aoMapIntensity.value=d.aoMapIntensity,e(d.aoMap,g.aoMapTransform))}function o(g,d){g.diffuse.value.copy(d.color),g.opacity.value=d.opacity,d.map&&(g.map.value=d.map,e(d.map,g.mapTransform))}function a(g,d){g.dashSize.value=d.dashSize,g.totalSize.value=d.dashSize+d.gapSize,g.scale.value=d.scale}function l(g,d,P,b){g.diffuse.value.copy(d.color),g.opacity.value=d.opacity,g.size.value=d.size*P,g.scale.value=b*.5,d.map&&(g.map.value=d.map,e(d.map,g.uvTransform)),d.alphaMap&&(g.alphaMap.value=d.alphaMap,e(d.alphaMap,g.alphaMapTransform)),d.alphaTest>0&&(g.alphaTest.value=d.alphaTest)}function u(g,d){g.diffuse.value.copy(d.color),g.opacity.value=d.opacity,g.rotation.value=d.rotation,d.map&&(g.map.value=d.map,e(d.map,g.mapTransform)),d.alphaMap&&(g.alphaMap.value=d.alphaMap,e(d.alphaMap,g.alphaMapTransform)),d.alphaTest>0&&(g.alphaTest.value=d.alphaTest)}function h(g,d){g.specular.value.copy(d.specular),g.shininess.value=Math.max(d.shininess,1e-4)}function p(g,d){d.gradientMap&&(g.gradientMap.value=d.gradientMap)}function m(g,d){g.metalness.value=d.metalness,d.metalnessMap&&(g.metalnessMap.value=d.metalnessMap,e(d.metalnessMap,g.metalnessMapTransform)),g.roughness.value=d.roughness,d.roughnessMap&&(g.roughnessMap.value=d.roughnessMap,e(d.roughnessMap,g.roughnessMapTransform)),d.envMap&&(g.envMapIntensity.value=d.envMapIntensity)}function _(g,d,P){g.ior.value=d.ior,d.sheen>0&&(g.sheenColor.value.copy(d.sheenColor).multiplyScalar(d.sheen),g.sheenRoughness.value=d.sheenRoughness,d.sheenColorMap&&(g.sheenColorMap.value=d.sheenColorMap,e(d.sheenColorMap,g.sheenColorMapTransform)),d.sheenRoughnessMap&&(g.sheenRoughnessMap.value=d.sheenRoughnessMap,e(d.sheenRoughnessMap,g.sheenRoughnessMapTransform))),d.clearcoat>0&&(g.clearcoat.value=d.clearcoat,g.clearcoatRoughness.value=d.clearcoatRoughness,d.clearcoatMap&&(g.clearcoatMap.value=d.clearcoatMap,e(d.clearcoatMap,g.clearcoatMapTransform)),d.clearcoatRoughnessMap&&(g.clearcoatRoughnessMap.value=d.clearcoatRoughnessMap,e(d.clearcoatRoughnessMap,g.clearcoatRoughnessMapTransform)),d.clearcoatNormalMap&&(g.clearcoatNormalMap.value=d.clearcoatNormalMap,e(d.clearcoatNormalMap,g.clearcoatNormalMapTransform),g.clearcoatNormalScale.value.copy(d.clearcoatNormalScale),d.side===Ae&&g.clearcoatNormalScale.value.negate())),d.dispersion>0&&(g.dispersion.value=d.dispersion),d.iridescence>0&&(g.iridescence.value=d.iridescence,g.iridescenceIOR.value=d.iridescenceIOR,g.iridescenceThicknessMinimum.value=d.iridescenceThicknessRange[0],g.iridescenceThicknessMaximum.value=d.iridescenceThicknessRange[1],d.iridescenceMap&&(g.iridescenceMap.value=d.iridescenceMap,e(d.iridescenceMap,g.iridescenceMapTransform)),d.iridescenceThicknessMap&&(g.iridescenceThicknessMap.value=d.iridescenceThicknessMap,e(d.iridescenceThicknessMap,g.iridescenceThicknessMapTransform))),d.transmission>0&&(g.transmission.value=d.transmission,g.transmissionSamplerMap.value=P.texture,g.transmissionSamplerSize.value.set(P.width,P.height),d.transmissionMap&&(g.transmissionMap.value=d.transmissionMap,e(d.transmissionMap,g.transmissionMapTransform)),g.thickness.value=d.thickness,d.thicknessMap&&(g.thicknessMap.value=d.thicknessMap,e(d.thicknessMap,g.thicknessMapTransform)),g.attenuationDistance.value=d.attenuationDistance,g.attenuationColor.value.copy(d.attenuationColor)),d.anisotropy>0&&(g.anisotropyVector.value.set(d.anisotropy*Math.cos(d.anisotropyRotation),d.anisotropy*Math.sin(d.anisotropyRotation)),d.anisotropyMap&&(g.anisotropyMap.value=d.anisotropyMap,e(d.anisotropyMap,g.anisotropyMapTransform))),g.specularIntensity.value=d.specularIntensity,g.specularColor.value.copy(d.specularColor),d.specularColorMap&&(g.specularColorMap.value=d.specularColorMap,e(d.specularColorMap,g.specularColorMapTransform)),d.specularIntensityMap&&(g.specularIntensityMap.value=d.specularIntensityMap,e(d.specularIntensityMap,g.specularIntensityMapTransform))}function x(g,d){d.matcap&&(g.matcap.value=d.matcap)}function y(g,d){const P=t.get(d).light;g.referencePosition.value.setFromMatrixPosition(P.matrixWorld),g.nearDistance.value=P.shadow.camera.near,g.farDistance.value=P.shadow.camera.far}return{refreshFogUniforms:i,refreshMaterialUniforms:r}}function f0(n,t,e,i){let r={},s={},o=[];const a=n.getParameter(n.MAX_UNIFORM_BUFFER_BINDINGS);function l(P,b){const w=b.program;i.uniformBlockBinding(P,w)}function u(P,b){let w=r[P.id];w===void 0&&(x(P),w=h(P),r[P.id]=w,P.addEventListener("dispose",g));const D=b.program;i.updateUBOMapping(P,D);const L=t.render.frame;s[P.id]!==L&&(m(P),s[P.id]=L)}function h(P){const b=p();P.__bindingPointIndex=b;const w=n.createBuffer(),D=P.__size,L=P.usage;return n.bindBuffer(n.UNIFORM_BUFFER,w),n.bufferData(n.UNIFORM_BUFFER,D,L),n.bindBuffer(n.UNIFORM_BUFFER,null),n.bindBufferBase(n.UNIFORM_BUFFER,b,w),w}function p(){for(let P=0;P<a;P++)if(o.indexOf(P)===-1)return o.push(P),P;return console.error("THREE.WebGLRenderer: Maximum number of simultaneously usable uniforms groups reached."),0}function m(P){const b=r[P.id],w=P.uniforms,D=P.__cache;n.bindBuffer(n.UNIFORM_BUFFER,b);for(let L=0,I=w.length;L<I;L++){const N=Array.isArray(w[L])?w[L]:[w[L]];for(let A=0,T=N.length;A<T;A++){const O=N[A];if(_(O,L,A,D)===!0){const $=O.__offset,W=Array.isArray(O.value)?O.value:[O.value];let K=0;for(let it=0;it<W.length;it++){const Z=W[it],at=y(Z);typeof Z=="number"||typeof Z=="boolean"?(O.__data[0]=Z,n.bufferSubData(n.UNIFORM_BUFFER,$+K,O.__data)):Z.isMatrix3?(O.__data[0]=Z.elements[0],O.__data[1]=Z.elements[1],O.__data[2]=Z.elements[2],O.__data[3]=0,O.__data[4]=Z.elements[3],O.__data[5]=Z.elements[4],O.__data[6]=Z.elements[5],O.__data[7]=0,O.__data[8]=Z.elements[6],O.__data[9]=Z.elements[7],O.__data[10]=Z.elements[8],O.__data[11]=0):(Z.toArray(O.__data,K),K+=at.storage/Float32Array.BYTES_PER_ELEMENT)}n.bufferSubData(n.UNIFORM_BUFFER,$,O.__data)}}}n.bindBuffer(n.UNIFORM_BUFFER,null)}function _(P,b,w,D){const L=P.value,I=b+"_"+w;if(D[I]===void 0)return typeof L=="number"||typeof L=="boolean"?D[I]=L:D[I]=L.clone(),!0;{const N=D[I];if(typeof L=="number"||typeof L=="boolean"){if(N!==L)return D[I]=L,!0}else if(N.equals(L)===!1)return N.copy(L),!0}return!1}function x(P){const b=P.uniforms;let w=0;const D=16;for(let I=0,N=b.length;I<N;I++){const A=Array.isArray(b[I])?b[I]:[b[I]];for(let T=0,O=A.length;T<O;T++){const $=A[T],W=Array.isArray($.value)?$.value:[$.value];for(let K=0,it=W.length;K<it;K++){const Z=W[K],at=y(Z),Y=w%D,ft=Y%at.boundary,gt=Y+ft;w+=ft,gt!==0&&D-gt<at.storage&&(w+=D-gt),$.__data=new Float32Array(at.storage/Float32Array.BYTES_PER_ELEMENT),$.__offset=w,w+=at.storage}}}const L=w%D;return L>0&&(w+=D-L),P.__size=w,P.__cache={},this}function y(P){const b={boundary:0,storage:0};return typeof P=="number"||typeof P=="boolean"?(b.boundary=4,b.storage=4):P.isVector2?(b.boundary=8,b.storage=8):P.isVector3||P.isColor?(b.boundary=16,b.storage=12):P.isVector4?(b.boundary=16,b.storage=16):P.isMatrix3?(b.boundary=48,b.storage=48):P.isMatrix4?(b.boundary=64,b.storage=64):P.isTexture?console.warn("THREE.WebGLRenderer: Texture samplers can not be part of an uniforms group."):console.warn("THREE.WebGLRenderer: Unsupported uniform value type.",P),b}function g(P){const b=P.target;b.removeEventListener("dispose",g);const w=o.indexOf(b.__bindingPointIndex);o.splice(w,1),n.deleteBuffer(r[b.id]),delete r[b.id],delete s[b.id]}function d(){for(const P in r)n.deleteBuffer(r[P]);o=[],r={},s={}}return{bind:l,update:u,dispose:d}}class h0{constructor(t={}){const{canvas:e=Wu(),context:i=null,depth:r=!0,stencil:s=!1,alpha:o=!1,antialias:a=!1,premultipliedAlpha:l=!0,preserveDrawingBuffer:u=!1,powerPreference:h="default",failIfMajorPerformanceCaveat:p=!1,reverseDepthBuffer:m=!1}=t;this.isWebGLRenderer=!0;let _;if(i!==null){if(typeof WebGLRenderingContext<"u"&&i instanceof WebGLRenderingContext)throw new Error("THREE.WebGLRenderer: WebGL 1 is not supported since r163.");_=i.getContextAttributes().alpha}else _=o;const x=new Uint32Array(4),y=new Int32Array(4);let g=null,d=null;const P=[],b=[];this.domElement=e,this.debug={checkShaderErrors:!0,onShaderError:null},this.autoClear=!0,this.autoClearColor=!0,this.autoClearDepth=!0,this.autoClearStencil=!0,this.sortObjects=!0,this.clippingPlanes=[],this.localClippingEnabled=!1,this.toneMapping=Rn,this.toneMappingExposure=1,this.transmissionResolutionScale=1;const w=this;let D=!1;this._outputColorSpace=Fe;let L=0,I=0,N=null,A=-1,T=null;const O=new se,$=new se;let W=null;const K=new Wt(0);let it=0,Z=e.width,at=e.height,Y=1,ft=null,gt=null;const ht=new se(0,0,Z,at),bt=new se(0,0,Z,at);let qt=!1;const Q=new ka;let ct=!1,St=!1;const ut=new te,Tt=new te,Bt=new H,Pt=new se,ne={background:null,fog:null,environment:null,overrideMaterial:null,isScene:!0};let Jt=!1;function It(){return N===null?Y:1}let U=i;function Ee(M,z){return e.getContext(M,z)}try{const M={alpha:!0,depth:r,stencil:s,antialias:a,premultipliedAlpha:l,preserveDrawingBuffer:u,powerPreference:h,failIfMajorPerformanceCaveat:p};if("setAttribute"in e&&e.setAttribute("data-engine",`three.js r${La}`),e.addEventListener("webglcontextlost",nt,!1),e.addEventListener("webglcontextrestored",_t,!1),e.addEventListener("webglcontextcreationerror",mt,!1),U===null){const z="webgl2";if(U=Ee(z,M),U===null)throw Ee(z)?new Error("Error creating WebGL context with your selected attributes."):new Error("Error creating WebGL context.")}}catch(M){throw console.error("THREE.WebGLRenderer: "+M.message),M}let zt,Ot,wt,Zt,Et,R,S,G,tt,et,J,v,c,f,E,C,F,V,ot,st,Mt,vt,At,B;function pt(){zt=new Mp(U),zt.init(),vt=new r0(U,zt),Ot=new mp(U,zt,t,vt),wt=new n0(U,zt),Ot.reverseDepthBuffer&&m&&wt.buffers.depth.setReversed(!0),Zt=new wp(U),Et=new Gm,R=new i0(U,zt,wt,Et,Ot,vt,Zt),S=new gp(w),G=new Sp(w),tt=new Lf(U),At=new dp(U,tt),et=new Ep(U,tt,Zt,At),J=new bp(U,et,tt,Zt),ot=new Ap(U,Ot,R),C=new _p(Et),v=new Vm(w,S,G,zt,Ot,At,C),c=new u0(w,Et),f=new Xm,E=new Zm(zt),V=new hp(w,S,G,wt,J,_,l),F=new t0(w,J,Ot),B=new f0(U,Zt,Ot,wt),st=new pp(U,zt,Zt),Mt=new Tp(U,zt,Zt),Zt.programs=v.programs,w.capabilities=Ot,w.extensions=zt,w.properties=Et,w.renderLists=f,w.shadowMap=F,w.state=wt,w.info=Zt}pt();const j=new c0(w,U);this.xr=j,this.getContext=function(){return U},this.getContextAttributes=function(){return U.getContextAttributes()},this.forceContextLoss=function(){const M=zt.get("WEBGL_lose_context");M&&M.loseContext()},this.forceContextRestore=function(){const M=zt.get("WEBGL_lose_context");M&&M.restoreContext()},this.getPixelRatio=function(){return Y},this.setPixelRatio=function(M){M!==void 0&&(Y=M,this.setSize(Z,at,!1))},this.getSize=function(M){return M.set(Z,at)},this.setSize=function(M,z,X=!0){if(j.isPresenting){console.warn("THREE.WebGLRenderer: Can't change size while VR device is presenting.");return}Z=M,at=z,e.width=Math.floor(M*Y),e.height=Math.floor(z*Y),X===!0&&(e.style.width=M+"px",e.style.height=z+"px"),this.setViewport(0,0,M,z)},this.getDrawingBufferSize=function(M){return M.set(Z*Y,at*Y).floor()},this.setDrawingBufferSize=function(M,z,X){Z=M,at=z,Y=X,e.width=Math.floor(M*X),e.height=Math.floor(z*X),this.setViewport(0,0,M,z)},this.getCurrentViewport=function(M){return M.copy(O)},this.getViewport=function(M){return M.copy(ht)},this.setViewport=function(M,z,X,q){M.isVector4?ht.set(M.x,M.y,M.z,M.w):ht.set(M,z,X,q),wt.viewport(O.copy(ht).multiplyScalar(Y).round())},this.getScissor=function(M){return M.copy(bt)},this.setScissor=function(M,z,X,q){M.isVector4?bt.set(M.x,M.y,M.z,M.w):bt.set(M,z,X,q),wt.scissor($.copy(bt).multiplyScalar(Y).round())},this.getScissorTest=function(){return qt},this.setScissorTest=function(M){wt.setScissorTest(qt=M)},this.setOpaqueSort=function(M){ft=M},this.setTransparentSort=function(M){gt=M},this.getClearColor=function(M){return M.copy(V.getClearColor())},this.setClearColor=function(){V.setClearColor(...arguments)},this.getClearAlpha=function(){return V.getClearAlpha()},this.setClearAlpha=function(){V.setClearAlpha(...arguments)},this.clear=function(M=!0,z=!0,X=!0){let q=0;if(M){let k=!1;if(N!==null){const rt=N.texture.format;k=rt===Oa||rt===Na||rt===Fa}if(k){const rt=N.texture.type,dt=rt===Je||rt===Yn||rt===ki||rt===Hi||rt===Ia||rt===Ua,xt=V.getClearColor(),yt=V.getClearAlpha(),Dt=xt.r,Lt=xt.g,Ct=xt.b;dt?(x[0]=Dt,x[1]=Lt,x[2]=Ct,x[3]=yt,U.clearBufferuiv(U.COLOR,0,x)):(y[0]=Dt,y[1]=Lt,y[2]=Ct,y[3]=yt,U.clearBufferiv(U.COLOR,0,y))}else q|=U.COLOR_BUFFER_BIT}z&&(q|=U.DEPTH_BUFFER_BIT),X&&(q|=U.STENCIL_BUFFER_BIT,this.state.buffers.stencil.setMask(4294967295)),U.clear(q)},this.clearColor=function(){this.clear(!0,!1,!1)},this.clearDepth=function(){this.clear(!1,!0,!1)},this.clearStencil=function(){this.clear(!1,!1,!0)},this.dispose=function(){e.removeEventListener("webglcontextlost",nt,!1),e.removeEventListener("webglcontextrestored",_t,!1),e.removeEventListener("webglcontextcreationerror",mt,!1),V.dispose(),f.dispose(),E.dispose(),Et.dispose(),S.dispose(),G.dispose(),J.dispose(),At.dispose(),B.dispose(),v.dispose(),j.dispose(),j.removeEventListener("sessionstart",Ya),j.removeEventListener("sessionend",ja),Un.stop()};function nt(M){M.preventDefault(),console.log("THREE.WebGLRenderer: Context Lost."),D=!0}function _t(){console.log("THREE.WebGLRenderer: Context Restored."),D=!1;const M=Zt.autoReset,z=F.enabled,X=F.autoUpdate,q=F.needsUpdate,k=F.type;pt(),Zt.autoReset=M,F.enabled=z,F.autoUpdate=X,F.needsUpdate=q,F.type=k}function mt(M){console.error("THREE.WebGLRenderer: A WebGL context could not be created. Reason: ",M.statusMessage)}function Ut(M){const z=M.target;z.removeEventListener("dispose",Ut),ie(z)}function ie(M){pe(M),Et.remove(M)}function pe(M){const z=Et.get(M).programs;z!==void 0&&(z.forEach(function(X){v.releaseProgram(X)}),M.isShaderMaterial&&v.releaseShaderCache(M))}this.renderBufferDirect=function(M,z,X,q,k,rt){z===null&&(z=ne);const dt=k.isMesh&&k.matrixWorld.determinant()<0,xt=Gl(M,z,X,q,k);wt.setMaterial(q,dt);let yt=X.index,Dt=1;if(q.wireframe===!0){if(yt=et.getWireframeAttribute(X),yt===void 0)return;Dt=2}const Lt=X.drawRange,Ct=X.attributes.position;let Ht=Lt.start*Dt,$t=(Lt.start+Lt.count)*Dt;rt!==null&&(Ht=Math.max(Ht,rt.start*Dt),$t=Math.min($t,(rt.start+rt.count)*Dt)),yt!==null?(Ht=Math.max(Ht,0),$t=Math.min($t,yt.count)):Ct!=null&&(Ht=Math.max(Ht,0),$t=Math.min($t,Ct.count));const ae=$t-Ht;if(ae<0||ae===1/0)return;At.setup(k,q,xt,X,yt);let re,Vt=st;if(yt!==null&&(re=tt.get(yt),Vt=Mt,Vt.setIndex(re)),k.isMesh)q.wireframe===!0?(wt.setLineWidth(q.wireframeLinewidth*It()),Vt.setMode(U.LINES)):Vt.setMode(U.TRIANGLES);else if(k.isLine){let Rt=q.linewidth;Rt===void 0&&(Rt=1),wt.setLineWidth(Rt*It()),k.isLineSegments?Vt.setMode(U.LINES):k.isLineLoop?Vt.setMode(U.LINE_LOOP):Vt.setMode(U.LINE_STRIP)}else k.isPoints?Vt.setMode(U.POINTS):k.isSprite&&Vt.setMode(U.TRIANGLES);if(k.isBatchedMesh)if(k._multiDrawInstances!==null)Ir("THREE.WebGLRenderer: renderMultiDrawInstances has been deprecated and will be removed in r184. Append to renderMultiDraw arguments and use indirection."),Vt.renderMultiDrawInstances(k._multiDrawStarts,k._multiDrawCounts,k._multiDrawCount,k._multiDrawInstances);else if(zt.get("WEBGL_multi_draw"))Vt.renderMultiDraw(k._multiDrawStarts,k._multiDrawCounts,k._multiDrawCount);else{const Rt=k._multiDrawStarts,he=k._multiDrawCounts,Yt=k._multiDrawCount,Be=yt?tt.get(yt).bytesPerElement:1,ti=Et.get(q).currentProgram.getUniforms();for(let Ce=0;Ce<Yt;Ce++)ti.setValue(U,"_gl_DrawID",Ce),Vt.render(Rt[Ce]/Be,he[Ce])}else if(k.isInstancedMesh)Vt.renderInstances(Ht,ae,k.count);else if(X.isInstancedBufferGeometry){const Rt=X._maxInstanceCount!==void 0?X._maxInstanceCount:1/0,he=Math.min(X.instanceCount,Rt);Vt.renderInstances(Ht,ae,he)}else Vt.render(Ht,ae)};function jt(M,z,X){M.transparent===!0&&M.side===ve&&M.forceSinglePass===!1?(M.side=Ae,M.needsUpdate=!0,Ji(M,z,X),M.side=Pn,M.needsUpdate=!0,Ji(M,z,X),M.side=ve):Ji(M,z,X)}this.compile=function(M,z,X=null){X===null&&(X=M),d=E.get(X),d.init(z),b.push(d),X.traverseVisible(function(k){k.isLight&&k.layers.test(z.layers)&&(d.pushLight(k),k.castShadow&&d.pushShadow(k))}),M!==X&&M.traverseVisible(function(k){k.isLight&&k.layers.test(z.layers)&&(d.pushLight(k),k.castShadow&&d.pushShadow(k))}),d.setupLights();const q=new Set;return M.traverse(function(k){if(!(k.isMesh||k.isPoints||k.isLine||k.isSprite))return;const rt=k.material;if(rt)if(Array.isArray(rt))for(let dt=0;dt<rt.length;dt++){const xt=rt[dt];jt(xt,X,k),q.add(xt)}else jt(rt,X,k),q.add(rt)}),d=b.pop(),q},this.compileAsync=function(M,z,X=null){const q=this.compile(M,z,X);return new Promise(k=>{function rt(){if(q.forEach(function(dt){Et.get(dt).currentProgram.isReady()&&q.delete(dt)}),q.size===0){k(M);return}setTimeout(rt,10)}zt.get("KHR_parallel_shader_compile")!==null?rt():setTimeout(rt,10)})};let Oe=null;function Qe(M){Oe&&Oe(M)}function Ya(){Un.stop()}function ja(){Un.start()}const Un=new sl;Un.setAnimationLoop(Qe),typeof self<"u"&&Un.setContext(self),this.setAnimationLoop=function(M){Oe=M,j.setAnimationLoop(M),M===null?Un.stop():Un.start()},j.addEventListener("sessionstart",Ya),j.addEventListener("sessionend",ja),this.render=function(M,z){if(z!==void 0&&z.isCamera!==!0){console.error("THREE.WebGLRenderer.render: camera is not an instance of THREE.Camera.");return}if(D===!0)return;if(M.matrixWorldAutoUpdate===!0&&M.updateMatrixWorld(),z.parent===null&&z.matrixWorldAutoUpdate===!0&&z.updateMatrixWorld(),j.enabled===!0&&j.isPresenting===!0&&(j.cameraAutoUpdate===!0&&j.updateCamera(z),z=j.getCamera()),M.isScene===!0&&M.onBeforeRender(w,M,z,N),d=E.get(M,b.length),d.init(z),b.push(d),Tt.multiplyMatrices(z.projectionMatrix,z.matrixWorldInverse),Q.setFromProjectionMatrix(Tt),St=this.localClippingEnabled,ct=C.init(this.clippingPlanes,St),g=f.get(M,P.length),g.init(),P.push(g),j.enabled===!0&&j.isPresenting===!0){const rt=w.xr.getDepthSensingMesh();rt!==null&&Zr(rt,z,-1/0,w.sortObjects)}Zr(M,z,0,w.sortObjects),g.finish(),w.sortObjects===!0&&g.sort(ft,gt),Jt=j.enabled===!1||j.isPresenting===!1||j.hasDepthSensing()===!1,Jt&&V.addToRenderList(g,M),this.info.render.frame++,ct===!0&&C.beginShadows();const X=d.state.shadowsArray;F.render(X,M,z),ct===!0&&C.endShadows(),this.info.autoReset===!0&&this.info.reset();const q=g.opaque,k=g.transmissive;if(d.setupLights(),z.isArrayCamera){const rt=z.cameras;if(k.length>0)for(let dt=0,xt=rt.length;dt<xt;dt++){const yt=rt[dt];Za(q,k,M,yt)}Jt&&V.render(M);for(let dt=0,xt=rt.length;dt<xt;dt++){const yt=rt[dt];Ka(g,M,yt,yt.viewport)}}else k.length>0&&Za(q,k,M,z),Jt&&V.render(M),Ka(g,M,z);N!==null&&I===0&&(R.updateMultisampleRenderTarget(N),R.updateRenderTargetMipmap(N)),M.isScene===!0&&M.onAfterRender(w,M,z),At.resetDefaultState(),A=-1,T=null,b.pop(),b.length>0?(d=b[b.length-1],ct===!0&&C.setGlobalState(w.clippingPlanes,d.state.camera)):d=null,P.pop(),P.length>0?g=P[P.length-1]:g=null};function Zr(M,z,X,q){if(M.visible===!1)return;if(M.layers.test(z.layers)){if(M.isGroup)X=M.renderOrder;else if(M.isLOD)M.autoUpdate===!0&&M.update(z);else if(M.isLight)d.pushLight(M),M.castShadow&&d.pushShadow(M);else if(M.isSprite){if(!M.frustumCulled||Q.intersectsSprite(M)){q&&Pt.setFromMatrixPosition(M.matrixWorld).applyMatrix4(Tt);const dt=J.update(M),xt=M.material;xt.visible&&g.push(M,dt,xt,X,Pt.z,null)}}else if((M.isMesh||M.isLine||M.isPoints)&&(!M.frustumCulled||Q.intersectsObject(M))){const dt=J.update(M),xt=M.material;if(q&&(M.boundingSphere!==void 0?(M.boundingSphere===null&&M.computeBoundingSphere(),Pt.copy(M.boundingSphere.center)):(dt.boundingSphere===null&&dt.computeBoundingSphere(),Pt.copy(dt.boundingSphere.center)),Pt.applyMatrix4(M.matrixWorld).applyMatrix4(Tt)),Array.isArray(xt)){const yt=dt.groups;for(let Dt=0,Lt=yt.length;Dt<Lt;Dt++){const Ct=yt[Dt],Ht=xt[Ct.materialIndex];Ht&&Ht.visible&&g.push(M,dt,Ht,X,Pt.z,Ct)}}else xt.visible&&g.push(M,dt,xt,X,Pt.z,null)}}const rt=M.children;for(let dt=0,xt=rt.length;dt<xt;dt++)Zr(rt[dt],z,X,q)}function Ka(M,z,X,q){const k=M.opaque,rt=M.transmissive,dt=M.transparent;d.setupLightsView(X),ct===!0&&C.setGlobalState(w.clippingPlanes,X),q&&wt.viewport(O.copy(q)),k.length>0&&Zi(k,z,X),rt.length>0&&Zi(rt,z,X),dt.length>0&&Zi(dt,z,X),wt.buffers.depth.setTest(!0),wt.buffers.depth.setMask(!0),wt.buffers.color.setMask(!0),wt.setPolygonOffset(!1)}function Za(M,z,X,q){if((X.isScene===!0?X.overrideMaterial:null)!==null)return;d.state.transmissionRenderTarget[q.id]===void 0&&(d.state.transmissionRenderTarget[q.id]=new jn(1,1,{generateMipmaps:!0,type:zt.has("EXT_color_buffer_half_float")||zt.has("EXT_color_buffer_float")?qi:Je,minFilter:$n,samples:4,stencilBuffer:s,resolveDepthBuffer:!1,resolveStencilBuffer:!1,colorSpace:Gt.workingColorSpace}));const rt=d.state.transmissionRenderTarget[q.id],dt=q.viewport||O;rt.setSize(dt.z*w.transmissionResolutionScale,dt.w*w.transmissionResolutionScale);const xt=w.getRenderTarget();w.setRenderTarget(rt),w.getClearColor(K),it=w.getClearAlpha(),it<1&&w.setClearColor(16777215,.5),w.clear(),Jt&&V.render(X);const yt=w.toneMapping;w.toneMapping=Rn;const Dt=q.viewport;if(q.viewport!==void 0&&(q.viewport=void 0),d.setupLightsView(q),ct===!0&&C.setGlobalState(w.clippingPlanes,q),Zi(M,X,q),R.updateMultisampleRenderTarget(rt),R.updateRenderTargetMipmap(rt),zt.has("WEBGL_multisampled_render_to_texture")===!1){let Lt=!1;for(let Ct=0,Ht=z.length;Ct<Ht;Ct++){const $t=z[Ct],ae=$t.object,re=$t.geometry,Vt=$t.material,Rt=$t.group;if(Vt.side===ve&&ae.layers.test(q.layers)){const he=Vt.side;Vt.side=Ae,Vt.needsUpdate=!0,Ja(ae,X,q,re,Vt,Rt),Vt.side=he,Vt.needsUpdate=!0,Lt=!0}}Lt===!0&&(R.updateMultisampleRenderTarget(rt),R.updateRenderTargetMipmap(rt))}w.setRenderTarget(xt),w.setClearColor(K,it),Dt!==void 0&&(q.viewport=Dt),w.toneMapping=yt}function Zi(M,z,X){const q=z.isScene===!0?z.overrideMaterial:null;for(let k=0,rt=M.length;k<rt;k++){const dt=M[k],xt=dt.object,yt=dt.geometry,Dt=dt.group;let Lt=dt.material;Lt.allowOverride===!0&&q!==null&&(Lt=q),xt.layers.test(X.layers)&&Ja(xt,z,X,yt,Lt,Dt)}}function Ja(M,z,X,q,k,rt){M.onBeforeRender(w,z,X,q,k,rt),M.modelViewMatrix.multiplyMatrices(X.matrixWorldInverse,M.matrixWorld),M.normalMatrix.getNormalMatrix(M.modelViewMatrix),k.onBeforeRender(w,z,X,q,M,rt),k.transparent===!0&&k.side===ve&&k.forceSinglePass===!1?(k.side=Ae,k.needsUpdate=!0,w.renderBufferDirect(X,z,q,k,M,rt),k.side=Pn,k.needsUpdate=!0,w.renderBufferDirect(X,z,q,k,M,rt),k.side=ve):w.renderBufferDirect(X,z,q,k,M,rt),M.onAfterRender(w,z,X,q,k,rt)}function Ji(M,z,X){z.isScene!==!0&&(z=ne);const q=Et.get(M),k=d.state.lights,rt=d.state.shadowsArray,dt=k.state.version,xt=v.getParameters(M,k.state,rt,z,X),yt=v.getProgramCacheKey(xt);let Dt=q.programs;q.environment=M.isMeshStandardMaterial?z.environment:null,q.fog=z.fog,q.envMap=(M.isMeshStandardMaterial?G:S).get(M.envMap||q.environment),q.envMapRotation=q.environment!==null&&M.envMap===null?z.environmentRotation:M.envMapRotation,Dt===void 0&&(M.addEventListener("dispose",Ut),Dt=new Map,q.programs=Dt);let Lt=Dt.get(yt);if(Lt!==void 0){if(q.currentProgram===Lt&&q.lightsStateVersion===dt)return to(M,xt),Lt}else xt.uniforms=v.getUniforms(M),M.onBeforeCompile(xt,w),Lt=v.acquireProgram(xt,yt),Dt.set(yt,Lt),q.uniforms=xt.uniforms;const Ct=q.uniforms;return(!M.isShaderMaterial&&!M.isRawShaderMaterial||M.clipping===!0)&&(Ct.clippingPlanes=C.uniform),to(M,xt),q.needsLights=Xl(M),q.lightsStateVersion=dt,q.needsLights&&(Ct.ambientLightColor.value=k.state.ambient,Ct.lightProbe.value=k.state.probe,Ct.directionalLights.value=k.state.directional,Ct.directionalLightShadows.value=k.state.directionalShadow,Ct.spotLights.value=k.state.spot,Ct.spotLightShadows.value=k.state.spotShadow,Ct.rectAreaLights.value=k.state.rectArea,Ct.ltc_1.value=k.state.rectAreaLTC1,Ct.ltc_2.value=k.state.rectAreaLTC2,Ct.pointLights.value=k.state.point,Ct.pointLightShadows.value=k.state.pointShadow,Ct.hemisphereLights.value=k.state.hemi,Ct.directionalShadowMap.value=k.state.directionalShadowMap,Ct.directionalShadowMatrix.value=k.state.directionalShadowMatrix,Ct.spotShadowMap.value=k.state.spotShadowMap,Ct.spotLightMatrix.value=k.state.spotLightMatrix,Ct.spotLightMap.value=k.state.spotLightMap,Ct.pointShadowMap.value=k.state.pointShadowMap,Ct.pointShadowMatrix.value=k.state.pointShadowMatrix),q.currentProgram=Lt,q.uniformsList=null,Lt}function Qa(M){if(M.uniformsList===null){const z=M.currentProgram.getUniforms();M.uniformsList=Ur.seqWithValue(z.seq,M.uniforms)}return M.uniformsList}function to(M,z){const X=Et.get(M);X.outputColorSpace=z.outputColorSpace,X.batching=z.batching,X.batchingColor=z.batchingColor,X.instancing=z.instancing,X.instancingColor=z.instancingColor,X.instancingMorph=z.instancingMorph,X.skinning=z.skinning,X.morphTargets=z.morphTargets,X.morphNormals=z.morphNormals,X.morphColors=z.morphColors,X.morphTargetsCount=z.morphTargetsCount,X.numClippingPlanes=z.numClippingPlanes,X.numIntersection=z.numClipIntersection,X.vertexAlphas=z.vertexAlphas,X.vertexTangents=z.vertexTangents,X.toneMapping=z.toneMapping}function Gl(M,z,X,q,k){z.isScene!==!0&&(z=ne),R.resetTextureUnits();const rt=z.fog,dt=q.isMeshStandardMaterial?z.environment:null,xt=N===null?w.outputColorSpace:N.isXRRenderTarget===!0?N.texture.colorSpace:Si,yt=(q.isMeshStandardMaterial?G:S).get(q.envMap||dt),Dt=q.vertexColors===!0&&!!X.attributes.color&&X.attributes.color.itemSize===4,Lt=!!X.attributes.tangent&&(!!q.normalMap||q.anisotropy>0),Ct=!!X.morphAttributes.position,Ht=!!X.morphAttributes.normal,$t=!!X.morphAttributes.color;let ae=Rn;q.toneMapped&&(N===null||N.isXRRenderTarget===!0)&&(ae=w.toneMapping);const re=X.morphAttributes.position||X.morphAttributes.normal||X.morphAttributes.color,Vt=re!==void 0?re.length:0,Rt=Et.get(q),he=d.state.lights;if(ct===!0&&(St===!0||M!==T)){const Se=M===T&&q.id===A;C.setState(q,M,Se)}let Yt=!1;q.version===Rt.__version?(Rt.needsLights&&Rt.lightsStateVersion!==he.state.version||Rt.outputColorSpace!==xt||k.isBatchedMesh&&Rt.batching===!1||!k.isBatchedMesh&&Rt.batching===!0||k.isBatchedMesh&&Rt.batchingColor===!0&&k.colorTexture===null||k.isBatchedMesh&&Rt.batchingColor===!1&&k.colorTexture!==null||k.isInstancedMesh&&Rt.instancing===!1||!k.isInstancedMesh&&Rt.instancing===!0||k.isSkinnedMesh&&Rt.skinning===!1||!k.isSkinnedMesh&&Rt.skinning===!0||k.isInstancedMesh&&Rt.instancingColor===!0&&k.instanceColor===null||k.isInstancedMesh&&Rt.instancingColor===!1&&k.instanceColor!==null||k.isInstancedMesh&&Rt.instancingMorph===!0&&k.morphTexture===null||k.isInstancedMesh&&Rt.instancingMorph===!1&&k.morphTexture!==null||Rt.envMap!==yt||q.fog===!0&&Rt.fog!==rt||Rt.numClippingPlanes!==void 0&&(Rt.numClippingPlanes!==C.numPlanes||Rt.numIntersection!==C.numIntersection)||Rt.vertexAlphas!==Dt||Rt.vertexTangents!==Lt||Rt.morphTargets!==Ct||Rt.morphNormals!==Ht||Rt.morphColors!==$t||Rt.toneMapping!==ae||Rt.morphTargetsCount!==Vt)&&(Yt=!0):(Yt=!0,Rt.__version=q.version);let Be=Rt.currentProgram;Yt===!0&&(Be=Ji(q,z,k));let ti=!1,Ce=!1,Ci=!1;const ee=Be.getUniforms(),De=Rt.uniforms;if(wt.useProgram(Be.program)&&(ti=!0,Ce=!0,Ci=!0),q.id!==A&&(A=q.id,Ce=!0),ti||T!==M){wt.buffers.depth.getReversed()?(ut.copy(M.projectionMatrix),qu(ut),$u(ut),ee.setValue(U,"projectionMatrix",ut)):ee.setValue(U,"projectionMatrix",M.projectionMatrix),ee.setValue(U,"viewMatrix",M.matrixWorldInverse);const Te=ee.map.cameraPosition;Te!==void 0&&Te.setValue(U,Bt.setFromMatrixPosition(M.matrixWorld)),Ot.logarithmicDepthBuffer&&ee.setValue(U,"logDepthBufFC",2/(Math.log(M.far+1)/Math.LN2)),(q.isMeshPhongMaterial||q.isMeshToonMaterial||q.isMeshLambertMaterial||q.isMeshBasicMaterial||q.isMeshStandardMaterial||q.isShaderMaterial)&&ee.setValue(U,"isOrthographic",M.isOrthographicCamera===!0),T!==M&&(T=M,Ce=!0,Ci=!0)}if(k.isSkinnedMesh){ee.setOptional(U,k,"bindMatrix"),ee.setOptional(U,k,"bindMatrixInverse");const Se=k.skeleton;Se&&(Se.boneTexture===null&&Se.computeBoneTexture(),ee.setValue(U,"boneTexture",Se.boneTexture,R))}k.isBatchedMesh&&(ee.setOptional(U,k,"batchingTexture"),ee.setValue(U,"batchingTexture",k._matricesTexture,R),ee.setOptional(U,k,"batchingIdTexture"),ee.setValue(U,"batchingIdTexture",k._indirectTexture,R),ee.setOptional(U,k,"batchingColorTexture"),k._colorsTexture!==null&&ee.setValue(U,"batchingColorTexture",k._colorsTexture,R));const Ie=X.morphAttributes;if((Ie.position!==void 0||Ie.normal!==void 0||Ie.color!==void 0)&&ot.update(k,X,Be),(Ce||Rt.receiveShadow!==k.receiveShadow)&&(Rt.receiveShadow=k.receiveShadow,ee.setValue(U,"receiveShadow",k.receiveShadow)),q.isMeshGouraudMaterial&&q.envMap!==null&&(De.envMap.value=yt,De.flipEnvMap.value=yt.isCubeTexture&&yt.isRenderTargetTexture===!1?-1:1),q.isMeshStandardMaterial&&q.envMap===null&&z.environment!==null&&(De.envMapIntensity.value=z.environmentIntensity),Ce&&(ee.setValue(U,"toneMappingExposure",w.toneMappingExposure),Rt.needsLights&&Wl(De,Ci),rt&&q.fog===!0&&c.refreshFogUniforms(De,rt),c.refreshMaterialUniforms(De,q,Y,at,d.state.transmissionRenderTarget[M.id]),Ur.upload(U,Qa(Rt),De,R)),q.isShaderMaterial&&q.uniformsNeedUpdate===!0&&(Ur.upload(U,Qa(Rt),De,R),q.uniformsNeedUpdate=!1),q.isSpriteMaterial&&ee.setValue(U,"center",k.center),ee.setValue(U,"modelViewMatrix",k.modelViewMatrix),ee.setValue(U,"normalMatrix",k.normalMatrix),ee.setValue(U,"modelMatrix",k.matrixWorld),q.isShaderMaterial||q.isRawShaderMaterial){const Se=q.uniformsGroups;for(let Te=0,Jr=Se.length;Te<Jr;Te++){const Fn=Se[Te];B.update(Fn,Be),B.bind(Fn,Be)}}return Be}function Wl(M,z){M.ambientLightColor.needsUpdate=z,M.lightProbe.needsUpdate=z,M.directionalLights.needsUpdate=z,M.directionalLightShadows.needsUpdate=z,M.pointLights.needsUpdate=z,M.pointLightShadows.needsUpdate=z,M.spotLights.needsUpdate=z,M.spotLightShadows.needsUpdate=z,M.rectAreaLights.needsUpdate=z,M.hemisphereLights.needsUpdate=z}function Xl(M){return M.isMeshLambertMaterial||M.isMeshToonMaterial||M.isMeshPhongMaterial||M.isMeshStandardMaterial||M.isShadowMaterial||M.isShaderMaterial&&M.lights===!0}this.getActiveCubeFace=function(){return L},this.getActiveMipmapLevel=function(){return I},this.getRenderTarget=function(){return N},this.setRenderTargetTextures=function(M,z,X){const q=Et.get(M);q.__autoAllocateDepthBuffer=M.resolveDepthBuffer===!1,q.__autoAllocateDepthBuffer===!1&&(q.__useRenderToTexture=!1),Et.get(M.texture).__webglTexture=z,Et.get(M.depthTexture).__webglTexture=q.__autoAllocateDepthBuffer?void 0:X,q.__hasExternalTextures=!0},this.setRenderTargetFramebuffer=function(M,z){const X=Et.get(M);X.__webglFramebuffer=z,X.__useDefaultFramebuffer=z===void 0};const ql=U.createFramebuffer();this.setRenderTarget=function(M,z=0,X=0){N=M,L=z,I=X;let q=!0,k=null,rt=!1,dt=!1;if(M){const yt=Et.get(M);if(yt.__useDefaultFramebuffer!==void 0)wt.bindFramebuffer(U.FRAMEBUFFER,null),q=!1;else if(yt.__webglFramebuffer===void 0)R.setupRenderTarget(M);else if(yt.__hasExternalTextures)R.rebindTextures(M,Et.get(M.texture).__webglTexture,Et.get(M.depthTexture).__webglTexture);else if(M.depthBuffer){const Ct=M.depthTexture;if(yt.__boundDepthTexture!==Ct){if(Ct!==null&&Et.has(Ct)&&(M.width!==Ct.image.width||M.height!==Ct.image.height))throw new Error("WebGLRenderTarget: Attached DepthTexture is initialized to the incorrect size.");R.setupDepthRenderbuffer(M)}}const Dt=M.texture;(Dt.isData3DTexture||Dt.isDataArrayTexture||Dt.isCompressedArrayTexture)&&(dt=!0);const Lt=Et.get(M).__webglFramebuffer;M.isWebGLCubeRenderTarget?(Array.isArray(Lt[z])?k=Lt[z][X]:k=Lt[z],rt=!0):M.samples>0&&R.useMultisampledRTT(M)===!1?k=Et.get(M).__webglMultisampledFramebuffer:Array.isArray(Lt)?k=Lt[X]:k=Lt,O.copy(M.viewport),$.copy(M.scissor),W=M.scissorTest}else O.copy(ht).multiplyScalar(Y).floor(),$.copy(bt).multiplyScalar(Y).floor(),W=qt;if(X!==0&&(k=ql),wt.bindFramebuffer(U.FRAMEBUFFER,k)&&q&&wt.drawBuffers(M,k),wt.viewport(O),wt.scissor($),wt.setScissorTest(W),rt){const yt=Et.get(M.texture);U.framebufferTexture2D(U.FRAMEBUFFER,U.COLOR_ATTACHMENT0,U.TEXTURE_CUBE_MAP_POSITIVE_X+z,yt.__webglTexture,X)}else if(dt){const yt=Et.get(M.texture),Dt=z;U.framebufferTextureLayer(U.FRAMEBUFFER,U.COLOR_ATTACHMENT0,yt.__webglTexture,X,Dt)}else if(M!==null&&X!==0){const yt=Et.get(M.texture);U.framebufferTexture2D(U.FRAMEBUFFER,U.COLOR_ATTACHMENT0,U.TEXTURE_2D,yt.__webglTexture,X)}A=-1},this.readRenderTargetPixels=function(M,z,X,q,k,rt,dt){if(!(M&&M.isWebGLRenderTarget)){console.error("THREE.WebGLRenderer.readRenderTargetPixels: renderTarget is not THREE.WebGLRenderTarget.");return}let xt=Et.get(M).__webglFramebuffer;if(M.isWebGLCubeRenderTarget&&dt!==void 0&&(xt=xt[dt]),xt){wt.bindFramebuffer(U.FRAMEBUFFER,xt);try{const yt=M.texture,Dt=yt.format,Lt=yt.type;if(!Ot.textureFormatReadable(Dt)){console.error("THREE.WebGLRenderer.readRenderTargetPixels: renderTarget is not in RGBA or implementation defined format.");return}if(!Ot.textureTypeReadable(Lt)){console.error("THREE.WebGLRenderer.readRenderTargetPixels: renderTarget is not in UnsignedByteType or implementation defined type.");return}z>=0&&z<=M.width-q&&X>=0&&X<=M.height-k&&U.readPixels(z,X,q,k,vt.convert(Dt),vt.convert(Lt),rt)}finally{const yt=N!==null?Et.get(N).__webglFramebuffer:null;wt.bindFramebuffer(U.FRAMEBUFFER,yt)}}},this.readRenderTargetPixelsAsync=async function(M,z,X,q,k,rt,dt){if(!(M&&M.isWebGLRenderTarget))throw new Error("THREE.WebGLRenderer.readRenderTargetPixels: renderTarget is not THREE.WebGLRenderTarget.");let xt=Et.get(M).__webglFramebuffer;if(M.isWebGLCubeRenderTarget&&dt!==void 0&&(xt=xt[dt]),xt)if(z>=0&&z<=M.width-q&&X>=0&&X<=M.height-k){wt.bindFramebuffer(U.FRAMEBUFFER,xt);const yt=M.texture,Dt=yt.format,Lt=yt.type;if(!Ot.textureFormatReadable(Dt))throw new Error("THREE.WebGLRenderer.readRenderTargetPixelsAsync: renderTarget is not in RGBA or implementation defined format.");if(!Ot.textureTypeReadable(Lt))throw new Error("THREE.WebGLRenderer.readRenderTargetPixelsAsync: renderTarget is not in UnsignedByteType or implementation defined type.");const Ct=U.createBuffer();U.bindBuffer(U.PIXEL_PACK_BUFFER,Ct),U.bufferData(U.PIXEL_PACK_BUFFER,rt.byteLength,U.STREAM_READ),U.readPixels(z,X,q,k,vt.convert(Dt),vt.convert(Lt),0);const Ht=N!==null?Et.get(N).__webglFramebuffer:null;wt.bindFramebuffer(U.FRAMEBUFFER,Ht);const $t=U.fenceSync(U.SYNC_GPU_COMMANDS_COMPLETE,0);return U.flush(),await Xu(U,$t,4),U.bindBuffer(U.PIXEL_PACK_BUFFER,Ct),U.getBufferSubData(U.PIXEL_PACK_BUFFER,0,rt),U.deleteBuffer(Ct),U.deleteSync($t),rt}else throw new Error("THREE.WebGLRenderer.readRenderTargetPixelsAsync: requested read bounds are out of range.")},this.copyFramebufferToTexture=function(M,z=null,X=0){const q=Math.pow(2,-X),k=Math.floor(M.image.width*q),rt=Math.floor(M.image.height*q),dt=z!==null?z.x:0,xt=z!==null?z.y:0;R.setTexture2D(M,0),U.copyTexSubImage2D(U.TEXTURE_2D,X,0,0,dt,xt,k,rt),wt.unbindTexture()};const $l=U.createFramebuffer(),Yl=U.createFramebuffer();this.copyTextureToTexture=function(M,z,X=null,q=null,k=0,rt=null){rt===null&&(k!==0?(Ir("WebGLRenderer: copyTextureToTexture function signature has changed to support src and dst mipmap levels."),rt=k,k=0):rt=0);let dt,xt,yt,Dt,Lt,Ct,Ht,$t,ae;const re=M.isCompressedTexture?M.mipmaps[rt]:M.image;if(X!==null)dt=X.max.x-X.min.x,xt=X.max.y-X.min.y,yt=X.isBox3?X.max.z-X.min.z:1,Dt=X.min.x,Lt=X.min.y,Ct=X.isBox3?X.min.z:0;else{const Ie=Math.pow(2,-k);dt=Math.floor(re.width*Ie),xt=Math.floor(re.height*Ie),M.isDataArrayTexture?yt=re.depth:M.isData3DTexture?yt=Math.floor(re.depth*Ie):yt=1,Dt=0,Lt=0,Ct=0}q!==null?(Ht=q.x,$t=q.y,ae=q.z):(Ht=0,$t=0,ae=0);const Vt=vt.convert(z.format),Rt=vt.convert(z.type);let he;z.isData3DTexture?(R.setTexture3D(z,0),he=U.TEXTURE_3D):z.isDataArrayTexture||z.isCompressedArrayTexture?(R.setTexture2DArray(z,0),he=U.TEXTURE_2D_ARRAY):(R.setTexture2D(z,0),he=U.TEXTURE_2D),U.pixelStorei(U.UNPACK_FLIP_Y_WEBGL,z.flipY),U.pixelStorei(U.UNPACK_PREMULTIPLY_ALPHA_WEBGL,z.premultiplyAlpha),U.pixelStorei(U.UNPACK_ALIGNMENT,z.unpackAlignment);const Yt=U.getParameter(U.UNPACK_ROW_LENGTH),Be=U.getParameter(U.UNPACK_IMAGE_HEIGHT),ti=U.getParameter(U.UNPACK_SKIP_PIXELS),Ce=U.getParameter(U.UNPACK_SKIP_ROWS),Ci=U.getParameter(U.UNPACK_SKIP_IMAGES);U.pixelStorei(U.UNPACK_ROW_LENGTH,re.width),U.pixelStorei(U.UNPACK_IMAGE_HEIGHT,re.height),U.pixelStorei(U.UNPACK_SKIP_PIXELS,Dt),U.pixelStorei(U.UNPACK_SKIP_ROWS,Lt),U.pixelStorei(U.UNPACK_SKIP_IMAGES,Ct);const ee=M.isDataArrayTexture||M.isData3DTexture,De=z.isDataArrayTexture||z.isData3DTexture;if(M.isDepthTexture){const Ie=Et.get(M),Se=Et.get(z),Te=Et.get(Ie.__renderTarget),Jr=Et.get(Se.__renderTarget);wt.bindFramebuffer(U.READ_FRAMEBUFFER,Te.__webglFramebuffer),wt.bindFramebuffer(U.DRAW_FRAMEBUFFER,Jr.__webglFramebuffer);for(let Fn=0;Fn<yt;Fn++)ee&&(U.framebufferTextureLayer(U.READ_FRAMEBUFFER,U.COLOR_ATTACHMENT0,Et.get(M).__webglTexture,k,Ct+Fn),U.framebufferTextureLayer(U.DRAW_FRAMEBUFFER,U.COLOR_ATTACHMENT0,Et.get(z).__webglTexture,rt,ae+Fn)),U.blitFramebuffer(Dt,Lt,dt,xt,Ht,$t,dt,xt,U.DEPTH_BUFFER_BIT,U.NEAREST);wt.bindFramebuffer(U.READ_FRAMEBUFFER,null),wt.bindFramebuffer(U.DRAW_FRAMEBUFFER,null)}else if(k!==0||M.isRenderTargetTexture||Et.has(M)){const Ie=Et.get(M),Se=Et.get(z);wt.bindFramebuffer(U.READ_FRAMEBUFFER,$l),wt.bindFramebuffer(U.DRAW_FRAMEBUFFER,Yl);for(let Te=0;Te<yt;Te++)ee?U.framebufferTextureLayer(U.READ_FRAMEBUFFER,U.COLOR_ATTACHMENT0,Ie.__webglTexture,k,Ct+Te):U.framebufferTexture2D(U.READ_FRAMEBUFFER,U.COLOR_ATTACHMENT0,U.TEXTURE_2D,Ie.__webglTexture,k),De?U.framebufferTextureLayer(U.DRAW_FRAMEBUFFER,U.COLOR_ATTACHMENT0,Se.__webglTexture,rt,ae+Te):U.framebufferTexture2D(U.DRAW_FRAMEBUFFER,U.COLOR_ATTACHMENT0,U.TEXTURE_2D,Se.__webglTexture,rt),k!==0?U.blitFramebuffer(Dt,Lt,dt,xt,Ht,$t,dt,xt,U.COLOR_BUFFER_BIT,U.NEAREST):De?U.copyTexSubImage3D(he,rt,Ht,$t,ae+Te,Dt,Lt,dt,xt):U.copyTexSubImage2D(he,rt,Ht,$t,Dt,Lt,dt,xt);wt.bindFramebuffer(U.READ_FRAMEBUFFER,null),wt.bindFramebuffer(U.DRAW_FRAMEBUFFER,null)}else De?M.isDataTexture||M.isData3DTexture?U.texSubImage3D(he,rt,Ht,$t,ae,dt,xt,yt,Vt,Rt,re.data):z.isCompressedArrayTexture?U.compressedTexSubImage3D(he,rt,Ht,$t,ae,dt,xt,yt,Vt,re.data):U.texSubImage3D(he,rt,Ht,$t,ae,dt,xt,yt,Vt,Rt,re):M.isDataTexture?U.texSubImage2D(U.TEXTURE_2D,rt,Ht,$t,dt,xt,Vt,Rt,re.data):M.isCompressedTexture?U.compressedTexSubImage2D(U.TEXTURE_2D,rt,Ht,$t,re.width,re.height,Vt,re.data):U.texSubImage2D(U.TEXTURE_2D,rt,Ht,$t,dt,xt,Vt,Rt,re);U.pixelStorei(U.UNPACK_ROW_LENGTH,Yt),U.pixelStorei(U.UNPACK_IMAGE_HEIGHT,Be),U.pixelStorei(U.UNPACK_SKIP_PIXELS,ti),U.pixelStorei(U.UNPACK_SKIP_ROWS,Ce),U.pixelStorei(U.UNPACK_SKIP_IMAGES,Ci),rt===0&&z.generateMipmaps&&U.generateMipmap(he),wt.unbindTexture()},this.copyTextureToTexture3D=function(M,z,X=null,q=null,k=0){return Ir('WebGLRenderer: copyTextureToTexture3D function has been deprecated. Use "copyTextureToTexture" instead.'),this.copyTextureToTexture(M,z,X,q,k)},this.initRenderTarget=function(M){Et.get(M).__webglFramebuffer===void 0&&R.setupRenderTarget(M)},this.initTexture=function(M){M.isCubeTexture?R.setTextureCube(M,0):M.isData3DTexture?R.setTexture3D(M,0):M.isDataArrayTexture||M.isCompressedArrayTexture?R.setTexture2DArray(M,0):R.setTexture2D(M,0),wt.unbindTexture()},this.resetState=function(){L=0,I=0,N=null,wt.reset(),At.reset()},typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("observe",{detail:this}))}get coordinateSystem(){return ln}get outputColorSpace(){return this._outputColorSpace}set outputColorSpace(t){this._outputColorSpace=t;const e=this.getContext();e.drawingBufferColorSpace=Gt._getDrawingBufferColorSpace(t),e.unpackColorSpace=Gt._getUnpackColorSpace()}}const d0=[{name:"Check_Point_2",uniform_scale:1,position:{x:524.914978,y:-72.053261,z:1609.72998},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:469.625,y:-50.83231,z:1613.47998},quat:{w:1,x:0,y:0,z:0}},{name:"Start_Point",uniform_scale:1,position:{x:645.16803,y:-831.709106,z:110.084},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:486.019012,y:-535.026123,z:676.150024},quat:{w:.931603,x:0,y:-.363478,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.710999,y:-841.440308,z:97.760902},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.682007,y:-839.661926,z:100.323997},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.471008,y:-839.745483,z:100.343002},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.054993,y:-839.763367,z:100.378998},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:640.877991,y:-841.469482,z:97.952599},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.987976,y:-843.000122,z:95.099403},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:643.051025,y:-844.409241,z:92.487297},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.125,y:-844.462646,z:92.621201},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.103027,y:-843.025818,z:95.214104},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:645.038025,y:-844.440796,z:92.780098},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.825989,y:-843.063599,z:95.252403},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.745972,y:-841.444153,z:97.939796},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Finish_Point",uniform_scale:1.396,position:{x:620.690979,y:372.30188,z:2564.110107},quat:{w:1,x:0,y:0,z:0}}],p0=[{name:"Start_Point",uniform_scale:1,position:{x:645.16803,y:-831.709106,z:110.084},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.710999,y:-841.440308,z:97.760902},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.682007,y:-839.661926,z:100.323997},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.471008,y:-839.745483,z:100.343002},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.054993,y:-839.763367,z:100.378998},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:640.877991,y:-841.469482,z:97.952599},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.987976,y:-843.000122,z:95.099403},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:643.051025,y:-844.409241,z:92.487297},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.125,y:-844.462646,z:92.621201},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.103027,y:-843.025818,z:95.214104},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:645.038025,y:-844.440796,z:92.780098},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.825989,y:-843.063599,z:95.252403},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.745972,y:-841.444153,z:97.939796},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:1051.060059,y:68.596832,z:1590.109985},quat:{w:.9987,x:0,y:.0509779,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:637.55603,y:-585.382996,z:606.434998},quat:{w:1,x:0,y:0,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:1119.880005,y:543.206604,z:2291.780029},quat:{w:1,x:0,y:0,z:0}}],m0=[{name:"Start_Point",uniform_scale:1,position:{x:645.16803,y:-831.709106,z:110.084},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.710999,y:-841.440308,z:97.760902},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.682007,y:-839.661926,z:100.323997},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.471008,y:-839.745483,z:100.343002},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.054993,y:-839.763367,z:100.378998},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:640.877991,y:-841.469482,z:97.952599},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.987976,y:-843.000122,z:95.099403},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:643.051025,y:-844.409241,z:92.487297},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.125,y:-844.462646,z:92.621201},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.103027,y:-843.025818,z:95.214104},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:645.038025,y:-844.440796,z:92.780098},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.825989,y:-843.063599,z:95.252403},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.745972,y:-841.444153,z:97.939796},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:608.411011,y:-551.169128,z:654.901001},quat:{w:.996218,x:0,y:.0868903,z:0}},{name:"Finish_Point",uniform_scale:1.396,position:{x:620.711975,y:372.307526,z:2564.129883},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:718.445007,y:-147.717392,z:1429.160034},quat:{w:.993846,x:0,y:.110772,z:0}}],_0=[{name:"Finish_Point",uniform_scale:1,position:{x:534.888,y:-284.277588,z:2313.639893},quat:{w:1,x:0,y:0,z:0}},{name:"Start_Point",uniform_scale:1,position:{x:519.367981,y:-1384.164185,z:99.7817},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:671.789978,y:-631.825745,z:1691.939941},quat:{w:.996721,x:0,y:-.0809115,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:334.958008,y:-701.713623,z:1544.140015},quat:{w:.98965,x:0,y:-.143503,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:259.109985,y:-1044.600708,z:872.73999},quat:{w:.997823,x:0,y:.0659521,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:507.769989,y:-691.222168,z:1560.670044},quat:{w:.990901,x:0,y:.13459,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:512.228027,y:-1066.99585,z:843.586975},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:526.361023,y:-1401.963379,z:52.828701},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:524.234009,y:-1401.888062,z:52.872601},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:521.651001,y:-1401.730713,z:53.207901},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:528.848999,y:-1401.966797,z:52.491501},quat:{w:.999456,x:0,y:-.0329958,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:509.837006,y:-1401.014038,z:55.337502},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:513.203979,y:-1401.264648,z:54.7066},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:516.369995,y:-1401.418091,z:54.133801},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:519.234009,y:-1401.579346,z:53.577999},quat:{w:1,x:0,y:0,z:0}},{name:"Start_Point",uniform_scale:1.612,position:{x:412.575012,y:-232.187561,z:-64.914001},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:723.994995,y:-1124.040405,z:702.659973},quat:{w:.964567,x:0,y:.263839,z:0}}],g0=[{name:"Check_Point_1",uniform_scale:1,position:{x:723.994995,y:-1247.439087,z:702.659973},quat:{w:.964567,x:0,y:.263839,z:0}},{name:"Start_Point",uniform_scale:1,position:{x:518.552979,y:-1535.383911,z:99.917},quat:{w:1,x:0,y:0,z:0}},{name:"Start_Point",uniform_scale:1.612,position:{x:412.575012,y:-257.285614,z:-64.914001},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:519.234009,y:-1554.089722,z:53.577999},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:516.369995,y:-1553.914795,z:54.133801},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:513.203979,y:-1553.747314,z:54.7066},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:509.837006,y:-1553.481201,z:55.337502},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:528.848999,y:-1554.503784,z:52.491501},quat:{w:.999456,x:0,y:-.0329958,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:521.651001,y:-1554.250122,z:53.207901},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:524.234009,y:-1554.415649,z:52.872601},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:526.361023,y:-1554.492065,z:52.828701},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:512.228027,y:-1185.055298,z:843.586975},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:507.769989,y:-767.855957,z:1560.670044},quat:{w:.990901,x:0,y:.13459,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:259.109985,y:-1160.998779,z:872.73999},quat:{w:.997823,x:0,y:.0659521,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:334.958008,y:-779.157715,z:1544.140015},quat:{w:.98965,x:0,y:-.143503,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:543.77002,y:-304.486206,z:2340.919922},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:671.789978,y:-702.809082,z:1691.939941},quat:{w:.996721,x:0,y:-.0809115,z:0}}],x0=[{name:"Check_Point_1",uniform_scale:1,position:{x:723.994995,y:-1247.439087,z:702.659973},quat:{w:.964567,x:0,y:.263839,z:0}},{name:"Start_Point",uniform_scale:1,position:{x:518.552979,y:-1535.383911,z:99.917},quat:{w:1,x:0,y:0,z:0}},{name:"Start_Point",uniform_scale:1.612,position:{x:412.575012,y:-257.285614,z:-64.914001},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:519.234009,y:-1554.089722,z:53.577999},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:516.369995,y:-1553.914795,z:54.133801},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:513.203979,y:-1553.747314,z:54.7066},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:509.837006,y:-1553.481201,z:55.337502},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:528.848999,y:-1554.503784,z:52.491501},quat:{w:.999456,x:0,y:-.0329958,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:521.651001,y:-1554.250122,z:53.207901},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:524.234009,y:-1554.415649,z:52.872601},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:526.361023,y:-1554.492065,z:52.828701},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:512.228027,y:-1185.055298,z:843.586975},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:507.769989,y:-767.855957,z:1560.670044},quat:{w:.990901,x:0,y:.13459,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:259.109985,y:-1160.998779,z:872.73999},quat:{w:.997823,x:0,y:.0659521,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:334.958008,y:-779.157715,z:1544.140015},quat:{w:.98965,x:0,y:-.143503,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:543.77002,y:-304.486206,z:2340.919922},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:671.789978,y:-702.809082,z:1691.939941},quat:{w:.996721,x:0,y:-.0809115,z:0}}],v0=[{name:"Player_Start_Location",uniform_scale:1,position:{x:734.325012,y:-461.033691,z:225.824997},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:732.656006,y:-461.06723,z:225.498001},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:731.10199,y:-461.079224,z:225.306},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:729.349976,y:-461.088135,z:225.104996},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:727.888,y:-461.091522,z:225},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:726.346985,y:-461.093994,z:224.899994},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:724.721008,y:-461.095154,z:224.837006},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:722.926025,y:-461.089417,z:224.729996},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:490.509003,y:-283.806305,z:607.773987},quat:{w:1,x:0,y:0,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:430.776001,y:738.914917,z:2657.77002},quat:{w:1,x:0,y:0,z:0}},{name:"Start_Point",uniform_scale:1,position:{x:729.526978,y:-457.720856,z:231.772003},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:432.812012,y:268.643463,z:1832},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:547.317017,y:265.436798,z:1833.48999},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:250,y:271.255768,z:1832},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:130,y:266.659546,z:1832},quat:{w:1,x:0,y:0,z:0}}],y0=[{name:"Player_Start_Location",uniform_scale:1,position:{x:433.334015,y:-513.037354,z:96.240799},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:435.20401,y:-513.162476,z:95.375198},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:437.582001,y:-513.566772,z:94.366997},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:440.243988,y:-513.761963,z:93.888298},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:442.778015,y:-513.983032,z:93.335999},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:248.843994,y:-45.584229,z:1096.150024},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:276.050995,y:-274.875824,z:464.118011},quat:{w:.962966,x:0,y:-.269621,z:0}},{name:"Start_Point",uniform_scale:1,position:{x:439.385986,y:-501.645935,z:114.804001},quat:{w:1,x:0,y:0,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:211.158005,y:292.066498,z:1897.359985},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:449.298004,y:-513.57428,z:94.264702},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:447.554993,y:-513.726257,z:93.949699},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:444.868011,y:-513.97467,z:93.357101},quat:{w:1,x:0,y:0,z:0}}],S0=[{name:"Check_Point_1",uniform_scale:1,position:{x:595.789978,y:-145.276123,z:827.879028},quat:{w:1,x:0,y:0,z:0}},{name:"Start_Point",uniform_scale:1,position:{x:801,y:-481.165039,z:299.283997},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:852.960022,y:-109.809311,z:847.317993},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:954.27002,y:387.389771,z:1827.089966},quat:{w:.966132,x:0,y:.258047,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:334.089996,y:404.545593,z:1607.920044},quat:{w:1,x:0,y:0,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:605.559998,y:1014.658264,z:2583.449951},quat:{w:.999118,x:0,y:-.0419877,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:562.330017,y:1013.05835,z:2580.01001},quat:{w:1,x:0,y:0,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:661.130005,y:1013.058411,z:2589.110107},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:805.97998,y:-484.148041,z:291.459991},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:808.049988,y:-484.111481,z:291.584015},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:799.973999,y:-484.205536,z:290.96701},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:798.021973,y:-484.156891,z:291.47699},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:795.872986,y:-484.126862,z:291.640015},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:793.724976,y:-484.082977,z:291.743011},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:801.661987,y:-484.198792,z:291.009003},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:803.802979,y:-484.175079,z:291.208008},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:15,y:314.300995,z:1607.920044},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:1032.959961,y:-148.53656,z:847.317993},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:1200,y:-192.438019,z:847.317993},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:1400,y:347.761963,z:1827.089966},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:1576,y:349.224152,z:1827.089966},quat:{w:1,x:0,y:0,z:0}}],M0={AlpineEasy:d0,AlpineHard:p0,AlpineMedium:m0,ForestEasy:_0,ForestHard:g0,ForestMedium:x0,VillageEasy:v0,VillageHard:y0,VillageMedium:S0};let Er;function ul(){if(!Er){Er=[];for(const[n,t]of Object.entries(M0))Er.push({name:n,entries:t})}return Er}function Fr(n,t){const e=t.x-n.x,i=t.y-n.y,r=t.z-n.z;return Math.sqrt(e*e+i*i+r*r)}function E0(n,t){const e={x:2*(n.quat.x*n.quat.z+n.quat.w*n.quat.y),y:2*(n.quat.y*n.quat.z-n.quat.w*n.quat.x),z:1-2*(n.quat.x*n.quat.x+n.quat.y*n.quat.y)},i=Math.sqrt(e.x*e.x+e.y*e.y+e.z*e.z);if(i<1e-6)return;const r={x:e.x/i,y:e.y/i,z:e.z/i},s={x:t.p2.x-t.p1.x,y:t.p2.y-t.p1.y,z:t.p2.z-t.p1.z},o=r.x*s.x+r.y*s.y+r.z*s.z;if(Math.abs(o)<1e-6)return;const a={x:t.p1.x-n.position.x,y:t.p1.y-n.position.y,z:t.p1.z-n.position.z},u=-(r.x*a.x+r.y*a.y+r.z*a.z)/o;return u<0||u>1?void 0:{x:t.p1.x+u*s.x,y:t.p1.y+u*s.y,z:t.p1.z+u*s.z}}function T0(n){const t=[],e=n.map(r=>({x:r.x,y:r.y,z:r.z})),i=["Check_Point_1","Check_Point_2","Start_Point","Finish_Point"];for(const r of ul()){const o=r.entries.filter(l=>i.includes(l.name)).map(l=>({planeName:l.name,plane:{position:l.position,quat:l.quat}})),a={name:r.name,collisions:[]};for(let l=0;l<n.length-1;l++){const u=e[l],h=e[l+1];for(const{planeName:p,plane:m}of o){const _=E0(m,{p1:u,p2:h});_!==void 0&&a.collisions.push({intersection:_,distance:Fr(_,m.position),p1:u,p2:h,frame1:l,frame2:l+1,objectName:p,plane:m})}}t.push(a)}return t}const Xe=90.5,Va=2,ac=5;function w0(n){const t=T0(n.coords.rows),e=A0(n,t);return{allCollisions:t,levelScores:e}}function A0(n,t){var i,r,s;let e=[];for(const o of t){const a=b0(o.name,n.coords.rows),l=(i=Is(o,"Start_Point"))==null?void 0:i.at(0),u=(r=Is(o,"Check_Point_1"))==null?void 0:r.at(0),h=(s=Is(o,"Finish_Point"))==null?void 0:s.at(0),p=o.collisions.filter(g=>g.objectName==="Check_Point_2"),m=p.filter(g=>g.distance<=Xe).filter(g=>!l||l.frame2<=g.frame1).filter(g=>!u||u.frame2<=g.frame1).filter(g=>!h||g.frame2<=h.frame2),_={nearestStartDist:a.dist,startPlaneDiffMs:Us(n,l,"Start_Point"),checkpoint1DiffMs:Us(n,u,"Check_Point_1"),finishPointDiffMs:Us(n,h,"Finish_Point"),checkpoint2Exists:m.length>0,firstValidStartPointCollision:l,firstValidCheckPoint1Collision:u,firstValidFinishPointCollision:h,allCheckPoint2Collisions:p,validCheckPoint2Collisions:m},x=fl(_),y=Object.values(x).filter(g=>g).length;e.push({name:o.name,scoreData:_,score:y})}return e=e.sort((o,a)=>a.score-o.score),e}function Is(n,t){return n.collisions.filter(e=>e.objectName===t&&e.distance<=Xe)}function Us(n,t,e){if(!t)return;const{crossStartPlusStartDelayMs:i,totalTimeToFinishMs:r,checkpoint1TotalMs:s}=n.timingDataFromHeader;switch(e){case"Start_Point":return t.frame2*10-i;case"Check_Point_1":return t.frame2*10-s;case"Finish_Point":return t.frame2*10-r;default:throw new Error(`Unhandled ${e}`)}}function b0(n,t){const e=ul().filter(u=>u.name===n);if(e.length!==1)throw new Error(`Expected to find only one level with levelName: ${n}, found ${e.length}`);const r=e[0].entries.filter(u=>u.name==="Player_Start_Location");if(r.length===0)throw new Error("No start locations");let s=r[0];const o={x:t[0].x,y:t[0].y,z:t[0].z};for(const u of r){const h=s.position,p=u.position,m=Fr(o,h);Fr(o,p)<m&&(s=u)}const a=s.position,l=Fr(o,a);return{gameObject:s,pos:a,dist:l}}function fl(n){return{hasPlayerStartLocation:n.nearestStartDist<=Va,hasStartPlane:n.startPlaneDiffMs!==void 0&&Math.abs(n.startPlaneDiffMs)<=10,hasCheckPoint1:n.checkpoint1DiffMs!==void 0&&Math.abs(n.checkpoint1DiffMs)<=10,hasFinishPoint:n.finishPointDiffMs!==void 0&&Math.abs(n.finishPointDiffMs)<=10,hasCheckPoint2:n.checkpoint2Exists}}function C0(n){const t=fl(n),e=[];return e.push([`Has Player Start Location ≤${Va}m`,{valid:t.hasPlayerStartLocation}]),e.push([`Has Start_Point ≤10ms, ≤${Xe}m`,{valid:t.hasStartPlane}]),e.push([`Has Check_Point_1 ≤10ms, ≤${Xe}m`,{valid:t.hasCheckPoint1}]),e.push([`Has Finish_Point ≤10ms, ≤${Xe}m`,{valid:t.hasFinishPoint}]),e.push([`Has Check_Point_2 ≤${Xe}m`,{valid:t.hasCheckPoint2}]),e}const Dn="Unknown Track";function hl(n,t){const{playerName:e,endNameAddr:i}=dl(n),r=R0(n,i),s=r.totalTimeToFinishMs-r.crossStartPlusStartDelayMs,o=L0(n)*10,a=r.crossStartPlusStartDelayMs-o,l=r.totalRecordingTimeMs-r.totalTimeToFinishMs,u={playerName:e,trackName:Dn,displayedMs:s,startMs:a,totalMs:s+a,lagBeforeStartMs:o,lagAfterFinishMs:l,recordingMs:r.totalRecordingTimeMs,checkpoint1Ms:r.checkpoint1TotalMs-r.crossStartPlusStartDelayMs,timingDataFromHeader:r};if(!(t!=null&&t.skipCoords)){const h=D0(n,r);u.coords=h;const p=w0(u);u.trackScoreData=p;const m=p.levelScores[0].score;if(m!==0){const _=p.levelScores.filter(x=>x.score===m).map(x=>x.name).sort();_.length===2&&_[0]==="ForestHard"&&_[1]==="ForestMedium"?u.trackName="Forest MediumOrHard":_.length===1?u.trackName=_[0]:u.trackName="UnknownTrack"}}return u}function dl(n){let t="",e="",i=48;for(;i<n.length;i+=2){if(n.slice(i,i+2)==="00"){e=Buffer.from(t,"hex").toString("utf8");break}t+=n.slice(i,i+2)}return{playerName:e,endNameAddr:i}}function R0(n,t){return{totalRecordingTimeMs:10*Tr(n.slice(8,12)),crossStartPlusStartDelayMs:10*Tr(n.slice(t+2,t+6)),checkpoint1TotalMs:10*Tr(n.slice(t+10,t+14)),totalTimeToFinishMs:10*Tr(n.slice(t+18,t+22))}}function Tr(n){const t=parseInt(n.slice(0,2),16);return parseInt(n.slice(2,4),16)<<8|t}const pl=new Uint32Array(1),P0=new Float32Array(pl.buffer);function Bi(n){let t="";for(let e=n.length-2;e>=0;e-=2)t+=n.substring(e,e+2);return pl[0]=parseInt(t,16),P0[0]}function L0(n){const t=ml(n);for(let e=1;e<t.length-1;++e)if(t[e]!==t[e+1])return e;throw new Error("unreachable")}function ml(n){const{endNameAddr:t}=dl(n),e=26,i=[n.slice(0,t+e)];for(let r=t+e;r<n.length;r+=218)i.push(n.slice(r,r+218));return i}function D0(n,t){var a;const e=ml(n),i=(a=e.at(-1))==null?void 0:a.match(/^[0]+$/),r={rows:[]},s=e.length-1-(i?1:0),o=t.totalRecordingTimeMs/10;s!==o&&console.warn(`Warning: Expected ${o} coordinate blocks, but got ${s}`);for(let l=1;l<e.length-(i?1:0);++l){const u=e[l],h=88,p=h+3*8,m=[];for(let x=0;x<3;x++){const y=[];for(let g=0;g<3;g++){const d=p+(x*3+g)*8;y.push(Bi(u.slice(d,d+8)))}m.push(y)}const _={x:Bi(u.slice(h,h+8)),y:Bi(u.slice(h+8,h+2*8)),z:Bi(u.slice(h+2*8,h+3*8)),rotation3x3:m,ex:u.slice(202,208),raw:u};r.rows.push(_)}return r}async function I0(){return await _l("replays/Village/Medium/VM 1.09.08 Dom.dat")}async function _l(n){const e=await(await fetch(n)).arrayBuffer(),i=Buffer.from(e).toString("hex");return hl(i)}const U0={vm109:"Ja3OmVZS2jQ"};function F0(n,t){console.log("Setup Video");const e=document.createElement("script"),i={videoTarget:void 0};e.src="https://www.youtube.com/iframe_api";const r=document.getElementsByTagName("script")[0];r.parentNode.insertBefore(e,r);let s;return window.onYouTubeIframeAPIReady=()=>{s=new YT.Player("player",{...t,videoId:n,playerVars:{playsinline:1,autoplay:1,rel:0,modestbranding:1,mute:1,controls:1,showinfo:0,fs:1,cc_load_policy:0,iv_load_policy:3,autohide:1,enablejsapi:1},events:{onReady:o=>{o.target.playVideo(),i.videoTarget=o.target},onStateChange:o=>{o.data===YT.PlayerState.ENDED&&(s.seekTo(0,!0),s.playVideo())}}})},i}function N0(n,t){if(t<1||t>=n.length-1)return 0;const e=1/100,i=n[t-1].y,r=n[t].y;return(n[t+1].y-2*r+i)/(e*e)}function oc(n,t,e=100){if(t===0)return 0;const i=n[t-1],r=n[t],s=r.x-i.x,o=r.y-i.y,a=r.z-i.z,l=Math.sqrt(s*s+o*o+a*a),u=1/(e*3600);return l/1e3/u}class O0{constructor(t,e={}){Qr(this,"coordinates");Qr(this,"jumpDetectionConfig");this.coordinates=t,this.jumpDetectionConfig={accelerationThreshold:e.accelerationThreshold??4.5,requiredDuration:e.requiredDuration??50}}calculateAcceleration(t){if(t<1||t>=this.coordinates.length-1)return 0;const e=1/100,i=this.coordinates[t-1].y,r=this.coordinates[t].y;return-(this.coordinates[t+1].y-2*r+i)/(e*e)}detectJumps(){const t=[],{accelerationThreshold:e,requiredDuration:i}=this.jumpDetectionConfig;let r=null,s=0;for(let o=1;o<this.coordinates.length-1;o++){const a=this.calculateAcceleration(o);a>e?(s++,r===null&&(r=o),s===i&&r!==null&&t.push({index:r,position:this.coordinates[r],acceleration:a,type:"takeoff"})):(r!==null&&s>=i&&t.push({index:o,position:this.coordinates[o],acceleration:a,type:"landing"}),r=null,s=0)}return t}createSlopeMesh(t={}){const{width:e=30,roughness:i=.8,metalness:r=.2,playerYOffset:s=1,debug:o=!1}=t,a=new bn,l=this.detectJumps(),u=x=>new H(-x.z,0,x.x).normalize().multiplyScalar(e/2),h=(x,y)=>{const g=[],d=[];for(let b=x;b<=y;b++){const w=this.coordinates[b],D=this.coordinates[Math.min(b+1,y)],L=new H(D.x-w.x,D.y-w.y,D.z-w.z).normalize(),I=u(L);if(g.push(w.x+I.x,w.y-s,w.z+I.z,w.x-I.x,w.y-s,w.z-I.z),b<y){const N=(b-x)*2;d.push(N,N+1,N+2,N+1,N+3,N+2)}}const P=new Le;return P.setAttribute("position",new ye(g,3)),P.setIndex(d),P.computeVertexNormals(),P},p=o?new un({color:16777215,wireframe:!0,side:ve}):new Lo({color:15790320,roughness:.9,metalness:.1,side:ve}),m=o?new un({color:4210752,wireframe:!0,side:ve}):new Lo({color:4210752,roughness:i,metalness:r,side:ve});let _=0;if(l.forEach(x=>{if(x.type==="takeoff"){if(_<x.index){const g=h(_,x.index),d=new ce(g,p);d.receiveShadow=!0,a.add(d)}const y=l.find(g=>g.type==="landing"&&g.index>x.index);if(y){const g=[],d=[],P=this.coordinates[x.index],b=this.coordinates[y.index],w=new H(b.x-P.x,b.y-P.y,b.z-P.z).normalize(),D=u(w),L=u(w);g.push(P.x+D.x,P.y-s,P.z+D.z,P.x-D.x,P.y-s,P.z-D.z,b.x+L.x,b.y-s,b.z+L.z,b.x-L.x,b.y-s,b.z-L.z),d.push(0,1,2,1,3,2);const I=new Le;I.setAttribute("position",new ye(g,3)),I.setIndex(d),I.computeVertexNormals(),a.add(new ce(I,m)),_=y.index}}}),_<this.coordinates.length-1){const x=h(_,this.coordinates.length-1);a.add(new ce(x,p))}return a}createJumpMarkers(t={}){const{width:e=30,takeoffColor:i=65280,landingColor:r=16711680}=t;return this.detectJumps().map(o=>{const a=new bn,l=new Ki(4,3),u=new un({color:o.type==="takeoff"?i:r,side:ve,transparent:!1}),h=new ce(l,u);h.position.y=5,h.position.x=2;const p=12,m=new Xr(.1,.1,p),_=new un({color:13421772}),x=new ce(m,_);x.position.y=-12/2+5,a.add(x),a.add(h);const y=Math.max(0,o.index-1),g=new H(this.coordinates[o.index].x-this.coordinates[y].x,this.coordinates[o.index].y-this.coordinates[y].y,this.coordinates[o.index].z-this.coordinates[y].z).normalize(),d=new H(-g.z,0,g.x).normalize().multiplyScalar(e/2),P=o.type==="takeoff"?-1:1;return a.position.set(o.position.x+d.x*P,o.position.y,o.position.z+d.z*P),a})}}const B0={distance:1.5,height:1.5,smoothing:.995};function z0(n,t,e={}){const i={...B0,...e},r=new H,s=new H,o=new H(0,0,1),a=new Le,l=new nl({color:65280,transparent:!0,opacity:.5}),u=new Float32Array(300);a.setAttribute("position",new $e(u,3));const h=new Mf(a,l),p=[];function m(x,y){const g=new H;y&&(g.set(-x.x- -y.x,-x.y- -y.y,x.z-y.z),g.lengthSq()>.01&&(g.normalize(),o.copy(g))),p.unshift(new H(-x.x,-x.y,x.z)),p.length>100&&p.pop();const d=h.geometry.attributes.position.array;for(let D=0;D<p.length;D++)d[D*3]=p[D].x,d[D*3+1]=p[D].y,d[D*3+2]=p[D].z;h.geometry.attributes.position.needsUpdate=!0;const P=new H(-o.x*i.distance,i.height,-o.z*i.distance),b=new H(-x.x+P.x,-x.y+P.y,x.z+P.z);s.lerp(b,i.smoothing),n.position.copy(s);const w=new H(-x.x,-x.y,x.z);r.lerp(w,i.smoothing),n.lookAt(r)}function _(x){Object.assign(i,x)}return{trail:h,trailPoints:p,updateCamera:m,updateConfig:_,getConfig:()=>({...i})}}function k0(n=.04){const t=new bn,e={headRadius:4,neckRadius:2,neckHeight:6,bodyWidth:5,bodyHeight:12.5,bodyDepth:7.5,boardWidth:6.25,boardHeight:.5,boardLength:20,headY:15,neckY:9,boardY:-7},i=e.headRadius*n,r=32,s=new Hr(i,r,r,0,Math.PI,0,Math.PI),o=new un({color:65280,side:ve}),a=new ce(s,o),l=new Hr(i,r,r,Math.PI,Math.PI,0,Math.PI),u=new un({color:16711680,side:ve}),h=new ce(l,u),p=new bn;p.add(a),p.add(h),p.position.y=e.headY*n,a.castShadow=!0,h.castShadow=!0;const m=new Xr(e.neckRadius*n,e.neckRadius*n,e.neckHeight*n,32),_=new Do,x=new ce(m,_);x.position.y=e.neckY*n;const y=new Kn(e.bodyWidth*n,e.bodyHeight*n,e.bodyDepth*n),g=new Do,d=new ce(y,g),P=new Kn(e.boardWidth*n,e.boardHeight*n,e.boardLength*n),b=new un({color:16776960}),w=new ce(P,b);return w.position.y=e.boardY*n,t.add(d),t.add(x),t.add(p),t.add(w),d.castShadow=!0,x.castShadow=!0,w.castShadow=!0,t}function H0({processCallback:n}){const t=Fs("dropzone"),e=Fs("replayFile"),i=Fs("dropZoneSubLabel");function r(h,p=!1){i.textContent=h,p&&(t.classList.add("error"),setTimeout(()=>{i.textContent="Drag and drop replay file here",t.classList.remove("error")},3e3))}async function s(h){r("Processing...");try{const p=await h.arrayBuffer(),m=Buffer.from(new Uint8Array(p)).toString("hex");await n(m),r(`Processed ${h.name}`)}catch(p){console.error("Error processing file:",p),r("Error processing file",!0)}}function o(h){h.preventDefault(),h.stopPropagation(),t.classList.add("dragover")}function a(h){h.preventDefault(),h.stopPropagation(),t.classList.remove("dragover")}async function l(h){var m;h.preventDefault(),h.stopPropagation(),t.classList.remove("dragover");const p=(m=h.dataTransfer)==null?void 0:m.files[0];p&&await s(p)}async function u(h){var _;const m=(_=h.target.files)==null?void 0:_[0];m&&await s(m)}t.addEventListener("dragover",o),t.addEventListener("dragleave",a),t.addEventListener("drop",l),e.addEventListener("change",u),document.addEventListener("dragover",h=>h.preventDefault()),document.addEventListener("drop",h=>h.preventDefault())}function Fs(n){const t=document.getElementById(n);if(!t)throw new Error(`Element with id '${n}' not found`);return t}function V0(n,t,e){const{offsetSeconds:i,videoId:r,syncWithVideo:s}=gl();xl(n),r.addEventListener("change",()=>{var o;n.videoId=r.value,(o=t.videoTarget)==null||o.loadVideoById(r.value)}),i.addEventListener("change",()=>{n.offsetSeconds=parseFloat(i.value)}),s.addEventListener("change",()=>{n.syncWithVideo=s.checked}),H0({processCallback:async o=>{s.checked=!1,n.syncWithVideo=!1;const a=await hl(o,{skipCoords:!1});e(a)}})}function gl(){const n=document.getElementById("offsetSeconds"),t=document.getElementById("videoId"),e=document.getElementById("replayFile"),i=document.getElementById("syncWithVideo");return{offsetSeconds:n,videoId:t,replayFile:e,syncWithVideo:i}}function xl(n){const{offsetSeconds:t,videoId:e,syncWithVideo:i}=gl();t.value=n.offsetSeconds.toString(),e.value=n.videoId,i.checked=n.syncWithVideo}function G0(n,t,e){const i=document.getElementById("presetSelector"),r=[{name:"Dom VM 1.09.08 with speed & inputs",videoId:"Ja3OmVZS2jQ",datFile:"replays/Village/Medium/VM 1.09.08 Dom.dat",offsetSeconds:-1.05},{name:"Dom AE 1.19.93",videoId:"VQJ3hyQctoY",datFile:"replays/Alpine/Easy/1.19.93 Dom.dat",offsetSeconds:-.21},{name:"Dom AM 1.22.90",videoId:"drLyc1Ty9sw",datFile:"replays/Alpine/Medium/1.22.91 Dom.dat",offsetSeconds:-.21},{name:"Dom FH 0.57.06",videoId:"DIPZX8KbBfQ",datFile:"replays/Forest/Hard/0.57.06 Dom.dat",offsetSeconds:-.21},{name:"Dom VE 1.17.06",videoId:"FtyCM-26eGg",datFile:"replays/Village/Easy/1.17.06 Dom.dat",offsetSeconds:-.22},{name:"Dom VM 1.02.44",videoId:"qblHObF7np8",datFile:"replays/Village/Medium/1.02.44 Dom.dat",offsetSeconds:-.21}];for(const s of r){const o=document.createElement("option");o.value=s.videoId,o.text=s.name,i.add(o)}i.addEventListener("change",async()=>{var a;const s=r.find(l=>l.videoId===i.value);if(!s)return;const o=await _l(s.datFile);n.videoId=s.videoId,n.offsetSeconds=s.offsetSeconds,(a=t.videoTarget)==null||a.loadVideoById(s.videoId),e(o)})}var vl=typeof global=="object"&&global&&global.Object===Object&&global,W0=typeof self=="object"&&self&&self.Object===Object&&self,_n=vl||W0||Function("return this")(),In=_n.Symbol,yl=Object.prototype,X0=yl.hasOwnProperty,q0=yl.toString,Ni=In?In.toStringTag:void 0;function $0(n){var t=X0.call(n,Ni),e=n[Ni];try{n[Ni]=void 0;var i=!0}catch{}var r=q0.call(n);return i&&(t?n[Ni]=e:delete n[Ni]),r}var Y0=Object.prototype,j0=Y0.toString;function K0(n){return j0.call(n)}var Z0="[object Null]",J0="[object Undefined]",cc=In?In.toStringTag:void 0;function Ai(n){return n==null?n===void 0?J0:Z0:cc&&cc in Object(n)?$0(n):K0(n)}function Ei(n){return n!=null&&typeof n=="object"}var Q0="[object Symbol]";function $r(n){return typeof n=="symbol"||Ei(n)&&Ai(n)==Q0}function t_(n,t){for(var e=-1,i=n==null?0:n.length,r=Array(i);++e<i;)r[e]=t(n[e],e,n);return r}var mn=Array.isArray,lc=In?In.prototype:void 0,uc=lc?lc.toString:void 0;function Sl(n){if(typeof n=="string")return n;if(mn(n))return t_(n,Sl)+"";if($r(n))return uc?uc.call(n):"";var t=n+"";return t=="0"&&1/n==-1/0?"-0":t}function Ga(n){var t=typeof n;return n!=null&&(t=="object"||t=="function")}function e_(n){return n}var n_="[object AsyncFunction]",i_="[object Function]",r_="[object GeneratorFunction]",s_="[object Proxy]";function Ml(n){if(!Ga(n))return!1;var t=Ai(n);return t==i_||t==r_||t==n_||t==s_}var Ns=_n["__core-js_shared__"],fc=function(){var n=/[^.]+$/.exec(Ns&&Ns.keys&&Ns.keys.IE_PROTO||"");return n?"Symbol(src)_1."+n:""}();function a_(n){return!!fc&&fc in n}var o_=Function.prototype,c_=o_.toString;function Qn(n){if(n!=null){try{return c_.call(n)}catch{}try{return n+""}catch{}}return""}var l_=/[\\^$.*+?()[\]{}|]/g,u_=/^\[object .+?Constructor\]$/,f_=Function.prototype,h_=Object.prototype,d_=f_.toString,p_=h_.hasOwnProperty,m_=RegExp("^"+d_.call(p_).replace(l_,"\\$&").replace(/hasOwnProperty|(function).*?(?=\\\()| for .+?(?=\\\])/g,"$1.*?")+"$");function __(n){if(!Ga(n)||a_(n))return!1;var t=Ml(n)?m_:u_;return t.test(Qn(n))}function g_(n,t){return n==null?void 0:n[t]}function bi(n,t){var e=g_(n,t);return __(e)?e:void 0}var Aa=bi(_n,"WeakMap"),x_=9007199254740991,v_=/^(?:0|[1-9]\d*)$/;function El(n,t){var e=typeof n;return t=t??x_,!!t&&(e=="number"||e!="symbol"&&v_.test(n))&&n>-1&&n%1==0&&n<t}function Tl(n,t){return n===t||n!==n&&t!==t}var y_=9007199254740991;function Wa(n){return typeof n=="number"&&n>-1&&n%1==0&&n<=y_}function S_(n){return n!=null&&Wa(n.length)&&!Ml(n)}var M_=Object.prototype;function E_(n){var t=n&&n.constructor,e=typeof t=="function"&&t.prototype||M_;return n===e}function T_(n,t){for(var e=-1,i=Array(n);++e<n;)i[e]=t(e);return i}var w_="[object Arguments]";function hc(n){return Ei(n)&&Ai(n)==w_}var wl=Object.prototype,A_=wl.hasOwnProperty,b_=wl.propertyIsEnumerable,Al=hc(function(){return arguments}())?hc:function(n){return Ei(n)&&A_.call(n,"callee")&&!b_.call(n,"callee")};function C_(){return!1}var bl=typeof fn=="object"&&fn&&!fn.nodeType&&fn,dc=bl&&typeof hn=="object"&&hn&&!hn.nodeType&&hn,R_=dc&&dc.exports===bl,pc=R_?_n.Buffer:void 0,P_=pc?pc.isBuffer:void 0,ba=P_||C_,L_="[object Arguments]",D_="[object Array]",I_="[object Boolean]",U_="[object Date]",F_="[object Error]",N_="[object Function]",O_="[object Map]",B_="[object Number]",z_="[object Object]",k_="[object RegExp]",H_="[object Set]",V_="[object String]",G_="[object WeakMap]",W_="[object ArrayBuffer]",X_="[object DataView]",q_="[object Float32Array]",$_="[object Float64Array]",Y_="[object Int8Array]",j_="[object Int16Array]",K_="[object Int32Array]",Z_="[object Uint8Array]",J_="[object Uint8ClampedArray]",Q_="[object Uint16Array]",tg="[object Uint32Array]",Qt={};Qt[q_]=Qt[$_]=Qt[Y_]=Qt[j_]=Qt[K_]=Qt[Z_]=Qt[J_]=Qt[Q_]=Qt[tg]=!0;Qt[L_]=Qt[D_]=Qt[W_]=Qt[I_]=Qt[X_]=Qt[U_]=Qt[F_]=Qt[N_]=Qt[O_]=Qt[B_]=Qt[z_]=Qt[k_]=Qt[H_]=Qt[V_]=Qt[G_]=!1;function eg(n){return Ei(n)&&Wa(n.length)&&!!Qt[Ai(n)]}function ng(n){return function(t){return n(t)}}var Cl=typeof fn=="object"&&fn&&!fn.nodeType&&fn,zi=Cl&&typeof hn=="object"&&hn&&!hn.nodeType&&hn,ig=zi&&zi.exports===Cl,Os=ig&&vl.process,mc=function(){try{var n=zi&&zi.require&&zi.require("util").types;return n||Os&&Os.binding&&Os.binding("util")}catch{}}(),_c=mc&&mc.isTypedArray,Rl=_c?ng(_c):eg,rg=Object.prototype,sg=rg.hasOwnProperty;function ag(n,t){var e=mn(n),i=!e&&Al(n),r=!e&&!i&&ba(n),s=!e&&!i&&!r&&Rl(n),o=e||i||r||s,a=o?T_(n.length,String):[],l=a.length;for(var u in n)sg.call(n,u)&&!(o&&(u=="length"||r&&(u=="offset"||u=="parent")||s&&(u=="buffer"||u=="byteLength"||u=="byteOffset")||El(u,l)))&&a.push(u);return a}function og(n,t){return function(e){return n(t(e))}}var cg=og(Object.keys,Object),lg=Object.prototype,ug=lg.hasOwnProperty;function fg(n){if(!E_(n))return cg(n);var t=[];for(var e in Object(n))ug.call(n,e)&&e!="constructor"&&t.push(e);return t}function Pl(n){return S_(n)?ag(n):fg(n)}var hg=/\.|\[(?:[^[\]]*|(["'])(?:(?!\1)[^\\]|\\.)*?\1)\]/,dg=/^\w*$/;function Xa(n,t){if(mn(n))return!1;var e=typeof n;return e=="number"||e=="symbol"||e=="boolean"||n==null||$r(n)?!0:dg.test(n)||!hg.test(n)||t!=null&&n in Object(t)}var Wi=bi(Object,"create");function pg(){this.__data__=Wi?Wi(null):{},this.size=0}function mg(n){var t=this.has(n)&&delete this.__data__[n];return this.size-=t?1:0,t}var _g="__lodash_hash_undefined__",gg=Object.prototype,xg=gg.hasOwnProperty;function vg(n){var t=this.__data__;if(Wi){var e=t[n];return e===_g?void 0:e}return xg.call(t,n)?t[n]:void 0}var yg=Object.prototype,Sg=yg.hasOwnProperty;function Mg(n){var t=this.__data__;return Wi?t[n]!==void 0:Sg.call(t,n)}var Eg="__lodash_hash_undefined__";function Tg(n,t){var e=this.__data__;return this.size+=this.has(n)?0:1,e[n]=Wi&&t===void 0?Eg:t,this}function Zn(n){var t=-1,e=n==null?0:n.length;for(this.clear();++t<e;){var i=n[t];this.set(i[0],i[1])}}Zn.prototype.clear=pg;Zn.prototype.delete=mg;Zn.prototype.get=vg;Zn.prototype.has=Mg;Zn.prototype.set=Tg;function wg(){this.__data__=[],this.size=0}function Yr(n,t){for(var e=n.length;e--;)if(Tl(n[e][0],t))return e;return-1}var Ag=Array.prototype,bg=Ag.splice;function Cg(n){var t=this.__data__,e=Yr(t,n);if(e<0)return!1;var i=t.length-1;return e==i?t.pop():bg.call(t,e,1),--this.size,!0}function Rg(n){var t=this.__data__,e=Yr(t,n);return e<0?void 0:t[e][1]}function Pg(n){return Yr(this.__data__,n)>-1}function Lg(n,t){var e=this.__data__,i=Yr(e,n);return i<0?(++this.size,e.push([n,t])):e[i][1]=t,this}function gn(n){var t=-1,e=n==null?0:n.length;for(this.clear();++t<e;){var i=n[t];this.set(i[0],i[1])}}gn.prototype.clear=wg;gn.prototype.delete=Cg;gn.prototype.get=Rg;gn.prototype.has=Pg;gn.prototype.set=Lg;var Xi=bi(_n,"Map");function Dg(){this.size=0,this.__data__={hash:new Zn,map:new(Xi||gn),string:new Zn}}function Ig(n){var t=typeof n;return t=="string"||t=="number"||t=="symbol"||t=="boolean"?n!=="__proto__":n===null}function jr(n,t){var e=n.__data__;return Ig(t)?e[typeof t=="string"?"string":"hash"]:e.map}function Ug(n){var t=jr(this,n).delete(n);return this.size-=t?1:0,t}function Fg(n){return jr(this,n).get(n)}function Ng(n){return jr(this,n).has(n)}function Og(n,t){var e=jr(this,n),i=e.size;return e.set(n,t),this.size+=e.size==i?0:1,this}function xn(n){var t=-1,e=n==null?0:n.length;for(this.clear();++t<e;){var i=n[t];this.set(i[0],i[1])}}xn.prototype.clear=Dg;xn.prototype.delete=Ug;xn.prototype.get=Fg;xn.prototype.has=Ng;xn.prototype.set=Og;var Bg="Expected a function";function qa(n,t){if(typeof n!="function"||t!=null&&typeof t!="function")throw new TypeError(Bg);var e=function(){var i=arguments,r=t?t.apply(this,i):i[0],s=e.cache;if(s.has(r))return s.get(r);var o=n.apply(this,i);return e.cache=s.set(r,o)||s,o};return e.cache=new(qa.Cache||xn),e}qa.Cache=xn;var zg=500;function kg(n){var t=qa(n,function(i){return e.size===zg&&e.clear(),i}),e=t.cache;return t}var Hg=/[^.[\]]+|\[(?:(-?\d+(?:\.\d+)?)|(["'])((?:(?!\2)[^\\]|\\.)*?)\2)\]|(?=(?:\.|\[\])(?:\.|\[\]|$))/g,Vg=/\\(\\)?/g,Gg=kg(function(n){var t=[];return n.charCodeAt(0)===46&&t.push(""),n.replace(Hg,function(e,i,r,s){t.push(r?s.replace(Vg,"$1"):i||e)}),t});function Wg(n){return n==null?"":Sl(n)}function Ll(n,t){return mn(n)?n:Xa(n,t)?[n]:Gg(Wg(n))}function Kr(n){if(typeof n=="string"||$r(n))return n;var t=n+"";return t=="0"&&1/n==-1/0?"-0":t}function Dl(n,t){t=Ll(t,n);for(var e=0,i=t.length;n!=null&&e<i;)n=n[Kr(t[e++])];return e&&e==i?n:void 0}function Xg(n,t,e){var i=n==null?void 0:Dl(n,t);return i===void 0?e:i}function qg(n,t){for(var e=-1,i=t.length,r=n.length;++e<i;)n[r+e]=t[e];return n}function $g(){this.__data__=new gn,this.size=0}function Yg(n){var t=this.__data__,e=t.delete(n);return this.size=t.size,e}function jg(n){return this.__data__.get(n)}function Kg(n){return this.__data__.has(n)}var Zg=200;function Jg(n,t){var e=this.__data__;if(e instanceof gn){var i=e.__data__;if(!Xi||i.length<Zg-1)return i.push([n,t]),this.size=++e.size,this;e=this.__data__=new xn(i)}return e.set(n,t),this.size=e.size,this}function pn(n){var t=this.__data__=new gn(n);this.size=t.size}pn.prototype.clear=$g;pn.prototype.delete=Yg;pn.prototype.get=jg;pn.prototype.has=Kg;pn.prototype.set=Jg;function Qg(n,t){for(var e=-1,i=n==null?0:n.length,r=0,s=[];++e<i;){var o=n[e];t(o,e,n)&&(s[r++]=o)}return s}function tx(){return[]}var ex=Object.prototype,nx=ex.propertyIsEnumerable,gc=Object.getOwnPropertySymbols,ix=gc?function(n){return n==null?[]:(n=Object(n),Qg(gc(n),function(t){return nx.call(n,t)}))}:tx;function rx(n,t,e){var i=t(n);return mn(n)?i:qg(i,e(n))}function xc(n){return rx(n,Pl,ix)}var Ca=bi(_n,"DataView"),Ra=bi(_n,"Promise"),Pa=bi(_n,"Set"),vc="[object Map]",sx="[object Object]",yc="[object Promise]",Sc="[object Set]",Mc="[object WeakMap]",Ec="[object DataView]",ax=Qn(Ca),ox=Qn(Xi),cx=Qn(Ra),lx=Qn(Pa),ux=Qn(Aa),wn=Ai;(Ca&&wn(new Ca(new ArrayBuffer(1)))!=Ec||Xi&&wn(new Xi)!=vc||Ra&&wn(Ra.resolve())!=yc||Pa&&wn(new Pa)!=Sc||Aa&&wn(new Aa)!=Mc)&&(wn=function(n){var t=Ai(n),e=t==sx?n.constructor:void 0,i=e?Qn(e):"";if(i)switch(i){case ax:return Ec;case ox:return vc;case cx:return yc;case lx:return Sc;case ux:return Mc}return t});var Tc=_n.Uint8Array,fx="__lodash_hash_undefined__";function hx(n){return this.__data__.set(n,fx),this}function dx(n){return this.__data__.has(n)}function Vr(n){var t=-1,e=n==null?0:n.length;for(this.__data__=new xn;++t<e;)this.add(n[t])}Vr.prototype.add=Vr.prototype.push=hx;Vr.prototype.has=dx;function px(n,t){for(var e=-1,i=n==null?0:n.length;++e<i;)if(t(n[e],e,n))return!0;return!1}function mx(n,t){return n.has(t)}var _x=1,gx=2;function Il(n,t,e,i,r,s){var o=e&_x,a=n.length,l=t.length;if(a!=l&&!(o&&l>a))return!1;var u=s.get(n),h=s.get(t);if(u&&h)return u==t&&h==n;var p=-1,m=!0,_=e&gx?new Vr:void 0;for(s.set(n,t),s.set(t,n);++p<a;){var x=n[p],y=t[p];if(i)var g=o?i(y,x,p,t,n,s):i(x,y,p,n,t,s);if(g!==void 0){if(g)continue;m=!1;break}if(_){if(!px(t,function(d,P){if(!mx(_,P)&&(x===d||r(x,d,e,i,s)))return _.push(P)})){m=!1;break}}else if(!(x===y||r(x,y,e,i,s))){m=!1;break}}return s.delete(n),s.delete(t),m}function xx(n){var t=-1,e=Array(n.size);return n.forEach(function(i,r){e[++t]=[r,i]}),e}function vx(n){var t=-1,e=Array(n.size);return n.forEach(function(i){e[++t]=i}),e}var yx=1,Sx=2,Mx="[object Boolean]",Ex="[object Date]",Tx="[object Error]",wx="[object Map]",Ax="[object Number]",bx="[object RegExp]",Cx="[object Set]",Rx="[object String]",Px="[object Symbol]",Lx="[object ArrayBuffer]",Dx="[object DataView]",wc=In?In.prototype:void 0,Bs=wc?wc.valueOf:void 0;function Ix(n,t,e,i,r,s,o){switch(e){case Dx:if(n.byteLength!=t.byteLength||n.byteOffset!=t.byteOffset)return!1;n=n.buffer,t=t.buffer;case Lx:return!(n.byteLength!=t.byteLength||!s(new Tc(n),new Tc(t)));case Mx:case Ex:case Ax:return Tl(+n,+t);case Tx:return n.name==t.name&&n.message==t.message;case bx:case Rx:return n==t+"";case wx:var a=xx;case Cx:var l=i&yx;if(a||(a=vx),n.size!=t.size&&!l)return!1;var u=o.get(n);if(u)return u==t;i|=Sx,o.set(n,t);var h=Il(a(n),a(t),i,r,s,o);return o.delete(n),h;case Px:if(Bs)return Bs.call(n)==Bs.call(t)}return!1}var Ux=1,Fx=Object.prototype,Nx=Fx.hasOwnProperty;function Ox(n,t,e,i,r,s){var o=e&Ux,a=xc(n),l=a.length,u=xc(t),h=u.length;if(l!=h&&!o)return!1;for(var p=l;p--;){var m=a[p];if(!(o?m in t:Nx.call(t,m)))return!1}var _=s.get(n),x=s.get(t);if(_&&x)return _==t&&x==n;var y=!0;s.set(n,t),s.set(t,n);for(var g=o;++p<l;){m=a[p];var d=n[m],P=t[m];if(i)var b=o?i(P,d,m,t,n,s):i(d,P,m,n,t,s);if(!(b===void 0?d===P||r(d,P,e,i,s):b)){y=!1;break}g||(g=m=="constructor")}if(y&&!g){var w=n.constructor,D=t.constructor;w!=D&&"constructor"in n&&"constructor"in t&&!(typeof w=="function"&&w instanceof w&&typeof D=="function"&&D instanceof D)&&(y=!1)}return s.delete(n),s.delete(t),y}var Bx=1,Ac="[object Arguments]",bc="[object Array]",wr="[object Object]",zx=Object.prototype,Cc=zx.hasOwnProperty;function kx(n,t,e,i,r,s){var o=mn(n),a=mn(t),l=o?bc:wn(n),u=a?bc:wn(t);l=l==Ac?wr:l,u=u==Ac?wr:u;var h=l==wr,p=u==wr,m=l==u;if(m&&ba(n)){if(!ba(t))return!1;o=!0,h=!1}if(m&&!h)return s||(s=new pn),o||Rl(n)?Il(n,t,e,i,r,s):Ix(n,t,l,e,i,r,s);if(!(e&Bx)){var _=h&&Cc.call(n,"__wrapped__"),x=p&&Cc.call(t,"__wrapped__");if(_||x){var y=_?n.value():n,g=x?t.value():t;return s||(s=new pn),r(y,g,e,i,s)}}return m?(s||(s=new pn),Ox(n,t,e,i,r,s)):!1}function $a(n,t,e,i,r){return n===t?!0:n==null||t==null||!Ei(n)&&!Ei(t)?n!==n&&t!==t:kx(n,t,e,i,$a,r)}var Hx=1,Vx=2;function Gx(n,t,e,i){var r=e.length,s=r;if(n==null)return!s;for(n=Object(n);r--;){var o=e[r];if(o[2]?o[1]!==n[o[0]]:!(o[0]in n))return!1}for(;++r<s;){o=e[r];var a=o[0],l=n[a],u=o[1];if(o[2]){if(l===void 0&&!(a in n))return!1}else{var h=new pn,p;if(!(p===void 0?$a(u,l,Hx|Vx,i,h):p))return!1}}return!0}function Ul(n){return n===n&&!Ga(n)}function Wx(n){for(var t=Pl(n),e=t.length;e--;){var i=t[e],r=n[i];t[e]=[i,r,Ul(r)]}return t}function Fl(n,t){return function(e){return e==null?!1:e[n]===t&&(t!==void 0||n in Object(e))}}function Xx(n){var t=Wx(n);return t.length==1&&t[0][2]?Fl(t[0][0],t[0][1]):function(e){return e===n||Gx(e,n,t)}}function qx(n,t){return n!=null&&t in Object(n)}function $x(n,t,e){t=Ll(t,n);for(var i=-1,r=t.length,s=!1;++i<r;){var o=Kr(t[i]);if(!(s=n!=null&&e(n,o)))break;n=n[o]}return s||++i!=r?s:(r=n==null?0:n.length,!!r&&Wa(r)&&El(o,r)&&(mn(n)||Al(n)))}function Yx(n,t){return n!=null&&$x(n,t,qx)}var jx=1,Kx=2;function Zx(n,t){return Xa(n)&&Ul(t)?Fl(Kr(n),t):function(e){var i=Xg(e,n);return i===void 0&&i===t?Yx(e,n):$a(t,i,jx|Kx)}}function Jx(n){return function(t){return t==null?void 0:t[n]}}function Qx(n){return function(t){return Dl(t,n)}}function tv(n){return Xa(n)?Jx(Kr(n)):Qx(n)}function ev(n){return typeof n=="function"?n:n==null?e_:typeof n=="object"?mn(n)?Zx(n[0],n[1]):Xx(n):tv(n)}function nv(n,t){return n<t}function iv(n,t,e){for(var i=-1,r=n.length;++i<r;){var s=n[i],o=t(s);if(o!=null&&(a===void 0?o===o&&!$r(o):e(o,a)))var a=o,l=s}return l}function rv(n,t){return n&&n.length?iv(n,ev(t),nv):void 0}/**
 * @license lucide v0.511.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */const Nl={xmlns:"http://www.w3.org/2000/svg",width:24,height:24,viewBox:"0 0 24 24",fill:"none",stroke:"currentColor","stroke-width":2,"stroke-linecap":"round","stroke-linejoin":"round"};/**
 * @license lucide v0.511.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */const Ol=([n,t,e])=>{const i=document.createElementNS("http://www.w3.org/2000/svg",n);return Object.keys(t).forEach(r=>{i.setAttribute(r,String(t[r]))}),e!=null&&e.length&&e.forEach(r=>{const s=Ol(r);i.appendChild(s)}),i},sv=(n,t={})=>{const e="svg",i={...Nl,...t};return Ol([e,i,n])};/**
 * @license lucide v0.511.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */const av=n=>Array.from(n.attributes).reduce((t,e)=>(t[e.name]=e.value,t),{}),ov=n=>typeof n=="string"?n:!n||!n.class?"":n.class&&typeof n.class=="string"?n.class.split(" "):n.class&&Array.isArray(n.class)?n.class:"",cv=n=>n.flatMap(ov).map(e=>e.trim()).filter(Boolean).filter((e,i,r)=>r.indexOf(e)===i).join(" "),lv=n=>n.replace(/(\w)(\w*)(_|-|\s*)/g,(t,e,i)=>e.toUpperCase()+i.toLowerCase()),Rc=(n,{nameAttr:t,icons:e,attrs:i})=>{var p;const r=n.getAttribute(t);if(r==null)return;const s=lv(r),o=e[s];if(!o)return console.warn(`${n.outerHTML} icon name was not found in the provided icons object.`);const a=av(n),l={...Nl,"data-lucide":r,...i,...a},u=cv(["lucide",`lucide-${r}`,a,i]);u&&Object.assign(l,{class:u});const h=sv(o,l);return(p=n.parentNode)==null?void 0:p.replaceChild(h,n)};/**
 * @license lucide v0.511.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */const uv=[["circle",{cx:"12",cy:"12",r:"10"}],["path",{d:"M12 16v-4"}],["path",{d:"M12 8h.01"}]];/**
 * @license lucide v0.511.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */const fv=[["rect",{x:"14",y:"4",width:"4",height:"16",rx:"1"}],["rect",{x:"6",y:"4",width:"4",height:"16",rx:"1"}]];/**
 * @license lucide v0.511.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */const hv=[["polygon",{points:"6 3 20 12 6 21 6 3"}]];/**
 * @license lucide v0.511.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */const dv=[["polygon",{points:"19 20 9 12 19 4 19 20"}],["line",{x1:"5",x2:"5",y1:"19",y2:"5"}]];/**
 * @license lucide v0.511.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */const pv=[["polygon",{points:"5 4 15 12 5 20 5 4"}],["line",{x1:"19",x2:"19",y1:"5",y2:"19"}]];/**
 * @license lucide v0.511.0 - ISC
 *
 * This source code is licensed under the ISC license.
 * See the LICENSE file in the root directory of this source tree.
 */const mv=({icons:n={},nameAttr:t="data-lucide",attrs:e={}}={})=>{if(!Object.values(n).length)throw new Error(`Please provide an icons object.
If you want to use all the icons you can import it like:
 \`import { createIcons, icons } from 'lucide';
lucide.createIcons({icons});\``);if(typeof document>"u")throw new Error("`createIcons()` only works in a browser environment.");const i=document.querySelectorAll(`[${t}]`);if(Array.from(i).forEach(r=>Rc(r,{nameAttr:t,icons:n,attrs:e})),t==="data-lucide"){const r=document.querySelectorAll("[icon-name]");r.length>0&&(console.warn("[Lucide] Some icons were found with the now deprecated icon-name attribute. These will still be replaced for backwards compatibility, but will no longer be supported in v1.0 and you should switch to data-lucide"),Array.from(r).forEach(s=>Rc(s,{nameAttr:"icon-name",icons:n,attrs:e})))}},xe={width:480,height:360},Ge={offsetSeconds:-1.05,videoId:U0.vm109,syncWithVideo:!0},Pc=.02,Ze=F0(Ge.videoId,xe),ge={isPlaying:!0,resumedTime:0,lastFrameRendered:-5e3,lastFrameChecked:-5e3};async function _v(){document.getElementById("mainContainer").style.width=`${xe.width*2}px`;const n=xv(),t=await I0(),e={textFields:n,analyzeResult:t};let i;const r=s=>{i==null||i.dispose(),e.analyzeResult=s,i=gv(e)};G0(Ge,Ze,r),V0(Ge,Ze,r),r(t)}function gv(n){const{textFields:t,analyzeResult:e}=n;xl(Ge),t.nameText.textContent=e.playerName,t.timeText.textContent=Lc(e,0);const i=document.getElementById("playPauseButton"),r=document.getElementById("backButton"),s=document.getElementById("forwardButton"),o=document.getElementById("playerRange"),a=document.getElementById("frameInfo");document.getElementsByClassName("keyLeft")[0],document.getElementsByClassName("keyRight")[0],document.getElementsByClassName("keyShift")[0],i.onclick=()=>{var L,I;if(ge.isPlaying){if(ge.isPlaying=!1,Ge.syncWithVideo){(L=Ze.videoTarget)==null||L.pauseVideo();const{expectedFrame:N}=Bl();D(0,Math.floor(N))}}else ge.resumedTime=performance.now()-10*ge.lastFrameRendered,ge.isPlaying=!0,Ge.syncWithVideo&&((I=Ze.videoTarget)==null||I.playVideo());i.innerHTML=`<i data-lucide="${ge.isPlaying?"pause":"play"}"></i>`,Ic()},s.onclick=()=>{D(0,(ge.lastFrameRendered+1)%u.length)},r.onclick=()=>{D(0,(ge.lastFrameRendered-1+u.length)%u.length)};const l=document.getElementById("preText"),u=e.coords.rows.slice(0,-1);document.getElementById("headerInfo").innerHTML=Ev(e),document.getElementById("scoreInfo").innerHTML=Av(e),Ic(),o.min="0",o.max=u.length.toString();const h=new Ne(75,window.innerWidth/window.innerHeight,.01,1e3),p=new vf,m=document.getElementById("keyControls");m.style.bottom="0",m.style.right=`${xe.width}px`;const _=z0(h),x=new O0(u.map(L=>kl(L)),{accelerationThreshold:.05}),y=x.createSlopeMesh({playerYOffset:.4}),g=x.createJumpMarkers({});p.add(y),g.forEach(L=>p.add(L));const d=new Cf(16777215,1);d.castShadow=!0,p.add(d);const P=u[0],b=k0();Dc(b,P),p.add(b);const w=new h0({antialias:!0});w.setSize(xe.width,xe.height),w.setAnimationLoop(L=>D(L)),w.shadowMap.enabled=!0,document.getElementById("container").prepend(w.domElement),o.addEventListener("click",L=>{var A;const I=L.offsetX/o.offsetWidth,N=Math.floor(I*u.length);ge.resumedTime=performance.now()-10*N,Ge.syncWithVideo&&((A=Ze.videoTarget)==null||A.seekTo(N/100+Ge.offsetSeconds,!0)),ge.lastFrameChecked=N,D(0,N)});function D(L,I=void 0){if(I===void 0&&!ge.isPlaying)return;const N=I??Math.floor((L-ge.resumedTime)/10)%u.length;if(N<0||N>u.length-1)return;const A=u[N];yv(b,A),Dc(b,A);const T=oc(u,N);N>0&&_.updateCamera(u[N],u[N-1],b,T);const O=zl(N),$=`x: ${b.position.x}
y: ${b.position.y}
z: ${b.position.z}
accel(y): ${N0(u,N).toFixed(1)}
drift(s): ${O?(O.actualSeconds-O.expectedSeconds).toFixed(3):"N/A"}
rotation3x3:
[
  ${A.rotation3x3[0].map(W=>W.toFixed(3)).join(", ")}
  ${A.rotation3x3[1].map(W=>W.toFixed(3)).join(", ")}
  ${A.rotation3x3[2].map(W=>W.toFixed(3)).join(", ")}
]
${vv(A.raw)}`;o.value=N.toString(),a.innerHTML=`Frame: ${N} / ${u.length}`,t.timeText.textContent=Lc(e,N),t.speedText1.textContent=`${Math.floor(oc(u,N)).toFixed(0).padStart(3,"0")} km/h`,t.speedText2.textContent=t.speedText1.textContent.replace(" ",""),$!==l.innerHTML&&(l.innerHTML=$),Ze.videoTarget!==void 0&&Math.abs(ge.lastFrameChecked-N)>100&&(Mv(N),ge.lastFrameChecked=N),ge.lastFrameRendered=N,w.render(p,h)}return{dispose:()=>{console.log("DISPOSE!!!!"),w.domElement.remove(),w.dispose()}}}function xv(){const n=.07777777777777778*xe.height,t=124/540*xe.height,e=152/540*xe.height,i=22/720*xe.width,r=20/720*xe.width,s=22/540*xe.height,o=10/540*xe.height,a=Ar("",{top:`${n}px`,right:`${xe.width+i}px`,fontSize:`${s}px`}),l=Ar("",{top:`${n+i}px`,right:`${xe.width+i}px`,fontSize:`${s}px`}),u=Ar("000km/h",{top:`${e}px`,right:`${xe.width+i}px`,fontSize:`${s}px`}),h=Ar("000km/h",{top:`${t}px`,left:`${xe.width+r}px`,backgroundColor:"black",letterSpacing:"1px",padding:`0 ${o}px`,fontSize:`${s}px`});return{nameText:a,timeText:l,speedText1:u,speedText2:h}}function Ar(n,t){const e=document.createElement("div");return e.style.position="absolute",e.style.color="white",e.style.fontSize="22px",Object.assign(e.style,t),e.textContent=n,document.getElementById("container").appendChild(e),e}function Lc(n,t){let e=Math.max(0,10*t-(n.lagBeforeStartMs+n.startMs));return e=Math.min(e,n.displayedMs),on(e)}function on(n){return new Date(n).toISOString().slice(14,22).replace(".",":")}function vv(n){const t=new Array;for(let a=0;a<n.length;a+=8)t.push(Bi(n.slice(a,a+8)));const i=Math.ceil(t.length/7);let r=t.map(a=>a.toFixed(3));const s=Math.max(10,...r.map(a=>a.length));r=r.map(a=>a.padStart(s," ")).map((a,l)=>l>=11&&l<=22?`<mark>${a}</mark>`:a);const o=new Array;for(let a=0;a<t.length;a+=i)o.push(r.slice(a,a+i).join(", "));return`frame: ${n.length/2} bytes, ${t.length} 4-byte floats:
`+o.join(`
`)}function yv(n,t){const e=t.rotation3x3,i=new te().set(e[0][0],e[0][1],e[0][2],0,e[1][0],e[1][1],e[1][2],0,e[2][0],e[2][1],e[2][2],0,0,0,0,1),r=new Ye().setFromRotationMatrix(i,"XYZ");r.x=-r.x,r.y=-r.y;const s=new te().makeRotationFromEuler(r);n.setRotationFromMatrix(s)}let Sv=0;function Mv(n){if(Ze.videoTarget===void 0||!Ge.syncWithVideo)return;const t=zl(n);t&&Math.abs(t.actualSeconds-t.expectedSeconds)>Pc&&(console.log(`Synced video to replay, precision: ${Pc}, times: ${++Sv}.`),Bl(t.actualSeconds))}function Bl(n){n=n??Ze.videoTarget.getCurrentTime();const t=(n-Ge.offsetSeconds)*100;return ge.resumedTime=performance.now()-10*t,{expectedFrame:t}}function zl(n){const t=Ge.offsetSeconds+n/100;if(!Ze.videoTarget)return;const e=Ze.videoTarget.getCurrentTime();return{expectedSeconds:t,actualSeconds:e}}function Dc(n,t){const e=kl(t);n.position.x=e.x,n.position.y=e.y,n.position.z=e.z}function kl(n){return{x:-n.x,y:-n.y,z:n.z}}function Ev(n){var m;const{timingDataFromHeader:t}=n,e=t.totalTimeToFinishMs!==0,i=t.checkpoint1TotalMs!==0?on(n.checkpoint1Ms-10):"N/A (No CP1 data recorded)",r=Hl(n),s=r?on(r.frame2*10-n.timingDataFromHeader.crossStartPlusStartDelayMs):n.trackName===Dn?"N/A (Unknown Track)":"N/A (Could not find CP2)",o=n.trackName!==Dn?(m=n.trackScoreData)==null?void 0:m.levelScores[0]:void 0,a=e?on(n.displayedMs):"N/A (Did not finish)",l=on(n.startMs),u=e?on(n.totalMs):"N/A (Did not finish)";console.log(n);const h=[o==null?void 0:o.scoreData.firstValidCheckPoint1Collision,r,o==null?void 0:o.scoreData.firstValidFinishPointCollision],p=Tv(n);return`Player: ${n.playerName}
Track : ${n.trackName}
CP1   : ${i}${br(h,o==null?void 0:o.scoreData.firstValidCheckPoint1Collision,`Distance to Check_Point_1, must be ≤${Xe}m`)}
CP2   : ${s}${br(h,r,`Distance to Check_Point_2, must be ≤${Xe}m`)}
Time  : ${a}${br(h,o==null?void 0:o.scoreData.firstValidFinishPointCollision,`Distance to Finish_Point, must be ≤${Xe}m`)}

Legitimate Run? : ${p?`<span style='color:red'>No (${p})</span>`:"Yes"}
Start Time      : ${l}${br([],o==null?void 0:o.scoreData.firstValidStartPointCollision,`Distance to Start_Point, must be ≤${Xe}m`)}
Total Time      : ${u}
Lag before start: ${on(n.lagBeforeStartMs)}${wv(n)}
Leg after finish: ${on(n.lagAfterFinishMs)}
Recording time  : ${on(n.recordingMs)}
`}function Tv(n){var e;if(n.trackName===Dn)return"UnknownTrack";const t=(e=n.trackScoreData)==null?void 0:e.levelScores[0].score;if(t!==ac)return`Score ${t} is not max score of ${ac}`}function Hl(n){var l,u;if(n.trackName===Dn)return;const t=(l=n.trackScoreData)==null?void 0:l.levelScores[0];if(!t)return;const{scoreData:e}=t,i=(u=t.scoreData.validCheckPoint2Collisions)==null?void 0:u.at(0);if(i)return i;const{firstValidStartPointCollision:r,firstValidCheckPoint1Collision:s,firstValidFinishPointCollision:o}=e,a=t.scoreData.allCheckPoint2Collisions.filter(h=>!r||r.frame2<=h.frame1).filter(h=>!s||s.frame2<=h.frame1).filter(h=>!o||h.frame2<=o.frame2);return rv(a,h=>h.distance)}function br(n,t,e){if(!t)return"";const i=Math.max(...n.map(s=>(s==null?void 0:s.distance.toFixed(2).length)??0)),r=` (${t.distance.toFixed(2).padStart(i," ")}m)`;return Vl(r,e,t.distance>Xe)}function wv(n){var i;if(n.trackName===Dn)return"";const t=(i=n.trackScoreData)==null?void 0:i.levelScores[0],e=` (${t==null?void 0:t.scoreData.nearestStartDist.toFixed(2)}m)`;return Vl(e,`Distance to nearest Start_Point, must be ≤${Va}m`,((t==null?void 0:t.scoreData.nearestStartDist)??5)>2)}function Vl(n,t,e){const i=e?"color:darkorange":"color:green",r=`<span data-tooltip='${t}'> <i data-lucide="info" style='width:12px; height:12px; vertical-align:middle'></i></span>`;return`<span style='${i}'>${n}</span>${r}`}function Av(n){var a,l;const t=(a=n.trackScoreData)==null?void 0:a.levelScores,i=`
  <table>
    <thead>
      <th>Score</th>
      <th style='text-align: left'>Track</th>
    </thead>
    <tbody>
      ${[...new Set((l=n.trackScoreData)==null?void 0:l.levelScores.map(u=>u.score))].sort((u,h)=>h-u).map(u=>`<tr>
            <td>${u}</td>
            <td>${t==null?void 0:t.filter(h=>h.score===u).map(h=>h.name).sort().join(", ")}</td>
          </tr>`).join("")}
    </tbody>
  </table>`,r=Cv(n),s=Rv(n,t[0]),o=bv(n);return i+r+o+s}function bv(n){var s;if(n.trackName===Dn)return"";const t=(s=n.trackScoreData)==null?void 0:s.levelScores[0];if((t==null?void 0:t.score)!==5)return"";const e=Hl(n),i=o=>(o/1e3).toFixed(2);return`
    <div style='margin-top:15px'>High Times</div>
    <pre>${[`"${n.playerName}"`,i(n.checkpoint1Ms),i(e.frame2*10-n.timingDataFromHeader.crossStartPlusStartDelayMs),i(n.displayedMs),"2025/1/1",0].join(",")},</pre>
`}function Cv(n){var s;if(n.trackName===Dn)return"";const t=(s=n.trackScoreData)==null?void 0:s.levelScores[0];if(!t)return"";const e=t.scoreData,i=C0(e);return`

  <div style='margin-top:15px'>Score breakdown for ${t.name}</div>
  <table>
    <thead>
      <th>Metric</th>
      <th>Value</th>
    </thead>
    <tbody>
      ${i.map(o=>`<tr><td>${o[0]}</td><td>${o[1].valid?"✔️":"❌"}</td></tr>`).join("")}
    </tbody>
  </table>
  <div style='font-size:10px'>Explanation of the track logic is in <a target='_blank' href='https://github.com/domsleee/SS-Dat-Info/wiki/Dat%E2%80%90Info-design-notes#determining-trackname-of-a-dat-file'>the wiki</a>.</div>`}function Rv(n,t){if(!t)return"";const i=n.trackScoreData.allCollisions.find(s=>s.name===t.name).collisions.map(s=>[`${s.objectName}<br /><span style='font-size:8px'>(${s.plane.position.x.toFixed(2)},${s.plane.position.y.toFixed(2)},${s.plane.position.z.toFixed(2)})</span>`,s.frame1,s.frame2,s.distance.toFixed(2)]);return`
  <div style='margin-top:15px'>Collisions for ${t.name}</div>
  <table>
    <thead>
      <th>Plane</th>
      <th>Frame1</th>
      <th>Frame2</th>
      <th>Distance</th>
    </thead>
    <tbody>
      ${i.map(s=>`<tr>${s.map(o=>`<td>${o}</td>`).join("")}</tr>`).join(`
`)}
    </tbody>
  </table`}function Ic(){try{mv({icons:{Play:hv,Pause:fv,SkipBack:dv,SkipForward:pv,Info:uv}})}catch(n){console.error(n)}}_v()});export default Pv();
