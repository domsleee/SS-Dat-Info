var Gl=(n,t)=>()=>(t||n((t={exports:{}}).exports,t),t.exports);import"./modulepreload-polyfill-3cfb730f.js";var Ov=Gl((an,on)=>{var bc={},qr={};qr.byteLength=ql;qr.toByteArray=$l;qr.fromByteArray=jl;var Xe=[],Le=[],Vl=typeof Uint8Array<"u"?Uint8Array:Array,ss="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";for(var Jn=0,Wl=ss.length;Jn<Wl;++Jn)Xe[Jn]=ss[Jn],Le[ss.charCodeAt(Jn)]=Jn;Le["-".charCodeAt(0)]=62;Le["_".charCodeAt(0)]=63;function Cc(n){var t=n.length;if(t%4>0)throw new Error("Invalid string. Length must be a multiple of 4");var e=n.indexOf("=");e===-1&&(e=t);var i=e===t?0:4-e%4;return[e,i]}function ql(n){var t=Cc(n),e=t[0],i=t[1];return(e+i)*3/4-i}function Xl(n,t,e){return(t+e)*3/4-e}function $l(n){var t,e=Cc(n),i=e[0],r=e[1],s=new Vl(Xl(n,i,r)),o=0,a=r>0?i-4:i,c;for(c=0;c<a;c+=4)t=Le[n.charCodeAt(c)]<<18|Le[n.charCodeAt(c+1)]<<12|Le[n.charCodeAt(c+2)]<<6|Le[n.charCodeAt(c+3)],s[o++]=t>>16&255,s[o++]=t>>8&255,s[o++]=t&255;return r===2&&(t=Le[n.charCodeAt(c)]<<2|Le[n.charCodeAt(c+1)]>>4,s[o++]=t&255),r===1&&(t=Le[n.charCodeAt(c)]<<10|Le[n.charCodeAt(c+1)]<<4|Le[n.charCodeAt(c+2)]>>2,s[o++]=t>>8&255,s[o++]=t&255),s}function Yl(n){return Xe[n>>18&63]+Xe[n>>12&63]+Xe[n>>6&63]+Xe[n&63]}function Kl(n,t,e){for(var i,r=[],s=t;s<e;s+=3)i=(n[s]<<16&16711680)+(n[s+1]<<8&65280)+(n[s+2]&255),r.push(Yl(i));return r.join("")}function jl(n){for(var t,e=n.length,i=e%3,r=[],s=16383,o=0,a=e-i;o<a;o+=s)r.push(Kl(n,o,o+s>a?a:o+s));return i===1?(t=n[e-1],r.push(Xe[t>>2]+Xe[t<<4&63]+"==")):i===2&&(t=(n[e-2]<<8)+n[e-1],r.push(Xe[t>>10]+Xe[t>>4&63]+Xe[t<<2&63]+"=")),r.join("")}var Aa={};/*! ieee754. BSD-3-Clause License. Feross Aboukhadijeh <https://feross.org/opensource> */Aa.read=function(n,t,e,i,r){var s,o,a=r*8-i-1,c=(1<<a)-1,u=c>>1,f=-7,p=e?r-1:0,m=e?-1:1,g=n[t+p];for(p+=m,s=g&(1<<-f)-1,g>>=-f,f+=a;f>0;s=s*256+n[t+p],p+=m,f-=8);for(o=s&(1<<-f)-1,s>>=-f,f+=i;f>0;o=o*256+n[t+p],p+=m,f-=8);if(s===0)s=1-u;else{if(s===c)return o?NaN:(g?-1:1)*(1/0);o=o+Math.pow(2,i),s=s-u}return(g?-1:1)*o*Math.pow(2,s-i)};Aa.write=function(n,t,e,i,r,s){var o,a,c,u=s*8-r-1,f=(1<<u)-1,p=f>>1,m=r===23?Math.pow(2,-24)-Math.pow(2,-77):0,g=i?0:s-1,v=i?1:-1,M=t<0||t===0&&1/t<0?1:0;for(t=Math.abs(t),isNaN(t)||t===1/0?(a=isNaN(t)?1:0,o=f):(o=Math.floor(Math.log(t)/Math.LN2),t*(c=Math.pow(2,-o))<1&&(o--,c*=2),o+p>=1?t+=m/c:t+=m*Math.pow(2,1-p),t*c>=2&&(o++,c/=2),o+p>=f?(a=0,o=f):o+p>=1?(a=(t*c-1)*Math.pow(2,r),o=o+p):(a=t*Math.pow(2,p-1)*Math.pow(2,r),o=0));r>=8;n[e+g]=a&255,g+=v,a/=256,r-=8);for(o=o<<r|a,u+=r;u>0;n[e+g]=o&255,g+=v,o/=256,u-=8);n[e+g-v]|=M*128};/*!
 * The buffer module from node.js, for the browser.
 *
 * @author   Feross Aboukhadijeh <https://feross.org>
 * @license  MIT
 */(function(n){const t=qr,e=Aa,i=typeof Symbol=="function"&&typeof Symbol.for=="function"?Symbol.for("nodejs.util.inspect.custom"):null;n.Buffer=a,n.SlowBuffer=C,n.INSPECT_MAX_BYTES=50;const r=2147483647;n.kMaxLength=r,a.TYPED_ARRAY_SUPPORT=s(),!a.TYPED_ARRAY_SUPPORT&&typeof console<"u"&&typeof console.error=="function"&&console.error("This browser lacks typed array (Uint8Array) support which is required by `buffer` v5.x. Use `buffer` v4.x if you require old browser support.");function s(){try{const x=new Uint8Array(1),l={foo:function(){return 42}};return Object.setPrototypeOf(l,Uint8Array.prototype),Object.setPrototypeOf(x,l),x.foo()===42}catch{return!1}}Object.defineProperty(a.prototype,"parent",{enumerable:!0,get:function(){if(a.isBuffer(this))return this.buffer}}),Object.defineProperty(a.prototype,"offset",{enumerable:!0,get:function(){if(a.isBuffer(this))return this.byteOffset}});function o(x){if(x>r)throw new RangeError('The value "'+x+'" is invalid for option "size"');const l=new Uint8Array(x);return Object.setPrototypeOf(l,a.prototype),l}function a(x,l,h){if(typeof x=="number"){if(typeof l=="string")throw new TypeError('The "string" argument must be of type string. Received type number');return p(x)}return c(x,l,h)}a.poolSize=8192;function c(x,l,h){if(typeof x=="string")return m(x,l);if(ArrayBuffer.isView(x))return v(x);if(x==null)throw new TypeError("The first argument must be one of type string, Buffer, ArrayBuffer, Array, or Array-like Object. Received type "+typeof x);if(V(x,ArrayBuffer)||x&&V(x.buffer,ArrayBuffer)||typeof SharedArrayBuffer<"u"&&(V(x,SharedArrayBuffer)||x&&V(x.buffer,SharedArrayBuffer)))return M(x,l,h);if(typeof x=="number")throw new TypeError('The "value" argument must not be of type number. Received type number');const y=x.valueOf&&x.valueOf();if(y!=null&&y!==x)return a.from(y,l,h);const R=_(x);if(R)return R;if(typeof Symbol<"u"&&Symbol.toPrimitive!=null&&typeof x[Symbol.toPrimitive]=="function")return a.from(x[Symbol.toPrimitive]("string"),l,h);throw new TypeError("The first argument must be one of type string, Buffer, ArrayBuffer, Array, or Array-like Object. Received type "+typeof x)}a.from=function(x,l,h){return c(x,l,h)},Object.setPrototypeOf(a.prototype,Uint8Array.prototype),Object.setPrototypeOf(a,Uint8Array);function u(x){if(typeof x!="number")throw new TypeError('"size" argument must be of type number');if(x<0)throw new RangeError('The value "'+x+'" is invalid for option "size"')}function f(x,l,h){return u(x),x<=0?o(x):l!==void 0?typeof h=="string"?o(x).fill(l,h):o(x).fill(l):o(x)}a.alloc=function(x,l,h){return f(x,l,h)};function p(x){return u(x),o(x<0?0:d(x)|0)}a.allocUnsafe=function(x){return p(x)},a.allocUnsafeSlow=function(x){return p(x)};function m(x,l){if((typeof l!="string"||l==="")&&(l="utf8"),!a.isEncoding(l))throw new TypeError("Unknown encoding: "+l);const h=w(x,l)|0;let y=o(h);const R=y.write(x,l);return R!==h&&(y=y.slice(0,R)),y}function g(x){const l=x.length<0?0:d(x.length)|0,h=o(l);for(let y=0;y<l;y+=1)h[y]=x[y]&255;return h}function v(x){if(V(x,Uint8Array)){const l=new Uint8Array(x);return M(l.buffer,l.byteOffset,l.byteLength)}return g(x)}function M(x,l,h){if(l<0||x.byteLength<l)throw new RangeError('"offset" is outside of buffer bounds');if(x.byteLength<l+(h||0))throw new RangeError('"length" is outside of buffer bounds');let y;return l===void 0&&h===void 0?y=new Uint8Array(x):h===void 0?y=new Uint8Array(x,l):y=new Uint8Array(x,l,h),Object.setPrototypeOf(y,a.prototype),y}function _(x){if(a.isBuffer(x)){const l=d(x.length)|0,h=o(l);return h.length===0||x.copy(h,0,0,l),h}if(x.length!==void 0)return typeof x.length!="number"||it(x.length)?o(0):g(x);if(x.type==="Buffer"&&Array.isArray(x.data))return g(x.data)}function d(x){if(x>=r)throw new RangeError("Attempt to allocate Buffer larger than maximum size: 0x"+r.toString(16)+" bytes");return x|0}function C(x){return+x!=x&&(x=0),a.alloc(+x)}a.isBuffer=function(l){return l!=null&&l._isBuffer===!0&&l!==a.prototype},a.compare=function(l,h){if(V(l,Uint8Array)&&(l=a.from(l,l.offset,l.byteLength)),V(h,Uint8Array)&&(h=a.from(h,h.offset,h.byteLength)),!a.isBuffer(l)||!a.isBuffer(h))throw new TypeError('The "buf1", "buf2" arguments must be one of type Buffer or Uint8Array');if(l===h)return 0;let y=l.length,R=h.length;for(let O=0,G=Math.min(y,R);O<G;++O)if(l[O]!==h[O]){y=l[O],R=h[O];break}return y<R?-1:R<y?1:0},a.isEncoding=function(l){switch(String(l).toLowerCase()){case"hex":case"utf8":case"utf-8":case"ascii":case"latin1":case"binary":case"base64":case"ucs2":case"ucs-2":case"utf16le":case"utf-16le":return!0;default:return!1}},a.concat=function(l,h){if(!Array.isArray(l))throw new TypeError('"list" argument must be an Array of Buffers');if(l.length===0)return a.alloc(0);let y;if(h===void 0)for(h=0,y=0;y<l.length;++y)h+=l[y].length;const R=a.allocUnsafe(h);let O=0;for(y=0;y<l.length;++y){let G=l[y];if(V(G,Uint8Array))O+G.length>R.length?(a.isBuffer(G)||(G=a.from(G)),G.copy(R,O)):Uint8Array.prototype.set.call(R,G,O);else if(a.isBuffer(G))G.copy(R,O);else throw new TypeError('"list" argument must be an Array of Buffers');O+=G.length}return R};function w(x,l){if(a.isBuffer(x))return x.length;if(ArrayBuffer.isView(x)||V(x,ArrayBuffer))return x.byteLength;if(typeof x!="string")throw new TypeError('The "string" argument must be one of type string, Buffer, or ArrayBuffer. Received type '+typeof x);const h=x.length,y=arguments.length>2&&arguments[2]===!0;if(!y&&h===0)return 0;let R=!1;for(;;)switch(l){case"ascii":case"latin1":case"binary":return h;case"utf8":case"utf-8":return Jt(x).length;case"ucs2":case"ucs-2":case"utf16le":case"utf-16le":return h*2;case"hex":return h>>>1;case"base64":return L(x).length;default:if(R)return y?-1:Jt(x).length;l=(""+l).toLowerCase(),R=!0}}a.byteLength=w;function b(x,l,h){let y=!1;if((l===void 0||l<0)&&(l=0),l>this.length||((h===void 0||h>this.length)&&(h=this.length),h<=0)||(h>>>=0,l>>>=0,h<=l))return"";for(x||(x="utf8");;)switch(x){case"hex":return dt(this,l,h);case"utf8":case"utf-8":return Y(this,l,h);case"ascii":return st(this,l,h);case"latin1":case"binary":return j(this,l,h);case"base64":return W(this,l,h);case"ucs2":case"ucs-2":case"utf16le":case"utf-16le":return _t(this,l,h);default:if(y)throw new TypeError("Unknown encoding: "+x);x=(x+"").toLowerCase(),y=!0}}a.prototype._isBuffer=!0;function z(x,l,h){const y=x[l];x[l]=x[h],x[h]=y}a.prototype.swap16=function(){const l=this.length;if(l%2!==0)throw new RangeError("Buffer size must be a multiple of 16-bits");for(let h=0;h<l;h+=2)z(this,h,h+1);return this},a.prototype.swap32=function(){const l=this.length;if(l%4!==0)throw new RangeError("Buffer size must be a multiple of 32-bits");for(let h=0;h<l;h+=4)z(this,h,h+3),z(this,h+1,h+2);return this},a.prototype.swap64=function(){const l=this.length;if(l%8!==0)throw new RangeError("Buffer size must be a multiple of 64-bits");for(let h=0;h<l;h+=8)z(this,h,h+7),z(this,h+1,h+6),z(this,h+2,h+5),z(this,h+3,h+4);return this},a.prototype.toString=function(){const l=this.length;return l===0?"":arguments.length===0?Y(this,0,l):b.apply(this,arguments)},a.prototype.toLocaleString=a.prototype.toString,a.prototype.equals=function(l){if(!a.isBuffer(l))throw new TypeError("Argument must be a Buffer");return this===l?!0:a.compare(this,l)===0},a.prototype.inspect=function(){let l="";const h=n.INSPECT_MAX_BYTES;return l=this.toString("hex",0,h).replace(/(.{2})/g,"$1 ").trim(),this.length>h&&(l+=" ... "),"<Buffer "+l+">"},i&&(a.prototype[i]=a.prototype.inspect),a.prototype.compare=function(l,h,y,R,O){if(V(l,Uint8Array)&&(l=a.from(l,l.offset,l.byteLength)),!a.isBuffer(l))throw new TypeError('The "target" argument must be one of type Buffer or Uint8Array. Received type '+typeof l);if(h===void 0&&(h=0),y===void 0&&(y=l?l.length:0),R===void 0&&(R=0),O===void 0&&(O=this.length),h<0||y>l.length||R<0||O>this.length)throw new RangeError("out of range index");if(R>=O&&h>=y)return 0;if(R>=O)return-1;if(h>=y)return 1;if(h>>>=0,y>>>=0,R>>>=0,O>>>=0,this===l)return 0;let G=O-R,nt=y-h;const mt=Math.min(G,nt),xt=this.slice(R,O),Rt=l.slice(h,y);for(let U=0;U<mt;++U)if(xt[U]!==Rt[U]){G=xt[U],nt=Rt[U];break}return G<nt?-1:nt<G?1:0};function P(x,l,h,y,R){if(x.length===0)return-1;if(typeof h=="string"?(y=h,h=0):h>2147483647?h=2147483647:h<-2147483648&&(h=-2147483648),h=+h,it(h)&&(h=R?0:x.length-1),h<0&&(h=x.length+h),h>=x.length){if(R)return-1;h=x.length-1}else if(h<0)if(R)h=0;else return-1;if(typeof l=="string"&&(l=a.from(l,y)),a.isBuffer(l))return l.length===0?-1:I(x,l,h,y,R);if(typeof l=="number")return l=l&255,typeof Uint8Array.prototype.indexOf=="function"?R?Uint8Array.prototype.indexOf.call(x,l,h):Uint8Array.prototype.lastIndexOf.call(x,l,h):I(x,[l],h,y,R);throw new TypeError("val must be string, number or Buffer")}function I(x,l,h,y,R){let O=1,G=x.length,nt=l.length;if(y!==void 0&&(y=String(y).toLowerCase(),y==="ucs2"||y==="ucs-2"||y==="utf16le"||y==="utf-16le")){if(x.length<2||l.length<2)return-1;O=2,G/=2,nt/=2,h/=2}function mt(Rt,U){return O===1?Rt[U]:Rt.readUInt16BE(U*O)}let xt;if(R){let Rt=-1;for(xt=h;xt<G;xt++)if(mt(x,xt)===mt(l,Rt===-1?0:xt-Rt)){if(Rt===-1&&(Rt=xt),xt-Rt+1===nt)return Rt*O}else Rt!==-1&&(xt-=xt-Rt),Rt=-1}else for(h+nt>G&&(h=G-nt),xt=h;xt>=0;xt--){let Rt=!0;for(let U=0;U<nt;U++)if(mt(x,xt+U)!==mt(l,U)){Rt=!1;break}if(Rt)return xt}return-1}a.prototype.includes=function(l,h,y){return this.indexOf(l,h,y)!==-1},a.prototype.indexOf=function(l,h,y){return P(this,l,h,y,!0)},a.prototype.lastIndexOf=function(l,h,y){return P(this,l,h,y,!1)};function F(x,l,h,y){h=Number(h)||0;const R=x.length-h;y?(y=Number(y),y>R&&(y=R)):y=R;const O=l.length;y>O/2&&(y=O/2);let G;for(G=0;G<y;++G){const nt=parseInt(l.substr(G*2,2),16);if(it(nt))return G;x[h+G]=nt}return G}function A(x,l,h,y){return S(Jt(l,x.length-h),x,h,y)}function E(x,l,h,y){return S(Lt(l),x,h,y)}function N(x,l,h,y){return S(L(l),x,h,y)}function $(x,l,h,y){return S(It(l,x.length-h),x,h,y)}a.prototype.write=function(l,h,y,R){if(h===void 0)R="utf8",y=this.length,h=0;else if(y===void 0&&typeof h=="string")R=h,y=this.length,h=0;else if(isFinite(h))h=h>>>0,isFinite(y)?(y=y>>>0,R===void 0&&(R="utf8")):(R=y,y=void 0);else throw new Error("Buffer.write(string, encoding, offset[, length]) is no longer supported");const O=this.length-h;if((y===void 0||y>O)&&(y=O),l.length>0&&(y<0||h<0)||h>this.length)throw new RangeError("Attempt to write outside buffer bounds");R||(R="utf8");let G=!1;for(;;)switch(R){case"hex":return F(this,l,h,y);case"utf8":case"utf-8":return A(this,l,h,y);case"ascii":case"latin1":case"binary":return E(this,l,h,y);case"base64":return N(this,l,h,y);case"ucs2":case"ucs-2":case"utf16le":case"utf-16le":return $(this,l,h,y);default:if(G)throw new TypeError("Unknown encoding: "+R);R=(""+R).toLowerCase(),G=!0}},a.prototype.toJSON=function(){return{type:"Buffer",data:Array.prototype.slice.call(this._arr||this,0)}};function W(x,l,h){return l===0&&h===x.length?t.fromByteArray(x):t.fromByteArray(x.slice(l,h))}function Y(x,l,h){h=Math.min(x.length,h);const y=[];let R=l;for(;R<h;){const O=x[R];let G=null,nt=O>239?4:O>223?3:O>191?2:1;if(R+nt<=h){let mt,xt,Rt,U;switch(nt){case 1:O<128&&(G=O);break;case 2:mt=x[R+1],(mt&192)===128&&(U=(O&31)<<6|mt&63,U>127&&(G=U));break;case 3:mt=x[R+1],xt=x[R+2],(mt&192)===128&&(xt&192)===128&&(U=(O&15)<<12|(mt&63)<<6|xt&63,U>2047&&(U<55296||U>57343)&&(G=U));break;case 4:mt=x[R+1],xt=x[R+2],Rt=x[R+3],(mt&192)===128&&(xt&192)===128&&(Rt&192)===128&&(U=(O&15)<<18|(mt&63)<<12|(xt&63)<<6|Rt&63,U>65535&&U<1114112&&(G=U))}}G===null?(G=65533,nt=1):G>65535&&(G-=65536,y.push(G>>>10&1023|55296),G=56320|G&1023),y.push(G),R+=nt}return K(y)}const tt=4096;function K(x){const l=x.length;if(l<=tt)return String.fromCharCode.apply(String,x);let h="",y=0;for(;y<l;)h+=String.fromCharCode.apply(String,x.slice(y,y+=tt));return h}function st(x,l,h){let y="";h=Math.min(x.length,h);for(let R=l;R<h;++R)y+=String.fromCharCode(x[R]&127);return y}function j(x,l,h){let y="";h=Math.min(x.length,h);for(let R=l;R<h;++R)y+=String.fromCharCode(x[R]);return y}function dt(x,l,h){const y=x.length;(!l||l<0)&&(l=0),(!h||h<0||h>y)&&(h=y);let R="";for(let O=l;O<h;++O)R+=rt[x[O]];return R}function _t(x,l,h){const y=x.slice(l,h);let R="";for(let O=0;O<y.length-1;O+=2)R+=String.fromCharCode(y[O]+y[O+1]*256);return R}a.prototype.slice=function(l,h){const y=this.length;l=~~l,h=h===void 0?y:~~h,l<0?(l+=y,l<0&&(l=0)):l>y&&(l=y),h<0?(h+=y,h<0&&(h=0)):h>y&&(h=y),h<l&&(h=l);const R=this.subarray(l,h);return Object.setPrototypeOf(R,a.prototype),R};function ut(x,l,h){if(x%1!==0||x<0)throw new RangeError("offset is not uint");if(x+l>h)throw new RangeError("Trying to access beyond buffer length")}a.prototype.readUintLE=a.prototype.readUIntLE=function(l,h,y){l=l>>>0,h=h>>>0,y||ut(l,h,this.length);let R=this[l],O=1,G=0;for(;++G<h&&(O*=256);)R+=this[l+G]*O;return R},a.prototype.readUintBE=a.prototype.readUIntBE=function(l,h,y){l=l>>>0,h=h>>>0,y||ut(l,h,this.length);let R=this[l+--h],O=1;for(;h>0&&(O*=256);)R+=this[l+--h]*O;return R},a.prototype.readUint8=a.prototype.readUInt8=function(l,h){return l=l>>>0,h||ut(l,1,this.length),this[l]},a.prototype.readUint16LE=a.prototype.readUInt16LE=function(l,h){return l=l>>>0,h||ut(l,2,this.length),this[l]|this[l+1]<<8},a.prototype.readUint16BE=a.prototype.readUInt16BE=function(l,h){return l=l>>>0,h||ut(l,2,this.length),this[l]<<8|this[l+1]},a.prototype.readUint32LE=a.prototype.readUInt32LE=function(l,h){return l=l>>>0,h||ut(l,4,this.length),(this[l]|this[l+1]<<8|this[l+2]<<16)+this[l+3]*16777216},a.prototype.readUint32BE=a.prototype.readUInt32BE=function(l,h){return l=l>>>0,h||ut(l,4,this.length),this[l]*16777216+(this[l+1]<<16|this[l+2]<<8|this[l+3])},a.prototype.readBigUInt64LE=Q(function(l){l=l>>>0,Yt(l,"offset");const h=this[l],y=this[l+7];(h===void 0||y===void 0)&&Ot(l,this.length-8);const R=h+this[++l]*2**8+this[++l]*2**16+this[++l]*2**24,O=this[++l]+this[++l]*2**8+this[++l]*2**16+y*2**24;return BigInt(R)+(BigInt(O)<<BigInt(32))}),a.prototype.readBigUInt64BE=Q(function(l){l=l>>>0,Yt(l,"offset");const h=this[l],y=this[l+7];(h===void 0||y===void 0)&&Ot(l,this.length-8);const R=h*2**24+this[++l]*2**16+this[++l]*2**8+this[++l],O=this[++l]*2**24+this[++l]*2**16+this[++l]*2**8+y;return(BigInt(R)<<BigInt(32))+BigInt(O)}),a.prototype.readIntLE=function(l,h,y){l=l>>>0,h=h>>>0,y||ut(l,h,this.length);let R=this[l],O=1,G=0;for(;++G<h&&(O*=256);)R+=this[l+G]*O;return O*=128,R>=O&&(R-=Math.pow(2,8*h)),R},a.prototype.readIntBE=function(l,h,y){l=l>>>0,h=h>>>0,y||ut(l,h,this.length);let R=h,O=1,G=this[l+--R];for(;R>0&&(O*=256);)G+=this[l+--R]*O;return O*=128,G>=O&&(G-=Math.pow(2,8*h)),G},a.prototype.readInt8=function(l,h){return l=l>>>0,h||ut(l,1,this.length),this[l]&128?(255-this[l]+1)*-1:this[l]},a.prototype.readInt16LE=function(l,h){l=l>>>0,h||ut(l,2,this.length);const y=this[l]|this[l+1]<<8;return y&32768?y|4294901760:y},a.prototype.readInt16BE=function(l,h){l=l>>>0,h||ut(l,2,this.length);const y=this[l+1]|this[l]<<8;return y&32768?y|4294901760:y},a.prototype.readInt32LE=function(l,h){return l=l>>>0,h||ut(l,4,this.length),this[l]|this[l+1]<<8|this[l+2]<<16|this[l+3]<<24},a.prototype.readInt32BE=function(l,h){return l=l>>>0,h||ut(l,4,this.length),this[l]<<24|this[l+1]<<16|this[l+2]<<8|this[l+3]},a.prototype.readBigInt64LE=Q(function(l){l=l>>>0,Yt(l,"offset");const h=this[l],y=this[l+7];(h===void 0||y===void 0)&&Ot(l,this.length-8);const R=this[l+4]+this[l+5]*2**8+this[l+6]*2**16+(y<<24);return(BigInt(R)<<BigInt(32))+BigInt(h+this[++l]*2**8+this[++l]*2**16+this[++l]*2**24)}),a.prototype.readBigInt64BE=Q(function(l){l=l>>>0,Yt(l,"offset");const h=this[l],y=this[l+7];(h===void 0||y===void 0)&&Ot(l,this.length-8);const R=(h<<24)+this[++l]*2**16+this[++l]*2**8+this[++l];return(BigInt(R)<<BigInt(32))+BigInt(this[++l]*2**24+this[++l]*2**16+this[++l]*2**8+y)}),a.prototype.readFloatLE=function(l,h){return l=l>>>0,h||ut(l,4,this.length),e.read(this,l,!0,23,4)},a.prototype.readFloatBE=function(l,h){return l=l>>>0,h||ut(l,4,this.length),e.read(this,l,!1,23,4)},a.prototype.readDoubleLE=function(l,h){return l=l>>>0,h||ut(l,8,this.length),e.read(this,l,!0,52,8)},a.prototype.readDoubleBE=function(l,h){return l=l>>>0,h||ut(l,8,this.length),e.read(this,l,!1,52,8)};function wt(x,l,h,y,R,O){if(!a.isBuffer(x))throw new TypeError('"buffer" argument must be a Buffer instance');if(l>R||l<O)throw new RangeError('"value" argument is out of bounds');if(h+y>x.length)throw new RangeError("Index out of range")}a.prototype.writeUintLE=a.prototype.writeUIntLE=function(l,h,y,R){if(l=+l,h=h>>>0,y=y>>>0,!R){const nt=Math.pow(2,8*y)-1;wt(this,l,h,y,nt,0)}let O=1,G=0;for(this[h]=l&255;++G<y&&(O*=256);)this[h+G]=l/O&255;return h+y},a.prototype.writeUintBE=a.prototype.writeUIntBE=function(l,h,y,R){if(l=+l,h=h>>>0,y=y>>>0,!R){const nt=Math.pow(2,8*y)-1;wt(this,l,h,y,nt,0)}let O=y-1,G=1;for(this[h+O]=l&255;--O>=0&&(G*=256);)this[h+O]=l/G&255;return h+y},a.prototype.writeUint8=a.prototype.writeUInt8=function(l,h,y){return l=+l,h=h>>>0,y||wt(this,l,h,1,255,0),this[h]=l&255,h+1},a.prototype.writeUint16LE=a.prototype.writeUInt16LE=function(l,h,y){return l=+l,h=h>>>0,y||wt(this,l,h,2,65535,0),this[h]=l&255,this[h+1]=l>>>8,h+2},a.prototype.writeUint16BE=a.prototype.writeUInt16BE=function(l,h,y){return l=+l,h=h>>>0,y||wt(this,l,h,2,65535,0),this[h]=l>>>8,this[h+1]=l&255,h+2},a.prototype.writeUint32LE=a.prototype.writeUInt32LE=function(l,h,y){return l=+l,h=h>>>0,y||wt(this,l,h,4,4294967295,0),this[h+3]=l>>>24,this[h+2]=l>>>16,this[h+1]=l>>>8,this[h]=l&255,h+4},a.prototype.writeUint32BE=a.prototype.writeUInt32BE=function(l,h,y){return l=+l,h=h>>>0,y||wt(this,l,h,4,4294967295,0),this[h]=l>>>24,this[h+1]=l>>>16,this[h+2]=l>>>8,this[h+3]=l&255,h+4};function kt(x,l,h,y,R){D(l,y,R,x,h,7);let O=Number(l&BigInt(4294967295));x[h++]=O,O=O>>8,x[h++]=O,O=O>>8,x[h++]=O,O=O>>8,x[h++]=O;let G=Number(l>>BigInt(32)&BigInt(4294967295));return x[h++]=G,G=G>>8,x[h++]=G,G=G>>8,x[h++]=G,G=G>>8,x[h++]=G,h}function Z(x,l,h,y,R){D(l,y,R,x,h,7);let O=Number(l&BigInt(4294967295));x[h+7]=O,O=O>>8,x[h+6]=O,O=O>>8,x[h+5]=O,O=O>>8,x[h+4]=O;let G=Number(l>>BigInt(32)&BigInt(4294967295));return x[h+3]=G,G=G>>8,x[h+2]=G,G=G>>8,x[h+1]=G,G=G>>8,x[h]=G,h+8}a.prototype.writeBigUInt64LE=Q(function(l,h=0){return kt(this,l,h,BigInt(0),BigInt("0xffffffffffffffff"))}),a.prototype.writeBigUInt64BE=Q(function(l,h=0){return Z(this,l,h,BigInt(0),BigInt("0xffffffffffffffff"))}),a.prototype.writeIntLE=function(l,h,y,R){if(l=+l,h=h>>>0,!R){const mt=Math.pow(2,8*y-1);wt(this,l,h,y,mt-1,-mt)}let O=0,G=1,nt=0;for(this[h]=l&255;++O<y&&(G*=256);)l<0&&nt===0&&this[h+O-1]!==0&&(nt=1),this[h+O]=(l/G>>0)-nt&255;return h+y},a.prototype.writeIntBE=function(l,h,y,R){if(l=+l,h=h>>>0,!R){const mt=Math.pow(2,8*y-1);wt(this,l,h,y,mt-1,-mt)}let O=y-1,G=1,nt=0;for(this[h+O]=l&255;--O>=0&&(G*=256);)l<0&&nt===0&&this[h+O+1]!==0&&(nt=1),this[h+O]=(l/G>>0)-nt&255;return h+y},a.prototype.writeInt8=function(l,h,y){return l=+l,h=h>>>0,y||wt(this,l,h,1,127,-128),l<0&&(l=255+l+1),this[h]=l&255,h+1},a.prototype.writeInt16LE=function(l,h,y){return l=+l,h=h>>>0,y||wt(this,l,h,2,32767,-32768),this[h]=l&255,this[h+1]=l>>>8,h+2},a.prototype.writeInt16BE=function(l,h,y){return l=+l,h=h>>>0,y||wt(this,l,h,2,32767,-32768),this[h]=l>>>8,this[h+1]=l&255,h+2},a.prototype.writeInt32LE=function(l,h,y){return l=+l,h=h>>>0,y||wt(this,l,h,4,2147483647,-2147483648),this[h]=l&255,this[h+1]=l>>>8,this[h+2]=l>>>16,this[h+3]=l>>>24,h+4},a.prototype.writeInt32BE=function(l,h,y){return l=+l,h=h>>>0,y||wt(this,l,h,4,2147483647,-2147483648),l<0&&(l=4294967295+l+1),this[h]=l>>>24,this[h+1]=l>>>16,this[h+2]=l>>>8,this[h+3]=l&255,h+4},a.prototype.writeBigInt64LE=Q(function(l,h=0){return kt(this,l,h,-BigInt("0x8000000000000000"),BigInt("0x7fffffffffffffff"))}),a.prototype.writeBigInt64BE=Q(function(l,h=0){return Z(this,l,h,-BigInt("0x8000000000000000"),BigInt("0x7fffffffffffffff"))});function at(x,l,h,y,R,O){if(h+y>x.length)throw new RangeError("Index out of range");if(h<0)throw new RangeError("Index out of range")}function gt(x,l,h,y,R){return l=+l,h=h>>>0,R||at(x,l,h,4),e.write(x,l,h,y,23,4),h+4}a.prototype.writeFloatLE=function(l,h,y){return gt(this,l,h,!0,y)},a.prototype.writeFloatBE=function(l,h,y){return gt(this,l,h,!1,y)};function pt(x,l,h,y,R){return l=+l,h=h>>>0,R||at(x,l,h,8),e.write(x,l,h,y,52,8),h+8}a.prototype.writeDoubleLE=function(l,h,y){return pt(this,l,h,!0,y)},a.prototype.writeDoubleBE=function(l,h,y){return pt(this,l,h,!1,y)},a.prototype.copy=function(l,h,y,R){if(!a.isBuffer(l))throw new TypeError("argument should be a Buffer");if(y||(y=0),!R&&R!==0&&(R=this.length),h>=l.length&&(h=l.length),h||(h=0),R>0&&R<y&&(R=y),R===y||l.length===0||this.length===0)return 0;if(h<0)throw new RangeError("targetStart out of bounds");if(y<0||y>=this.length)throw new RangeError("Index out of range");if(R<0)throw new RangeError("sourceEnd out of bounds");R>this.length&&(R=this.length),l.length-h<R-y&&(R=l.length-h+y);const O=R-y;return this===l&&typeof Uint8Array.prototype.copyWithin=="function"?this.copyWithin(h,y,R):Uint8Array.prototype.set.call(l,this.subarray(y,R),h),O},a.prototype.fill=function(l,h,y,R){if(typeof l=="string"){if(typeof h=="string"?(R=h,h=0,y=this.length):typeof y=="string"&&(R=y,y=this.length),R!==void 0&&typeof R!="string")throw new TypeError("encoding must be a string");if(typeof R=="string"&&!a.isEncoding(R))throw new TypeError("Unknown encoding: "+R);if(l.length===1){const G=l.charCodeAt(0);(R==="utf8"&&G<128||R==="latin1")&&(l=G)}}else typeof l=="number"?l=l&255:typeof l=="boolean"&&(l=Number(l));if(h<0||this.length<h||this.length<y)throw new RangeError("Out of range index");if(y<=h)return this;h=h>>>0,y=y===void 0?this.length:y>>>0,l||(l=0);let O;if(typeof l=="number")for(O=h;O<y;++O)this[O]=l;else{const G=a.isBuffer(l)?l:a.from(l,R),nt=G.length;if(nt===0)throw new TypeError('The value "'+l+'" is invalid for argument "value"');for(O=0;O<y-h;++O)this[O+h]=G[O%nt]}return this};const At={};function Dt(x,l,h){At[x]=class extends h{constructor(){super(),Object.defineProperty(this,"message",{value:l.apply(this,arguments),writable:!0,configurable:!0}),this.name=`${this.name} [${x}]`,this.stack,delete this.name}get code(){return x}set code(R){Object.defineProperty(this,"code",{configurable:!0,enumerable:!0,value:R,writable:!0})}toString(){return`${this.name} [${x}]: ${this.message}`}}}Dt("ERR_BUFFER_OUT_OF_BOUNDS",function(x){return x?`${x} is outside of buffer bounds`:"Attempt to access memory outside buffer bounds"},RangeError),Dt("ERR_INVALID_ARG_TYPE",function(x,l){return`The "${x}" argument must be of type number. Received type ${typeof l}`},TypeError),Dt("ERR_OUT_OF_RANGE",function(x,l,h){let y=`The value of "${x}" is out of range.`,R=h;return Number.isInteger(h)&&Math.abs(h)>2**32?R=Ft(String(h)):typeof h=="bigint"&&(R=String(h),(h>BigInt(2)**BigInt(32)||h<-(BigInt(2)**BigInt(32)))&&(R=Ft(R)),R+="n"),y+=` It must be ${l}. Received ${R}`,y},RangeError);function Ft(x){let l="",h=x.length;const y=x[0]==="-"?1:0;for(;h>=y+4;h-=3)l=`_${x.slice(h-3,h)}${l}`;return`${x.slice(0,h)}${l}`}function Zt(x,l,h){Yt(l,"offset"),(x[l]===void 0||x[l+h]===void 0)&&Ot(l,x.length-(h+1))}function D(x,l,h,y,R,O){if(x>h||x<l){const G=typeof l=="bigint"?"n":"";let nt;throw O>3?l===0||l===BigInt(0)?nt=`>= 0${G} and < 2${G} ** ${(O+1)*8}${G}`:nt=`>= -(2${G} ** ${(O+1)*8-1}${G}) and < 2 ** ${(O+1)*8-1}${G}`:nt=`>= ${l}${G} and <= ${h}${G}`,new At.ERR_OUT_OF_RANGE("value",nt,x)}Zt(y,R,O)}function Yt(x,l){if(typeof x!="number")throw new At.ERR_INVALID_ARG_TYPE(l,"number",x)}function Ot(x,l,h){throw Math.floor(x)!==x?(Yt(x,h),new At.ERR_OUT_OF_RANGE(h||"offset","an integer",x)):l<0?new At.ERR_BUFFER_OUT_OF_BOUNDS:new At.ERR_OUT_OF_RANGE(h||"offset",`>= ${h?1:0} and <= ${l}`,x)}const Gt=/[^+/0-9A-Za-z-_]/g;function Mt(x){if(x=x.split("=")[0],x=x.trim().replace(Gt,""),x.length<2)return"";for(;x.length%4!==0;)x=x+"=";return x}function Jt(x,l){l=l||1/0;let h;const y=x.length;let R=null;const O=[];for(let G=0;G<y;++G){if(h=x.charCodeAt(G),h>55295&&h<57344){if(!R){if(h>56319){(l-=3)>-1&&O.push(239,191,189);continue}else if(G+1===y){(l-=3)>-1&&O.push(239,191,189);continue}R=h;continue}if(h<56320){(l-=3)>-1&&O.push(239,191,189),R=h;continue}h=(R-55296<<10|h-56320)+65536}else R&&(l-=3)>-1&&O.push(239,191,189);if(R=null,h<128){if((l-=1)<0)break;O.push(h)}else if(h<2048){if((l-=2)<0)break;O.push(h>>6|192,h&63|128)}else if(h<65536){if((l-=3)<0)break;O.push(h>>12|224,h>>6&63|128,h&63|128)}else if(h<1114112){if((l-=4)<0)break;O.push(h>>18|240,h>>12&63|128,h>>6&63|128,h&63|128)}else throw new Error("Invalid code point")}return O}function Lt(x){const l=[];for(let h=0;h<x.length;++h)l.push(x.charCodeAt(h)&255);return l}function It(x,l){let h,y,R;const O=[];for(let G=0;G<x.length&&!((l-=2)<0);++G)h=x.charCodeAt(G),y=h>>8,R=h%256,O.push(R),O.push(y);return O}function L(x){return t.toByteArray(Mt(x))}function S(x,l,h,y){let R;for(R=0;R<y&&!(R+h>=l.length||R>=x.length);++R)l[R+h]=x[R];return R}function V(x,l){return x instanceof l||x!=null&&x.constructor!=null&&x.constructor.name!=null&&x.constructor.name===l.name}function it(x){return x!==x}const rt=function(){const x="0123456789abcdef",l=new Array(256);for(let h=0;h<16;++h){const y=h*16;for(let R=0;R<16;++R)l[y+R]=x[h]+x[R]}return l}();function Q(x){return typeof BigInt>"u"?Et:x}function Et(){throw new Error("BigInt not supported")}})(bc);window.Buffer=bc.Buffer;/**
 * @license
 * Copyright 2010-2024 Three.js Authors
 * SPDX-License-Identifier: MIT
 */const ba="167",Zl=0,Za=1,Jl=2,Rc=1,Ql=2,en=3,An=0,Se=1,ge=2,Tn=0,gi=1,Ja=2,Qa=3,to=4,tu=5,Hn=100,eu=101,nu=102,iu=103,ru=104,su=200,au=201,ou=202,cu=203,ks=204,Gs=205,lu=206,uu=207,hu=208,fu=209,du=210,pu=211,mu=212,_u=213,gu=214,xu=0,vu=1,yu=2,Nr=3,Mu=4,Su=5,Eu=6,Tu=7,Pc=0,wu=1,Au=2,wn=0,bu=1,Cu=2,Ru=3,Pu=4,Lu=5,Iu=6,Du=7,Lc=300,yi=301,Mi=302,Vs=303,Ws=304,Xr=306,qs=1e3,Gn=1001,Xs=1002,De=1003,Uu=1004,Ji=1005,Be=1006,as=1007,Vn=1008,ln=1009,Ic=1010,Dc=1011,Gi=1012,Ca=1013,Wn=1014,nn=1015,qi=1016,Ra=1017,Pa=1018,Si=1020,Uc=35902,Nc=1021,Fc=1022,He=1023,Oc=1024,Bc=1025,xi=1026,Ei=1027,zc=1028,La=1029,Hc=1030,Ia=1031,Da=1033,Cr=33776,Rr=33777,Pr=33778,Lr=33779,$s=35840,Ys=35841,Ks=35842,js=35843,Zs=36196,Js=37492,Qs=37496,ta=37808,ea=37809,na=37810,ia=37811,ra=37812,sa=37813,aa=37814,oa=37815,ca=37816,la=37817,ua=37818,ha=37819,fa=37820,da=37821,Ir=36492,pa=36494,ma=36495,kc=36283,_a=36284,ga=36285,xa=36286,Nu=3200,Fu=3201,Ua=0,Ou=1,Sn="",We="srgb",Rn="srgb-linear",Na="display-p3",$r="display-p3-linear",Fr="linear",Kt="srgb",Or="rec709",Br="p3",Qn=7680,eo=519,Bu=512,zu=513,Hu=514,Gc=515,ku=516,Gu=517,Vu=518,Wu=519,no=35044,io="300 es",rn=2e3,zr=2001;class Ai{addEventListener(t,e){this._listeners===void 0&&(this._listeners={});const i=this._listeners;i[t]===void 0&&(i[t]=[]),i[t].indexOf(e)===-1&&i[t].push(e)}hasEventListener(t,e){if(this._listeners===void 0)return!1;const i=this._listeners;return i[t]!==void 0&&i[t].indexOf(e)!==-1}removeEventListener(t,e){if(this._listeners===void 0)return;const r=this._listeners[t];if(r!==void 0){const s=r.indexOf(e);s!==-1&&r.splice(s,1)}}dispatchEvent(t){if(this._listeners===void 0)return;const i=this._listeners[t.type];if(i!==void 0){t.target=this;const r=i.slice(0);for(let s=0,o=r.length;s<o;s++)r[s].call(this,t);t.target=null}}}const de=["00","01","02","03","04","05","06","07","08","09","0a","0b","0c","0d","0e","0f","10","11","12","13","14","15","16","17","18","19","1a","1b","1c","1d","1e","1f","20","21","22","23","24","25","26","27","28","29","2a","2b","2c","2d","2e","2f","30","31","32","33","34","35","36","37","38","39","3a","3b","3c","3d","3e","3f","40","41","42","43","44","45","46","47","48","49","4a","4b","4c","4d","4e","4f","50","51","52","53","54","55","56","57","58","59","5a","5b","5c","5d","5e","5f","60","61","62","63","64","65","66","67","68","69","6a","6b","6c","6d","6e","6f","70","71","72","73","74","75","76","77","78","79","7a","7b","7c","7d","7e","7f","80","81","82","83","84","85","86","87","88","89","8a","8b","8c","8d","8e","8f","90","91","92","93","94","95","96","97","98","99","9a","9b","9c","9d","9e","9f","a0","a1","a2","a3","a4","a5","a6","a7","a8","a9","aa","ab","ac","ad","ae","af","b0","b1","b2","b3","b4","b5","b6","b7","b8","b9","ba","bb","bc","bd","be","bf","c0","c1","c2","c3","c4","c5","c6","c7","c8","c9","ca","cb","cc","cd","ce","cf","d0","d1","d2","d3","d4","d5","d6","d7","d8","d9","da","db","dc","dd","de","df","e0","e1","e2","e3","e4","e5","e6","e7","e8","e9","ea","eb","ec","ed","ee","ef","f0","f1","f2","f3","f4","f5","f6","f7","f8","f9","fa","fb","fc","fd","fe","ff"],os=Math.PI/180,va=180/Math.PI;function Xi(){const n=Math.random()*4294967295|0,t=Math.random()*4294967295|0,e=Math.random()*4294967295|0,i=Math.random()*4294967295|0;return(de[n&255]+de[n>>8&255]+de[n>>16&255]+de[n>>24&255]+"-"+de[t&255]+de[t>>8&255]+"-"+de[t>>16&15|64]+de[t>>24&255]+"-"+de[e&63|128]+de[e>>8&255]+"-"+de[e>>16&255]+de[e>>24&255]+de[i&255]+de[i>>8&255]+de[i>>16&255]+de[i>>24&255]).toLowerCase()}function Me(n,t,e){return Math.max(t,Math.min(e,n))}function qu(n,t){return(n%t+t)%t}function cs(n,t,e){return(1-e)*n+e*t}function Pi(n,t){switch(t.constructor){case Float32Array:return n;case Uint32Array:return n/4294967295;case Uint16Array:return n/65535;case Uint8Array:return n/255;case Int32Array:return Math.max(n/2147483647,-1);case Int16Array:return Math.max(n/32767,-1);case Int8Array:return Math.max(n/127,-1);default:throw new Error("Invalid component type.")}}function ye(n,t){switch(t.constructor){case Float32Array:return n;case Uint32Array:return Math.round(n*4294967295);case Uint16Array:return Math.round(n*65535);case Uint8Array:return Math.round(n*255);case Int32Array:return Math.round(n*2147483647);case Int16Array:return Math.round(n*32767);case Int8Array:return Math.round(n*127);default:throw new Error("Invalid component type.")}}class Bt{constructor(t=0,e=0){Bt.prototype.isVector2=!0,this.x=t,this.y=e}get width(){return this.x}set width(t){this.x=t}get height(){return this.y}set height(t){this.y=t}set(t,e){return this.x=t,this.y=e,this}setScalar(t){return this.x=t,this.y=t,this}setX(t){return this.x=t,this}setY(t){return this.y=t,this}setComponent(t,e){switch(t){case 0:this.x=e;break;case 1:this.y=e;break;default:throw new Error("index is out of range: "+t)}return this}getComponent(t){switch(t){case 0:return this.x;case 1:return this.y;default:throw new Error("index is out of range: "+t)}}clone(){return new this.constructor(this.x,this.y)}copy(t){return this.x=t.x,this.y=t.y,this}add(t){return this.x+=t.x,this.y+=t.y,this}addScalar(t){return this.x+=t,this.y+=t,this}addVectors(t,e){return this.x=t.x+e.x,this.y=t.y+e.y,this}addScaledVector(t,e){return this.x+=t.x*e,this.y+=t.y*e,this}sub(t){return this.x-=t.x,this.y-=t.y,this}subScalar(t){return this.x-=t,this.y-=t,this}subVectors(t,e){return this.x=t.x-e.x,this.y=t.y-e.y,this}multiply(t){return this.x*=t.x,this.y*=t.y,this}multiplyScalar(t){return this.x*=t,this.y*=t,this}divide(t){return this.x/=t.x,this.y/=t.y,this}divideScalar(t){return this.multiplyScalar(1/t)}applyMatrix3(t){const e=this.x,i=this.y,r=t.elements;return this.x=r[0]*e+r[3]*i+r[6],this.y=r[1]*e+r[4]*i+r[7],this}min(t){return this.x=Math.min(this.x,t.x),this.y=Math.min(this.y,t.y),this}max(t){return this.x=Math.max(this.x,t.x),this.y=Math.max(this.y,t.y),this}clamp(t,e){return this.x=Math.max(t.x,Math.min(e.x,this.x)),this.y=Math.max(t.y,Math.min(e.y,this.y)),this}clampScalar(t,e){return this.x=Math.max(t,Math.min(e,this.x)),this.y=Math.max(t,Math.min(e,this.y)),this}clampLength(t,e){const i=this.length();return this.divideScalar(i||1).multiplyScalar(Math.max(t,Math.min(e,i)))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this}negate(){return this.x=-this.x,this.y=-this.y,this}dot(t){return this.x*t.x+this.y*t.y}cross(t){return this.x*t.y-this.y*t.x}lengthSq(){return this.x*this.x+this.y*this.y}length(){return Math.sqrt(this.x*this.x+this.y*this.y)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)}normalize(){return this.divideScalar(this.length()||1)}angle(){return Math.atan2(-this.y,-this.x)+Math.PI}angleTo(t){const e=Math.sqrt(this.lengthSq()*t.lengthSq());if(e===0)return Math.PI/2;const i=this.dot(t)/e;return Math.acos(Me(i,-1,1))}distanceTo(t){return Math.sqrt(this.distanceToSquared(t))}distanceToSquared(t){const e=this.x-t.x,i=this.y-t.y;return e*e+i*i}manhattanDistanceTo(t){return Math.abs(this.x-t.x)+Math.abs(this.y-t.y)}setLength(t){return this.normalize().multiplyScalar(t)}lerp(t,e){return this.x+=(t.x-this.x)*e,this.y+=(t.y-this.y)*e,this}lerpVectors(t,e,i){return this.x=t.x+(e.x-t.x)*i,this.y=t.y+(e.y-t.y)*i,this}equals(t){return t.x===this.x&&t.y===this.y}fromArray(t,e=0){return this.x=t[e],this.y=t[e+1],this}toArray(t=[],e=0){return t[e]=this.x,t[e+1]=this.y,t}fromBufferAttribute(t,e){return this.x=t.getX(e),this.y=t.getY(e),this}rotateAround(t,e){const i=Math.cos(e),r=Math.sin(e),s=this.x-t.x,o=this.y-t.y;return this.x=s*i-o*r+t.x,this.y=s*r+o*i+t.y,this}random(){return this.x=Math.random(),this.y=Math.random(),this}*[Symbol.iterator](){yield this.x,yield this.y}}class Nt{constructor(t,e,i,r,s,o,a,c,u){Nt.prototype.isMatrix3=!0,this.elements=[1,0,0,0,1,0,0,0,1],t!==void 0&&this.set(t,e,i,r,s,o,a,c,u)}set(t,e,i,r,s,o,a,c,u){const f=this.elements;return f[0]=t,f[1]=r,f[2]=a,f[3]=e,f[4]=s,f[5]=c,f[6]=i,f[7]=o,f[8]=u,this}identity(){return this.set(1,0,0,0,1,0,0,0,1),this}copy(t){const e=this.elements,i=t.elements;return e[0]=i[0],e[1]=i[1],e[2]=i[2],e[3]=i[3],e[4]=i[4],e[5]=i[5],e[6]=i[6],e[7]=i[7],e[8]=i[8],this}extractBasis(t,e,i){return t.setFromMatrix3Column(this,0),e.setFromMatrix3Column(this,1),i.setFromMatrix3Column(this,2),this}setFromMatrix4(t){const e=t.elements;return this.set(e[0],e[4],e[8],e[1],e[5],e[9],e[2],e[6],e[10]),this}multiply(t){return this.multiplyMatrices(this,t)}premultiply(t){return this.multiplyMatrices(t,this)}multiplyMatrices(t,e){const i=t.elements,r=e.elements,s=this.elements,o=i[0],a=i[3],c=i[6],u=i[1],f=i[4],p=i[7],m=i[2],g=i[5],v=i[8],M=r[0],_=r[3],d=r[6],C=r[1],w=r[4],b=r[7],z=r[2],P=r[5],I=r[8];return s[0]=o*M+a*C+c*z,s[3]=o*_+a*w+c*P,s[6]=o*d+a*b+c*I,s[1]=u*M+f*C+p*z,s[4]=u*_+f*w+p*P,s[7]=u*d+f*b+p*I,s[2]=m*M+g*C+v*z,s[5]=m*_+g*w+v*P,s[8]=m*d+g*b+v*I,this}multiplyScalar(t){const e=this.elements;return e[0]*=t,e[3]*=t,e[6]*=t,e[1]*=t,e[4]*=t,e[7]*=t,e[2]*=t,e[5]*=t,e[8]*=t,this}determinant(){const t=this.elements,e=t[0],i=t[1],r=t[2],s=t[3],o=t[4],a=t[5],c=t[6],u=t[7],f=t[8];return e*o*f-e*a*u-i*s*f+i*a*c+r*s*u-r*o*c}invert(){const t=this.elements,e=t[0],i=t[1],r=t[2],s=t[3],o=t[4],a=t[5],c=t[6],u=t[7],f=t[8],p=f*o-a*u,m=a*c-f*s,g=u*s-o*c,v=e*p+i*m+r*g;if(v===0)return this.set(0,0,0,0,0,0,0,0,0);const M=1/v;return t[0]=p*M,t[1]=(r*u-f*i)*M,t[2]=(a*i-r*o)*M,t[3]=m*M,t[4]=(f*e-r*c)*M,t[5]=(r*s-a*e)*M,t[6]=g*M,t[7]=(i*c-u*e)*M,t[8]=(o*e-i*s)*M,this}transpose(){let t;const e=this.elements;return t=e[1],e[1]=e[3],e[3]=t,t=e[2],e[2]=e[6],e[6]=t,t=e[5],e[5]=e[7],e[7]=t,this}getNormalMatrix(t){return this.setFromMatrix4(t).invert().transpose()}transposeIntoArray(t){const e=this.elements;return t[0]=e[0],t[1]=e[3],t[2]=e[6],t[3]=e[1],t[4]=e[4],t[5]=e[7],t[6]=e[2],t[7]=e[5],t[8]=e[8],this}setUvTransform(t,e,i,r,s,o,a){const c=Math.cos(s),u=Math.sin(s);return this.set(i*c,i*u,-i*(c*o+u*a)+o+t,-r*u,r*c,-r*(-u*o+c*a)+a+e,0,0,1),this}scale(t,e){return this.premultiply(ls.makeScale(t,e)),this}rotate(t){return this.premultiply(ls.makeRotation(-t)),this}translate(t,e){return this.premultiply(ls.makeTranslation(t,e)),this}makeTranslation(t,e){return t.isVector2?this.set(1,0,t.x,0,1,t.y,0,0,1):this.set(1,0,t,0,1,e,0,0,1),this}makeRotation(t){const e=Math.cos(t),i=Math.sin(t);return this.set(e,-i,0,i,e,0,0,0,1),this}makeScale(t,e){return this.set(t,0,0,0,e,0,0,0,1),this}equals(t){const e=this.elements,i=t.elements;for(let r=0;r<9;r++)if(e[r]!==i[r])return!1;return!0}fromArray(t,e=0){for(let i=0;i<9;i++)this.elements[i]=t[i+e];return this}toArray(t=[],e=0){const i=this.elements;return t[e]=i[0],t[e+1]=i[1],t[e+2]=i[2],t[e+3]=i[3],t[e+4]=i[4],t[e+5]=i[5],t[e+6]=i[6],t[e+7]=i[7],t[e+8]=i[8],t}clone(){return new this.constructor().fromArray(this.elements)}}const ls=new Nt;function Vc(n){for(let t=n.length-1;t>=0;--t)if(n[t]>=65535)return!0;return!1}function Hr(n){return document.createElementNS("http://www.w3.org/1999/xhtml",n)}function Xu(){const n=Hr("canvas");return n.style.display="block",n}const ro={};function Hi(n){n in ro||(ro[n]=!0,console.warn(n))}function $u(n,t,e){return new Promise(function(i,r){function s(){switch(n.clientWaitSync(t,n.SYNC_FLUSH_COMMANDS_BIT,0)){case n.WAIT_FAILED:r();break;case n.TIMEOUT_EXPIRED:setTimeout(s,e);break;default:i()}}setTimeout(s,e)})}const so=new Nt().set(.8224621,.177538,0,.0331941,.9668058,0,.0170827,.0723974,.9105199),ao=new Nt().set(1.2249401,-.2249404,0,-.0420569,1.0420571,0,-.0196376,-.0786361,1.0982735),Li={[Rn]:{transfer:Fr,primaries:Or,luminanceCoefficients:[.2126,.7152,.0722],toReference:n=>n,fromReference:n=>n},[We]:{transfer:Kt,primaries:Or,luminanceCoefficients:[.2126,.7152,.0722],toReference:n=>n.convertSRGBToLinear(),fromReference:n=>n.convertLinearToSRGB()},[$r]:{transfer:Fr,primaries:Br,luminanceCoefficients:[.2289,.6917,.0793],toReference:n=>n.applyMatrix3(ao),fromReference:n=>n.applyMatrix3(so)},[Na]:{transfer:Kt,primaries:Br,luminanceCoefficients:[.2289,.6917,.0793],toReference:n=>n.convertSRGBToLinear().applyMatrix3(ao),fromReference:n=>n.applyMatrix3(so).convertLinearToSRGB()}},Yu=new Set([Rn,$r]),$t={enabled:!0,_workingColorSpace:Rn,get workingColorSpace(){return this._workingColorSpace},set workingColorSpace(n){if(!Yu.has(n))throw new Error(`Unsupported working color space, "${n}".`);this._workingColorSpace=n},convert:function(n,t,e){if(this.enabled===!1||t===e||!t||!e)return n;const i=Li[t].toReference,r=Li[e].fromReference;return r(i(n))},fromWorkingColorSpace:function(n,t){return this.convert(n,this._workingColorSpace,t)},toWorkingColorSpace:function(n,t){return this.convert(n,t,this._workingColorSpace)},getPrimaries:function(n){return Li[n].primaries},getTransfer:function(n){return n===Sn?Fr:Li[n].transfer},getLuminanceCoefficients:function(n,t=this._workingColorSpace){return n.fromArray(Li[t].luminanceCoefficients)}};function vi(n){return n<.04045?n*.0773993808:Math.pow(n*.9478672986+.0521327014,2.4)}function us(n){return n<.0031308?n*12.92:1.055*Math.pow(n,.41666)-.055}let ti;class Ku{static getDataURL(t){if(/^data:/i.test(t.src)||typeof HTMLCanvasElement>"u")return t.src;let e;if(t instanceof HTMLCanvasElement)e=t;else{ti===void 0&&(ti=Hr("canvas")),ti.width=t.width,ti.height=t.height;const i=ti.getContext("2d");t instanceof ImageData?i.putImageData(t,0,0):i.drawImage(t,0,0,t.width,t.height),e=ti}return e.width>2048||e.height>2048?(console.warn("THREE.ImageUtils.getDataURL: Image converted to jpg for performance reasons",t),e.toDataURL("image/jpeg",.6)):e.toDataURL("image/png")}static sRGBToLinear(t){if(typeof HTMLImageElement<"u"&&t instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&t instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&t instanceof ImageBitmap){const e=Hr("canvas");e.width=t.width,e.height=t.height;const i=e.getContext("2d");i.drawImage(t,0,0,t.width,t.height);const r=i.getImageData(0,0,t.width,t.height),s=r.data;for(let o=0;o<s.length;o++)s[o]=vi(s[o]/255)*255;return i.putImageData(r,0,0),e}else if(t.data){const e=t.data.slice(0);for(let i=0;i<e.length;i++)e instanceof Uint8Array||e instanceof Uint8ClampedArray?e[i]=Math.floor(vi(e[i]/255)*255):e[i]=vi(e[i]);return{data:e,width:t.width,height:t.height}}else return console.warn("THREE.ImageUtils.sRGBToLinear(): Unsupported image type. No color space conversion applied."),t}}let ju=0;class Wc{constructor(t=null){this.isSource=!0,Object.defineProperty(this,"id",{value:ju++}),this.uuid=Xi(),this.data=t,this.dataReady=!0,this.version=0}set needsUpdate(t){t===!0&&this.version++}toJSON(t){const e=t===void 0||typeof t=="string";if(!e&&t.images[this.uuid]!==void 0)return t.images[this.uuid];const i={uuid:this.uuid,url:""},r=this.data;if(r!==null){let s;if(Array.isArray(r)){s=[];for(let o=0,a=r.length;o<a;o++)r[o].isDataTexture?s.push(hs(r[o].image)):s.push(hs(r[o]))}else s=hs(r);i.url=s}return e||(t.images[this.uuid]=i),i}}function hs(n){return typeof HTMLImageElement<"u"&&n instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&n instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&n instanceof ImageBitmap?Ku.getDataURL(n):n.data?{data:Array.from(n.data),width:n.width,height:n.height,type:n.data.constructor.name}:(console.warn("THREE.Texture: Unable to serialize Texture."),{})}let Zu=0;class Ee extends Ai{constructor(t=Ee.DEFAULT_IMAGE,e=Ee.DEFAULT_MAPPING,i=Gn,r=Gn,s=Be,o=Vn,a=He,c=ln,u=Ee.DEFAULT_ANISOTROPY,f=Sn){super(),this.isTexture=!0,Object.defineProperty(this,"id",{value:Zu++}),this.uuid=Xi(),this.name="",this.source=new Wc(t),this.mipmaps=[],this.mapping=e,this.channel=0,this.wrapS=i,this.wrapT=r,this.magFilter=s,this.minFilter=o,this.anisotropy=u,this.format=a,this.internalFormat=null,this.type=c,this.offset=new Bt(0,0),this.repeat=new Bt(1,1),this.center=new Bt(0,0),this.rotation=0,this.matrixAutoUpdate=!0,this.matrix=new Nt,this.generateMipmaps=!0,this.premultiplyAlpha=!1,this.flipY=!0,this.unpackAlignment=4,this.colorSpace=f,this.userData={},this.version=0,this.onUpdate=null,this.isRenderTargetTexture=!1,this.pmremVersion=0}get image(){return this.source.data}set image(t=null){this.source.data=t}updateMatrix(){this.matrix.setUvTransform(this.offset.x,this.offset.y,this.repeat.x,this.repeat.y,this.rotation,this.center.x,this.center.y)}clone(){return new this.constructor().copy(this)}copy(t){return this.name=t.name,this.source=t.source,this.mipmaps=t.mipmaps.slice(0),this.mapping=t.mapping,this.channel=t.channel,this.wrapS=t.wrapS,this.wrapT=t.wrapT,this.magFilter=t.magFilter,this.minFilter=t.minFilter,this.anisotropy=t.anisotropy,this.format=t.format,this.internalFormat=t.internalFormat,this.type=t.type,this.offset.copy(t.offset),this.repeat.copy(t.repeat),this.center.copy(t.center),this.rotation=t.rotation,this.matrixAutoUpdate=t.matrixAutoUpdate,this.matrix.copy(t.matrix),this.generateMipmaps=t.generateMipmaps,this.premultiplyAlpha=t.premultiplyAlpha,this.flipY=t.flipY,this.unpackAlignment=t.unpackAlignment,this.colorSpace=t.colorSpace,this.userData=JSON.parse(JSON.stringify(t.userData)),this.needsUpdate=!0,this}toJSON(t){const e=t===void 0||typeof t=="string";if(!e&&t.textures[this.uuid]!==void 0)return t.textures[this.uuid];const i={metadata:{version:4.6,type:"Texture",generator:"Texture.toJSON"},uuid:this.uuid,name:this.name,image:this.source.toJSON(t).uuid,mapping:this.mapping,channel:this.channel,repeat:[this.repeat.x,this.repeat.y],offset:[this.offset.x,this.offset.y],center:[this.center.x,this.center.y],rotation:this.rotation,wrap:[this.wrapS,this.wrapT],format:this.format,internalFormat:this.internalFormat,type:this.type,colorSpace:this.colorSpace,minFilter:this.minFilter,magFilter:this.magFilter,anisotropy:this.anisotropy,flipY:this.flipY,generateMipmaps:this.generateMipmaps,premultiplyAlpha:this.premultiplyAlpha,unpackAlignment:this.unpackAlignment};return Object.keys(this.userData).length>0&&(i.userData=this.userData),e||(t.textures[this.uuid]=i),i}dispose(){this.dispatchEvent({type:"dispose"})}transformUv(t){if(this.mapping!==Lc)return t;if(t.applyMatrix3(this.matrix),t.x<0||t.x>1)switch(this.wrapS){case qs:t.x=t.x-Math.floor(t.x);break;case Gn:t.x=t.x<0?0:1;break;case Xs:Math.abs(Math.floor(t.x)%2)===1?t.x=Math.ceil(t.x)-t.x:t.x=t.x-Math.floor(t.x);break}if(t.y<0||t.y>1)switch(this.wrapT){case qs:t.y=t.y-Math.floor(t.y);break;case Gn:t.y=t.y<0?0:1;break;case Xs:Math.abs(Math.floor(t.y)%2)===1?t.y=Math.ceil(t.y)-t.y:t.y=t.y-Math.floor(t.y);break}return this.flipY&&(t.y=1-t.y),t}set needsUpdate(t){t===!0&&(this.version++,this.source.needsUpdate=!0)}set needsPMREMUpdate(t){t===!0&&this.pmremVersion++}}Ee.DEFAULT_IMAGE=null;Ee.DEFAULT_MAPPING=Lc;Ee.DEFAULT_ANISOTROPY=1;class se{constructor(t=0,e=0,i=0,r=1){se.prototype.isVector4=!0,this.x=t,this.y=e,this.z=i,this.w=r}get width(){return this.z}set width(t){this.z=t}get height(){return this.w}set height(t){this.w=t}set(t,e,i,r){return this.x=t,this.y=e,this.z=i,this.w=r,this}setScalar(t){return this.x=t,this.y=t,this.z=t,this.w=t,this}setX(t){return this.x=t,this}setY(t){return this.y=t,this}setZ(t){return this.z=t,this}setW(t){return this.w=t,this}setComponent(t,e){switch(t){case 0:this.x=e;break;case 1:this.y=e;break;case 2:this.z=e;break;case 3:this.w=e;break;default:throw new Error("index is out of range: "+t)}return this}getComponent(t){switch(t){case 0:return this.x;case 1:return this.y;case 2:return this.z;case 3:return this.w;default:throw new Error("index is out of range: "+t)}}clone(){return new this.constructor(this.x,this.y,this.z,this.w)}copy(t){return this.x=t.x,this.y=t.y,this.z=t.z,this.w=t.w!==void 0?t.w:1,this}add(t){return this.x+=t.x,this.y+=t.y,this.z+=t.z,this.w+=t.w,this}addScalar(t){return this.x+=t,this.y+=t,this.z+=t,this.w+=t,this}addVectors(t,e){return this.x=t.x+e.x,this.y=t.y+e.y,this.z=t.z+e.z,this.w=t.w+e.w,this}addScaledVector(t,e){return this.x+=t.x*e,this.y+=t.y*e,this.z+=t.z*e,this.w+=t.w*e,this}sub(t){return this.x-=t.x,this.y-=t.y,this.z-=t.z,this.w-=t.w,this}subScalar(t){return this.x-=t,this.y-=t,this.z-=t,this.w-=t,this}subVectors(t,e){return this.x=t.x-e.x,this.y=t.y-e.y,this.z=t.z-e.z,this.w=t.w-e.w,this}multiply(t){return this.x*=t.x,this.y*=t.y,this.z*=t.z,this.w*=t.w,this}multiplyScalar(t){return this.x*=t,this.y*=t,this.z*=t,this.w*=t,this}applyMatrix4(t){const e=this.x,i=this.y,r=this.z,s=this.w,o=t.elements;return this.x=o[0]*e+o[4]*i+o[8]*r+o[12]*s,this.y=o[1]*e+o[5]*i+o[9]*r+o[13]*s,this.z=o[2]*e+o[6]*i+o[10]*r+o[14]*s,this.w=o[3]*e+o[7]*i+o[11]*r+o[15]*s,this}divideScalar(t){return this.multiplyScalar(1/t)}setAxisAngleFromQuaternion(t){this.w=2*Math.acos(t.w);const e=Math.sqrt(1-t.w*t.w);return e<1e-4?(this.x=1,this.y=0,this.z=0):(this.x=t.x/e,this.y=t.y/e,this.z=t.z/e),this}setAxisAngleFromRotationMatrix(t){let e,i,r,s;const c=t.elements,u=c[0],f=c[4],p=c[8],m=c[1],g=c[5],v=c[9],M=c[2],_=c[6],d=c[10];if(Math.abs(f-m)<.01&&Math.abs(p-M)<.01&&Math.abs(v-_)<.01){if(Math.abs(f+m)<.1&&Math.abs(p+M)<.1&&Math.abs(v+_)<.1&&Math.abs(u+g+d-3)<.1)return this.set(1,0,0,0),this;e=Math.PI;const w=(u+1)/2,b=(g+1)/2,z=(d+1)/2,P=(f+m)/4,I=(p+M)/4,F=(v+_)/4;return w>b&&w>z?w<.01?(i=0,r=.707106781,s=.707106781):(i=Math.sqrt(w),r=P/i,s=I/i):b>z?b<.01?(i=.707106781,r=0,s=.707106781):(r=Math.sqrt(b),i=P/r,s=F/r):z<.01?(i=.707106781,r=.707106781,s=0):(s=Math.sqrt(z),i=I/s,r=F/s),this.set(i,r,s,e),this}let C=Math.sqrt((_-v)*(_-v)+(p-M)*(p-M)+(m-f)*(m-f));return Math.abs(C)<.001&&(C=1),this.x=(_-v)/C,this.y=(p-M)/C,this.z=(m-f)/C,this.w=Math.acos((u+g+d-1)/2),this}setFromMatrixPosition(t){const e=t.elements;return this.x=e[12],this.y=e[13],this.z=e[14],this.w=e[15],this}min(t){return this.x=Math.min(this.x,t.x),this.y=Math.min(this.y,t.y),this.z=Math.min(this.z,t.z),this.w=Math.min(this.w,t.w),this}max(t){return this.x=Math.max(this.x,t.x),this.y=Math.max(this.y,t.y),this.z=Math.max(this.z,t.z),this.w=Math.max(this.w,t.w),this}clamp(t,e){return this.x=Math.max(t.x,Math.min(e.x,this.x)),this.y=Math.max(t.y,Math.min(e.y,this.y)),this.z=Math.max(t.z,Math.min(e.z,this.z)),this.w=Math.max(t.w,Math.min(e.w,this.w)),this}clampScalar(t,e){return this.x=Math.max(t,Math.min(e,this.x)),this.y=Math.max(t,Math.min(e,this.y)),this.z=Math.max(t,Math.min(e,this.z)),this.w=Math.max(t,Math.min(e,this.w)),this}clampLength(t,e){const i=this.length();return this.divideScalar(i||1).multiplyScalar(Math.max(t,Math.min(e,i)))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this.z=Math.floor(this.z),this.w=Math.floor(this.w),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this.z=Math.ceil(this.z),this.w=Math.ceil(this.w),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this.z=Math.round(this.z),this.w=Math.round(this.w),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this.z=Math.trunc(this.z),this.w=Math.trunc(this.w),this}negate(){return this.x=-this.x,this.y=-this.y,this.z=-this.z,this.w=-this.w,this}dot(t){return this.x*t.x+this.y*t.y+this.z*t.z+this.w*t.w}lengthSq(){return this.x*this.x+this.y*this.y+this.z*this.z+this.w*this.w}length(){return Math.sqrt(this.x*this.x+this.y*this.y+this.z*this.z+this.w*this.w)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)+Math.abs(this.z)+Math.abs(this.w)}normalize(){return this.divideScalar(this.length()||1)}setLength(t){return this.normalize().multiplyScalar(t)}lerp(t,e){return this.x+=(t.x-this.x)*e,this.y+=(t.y-this.y)*e,this.z+=(t.z-this.z)*e,this.w+=(t.w-this.w)*e,this}lerpVectors(t,e,i){return this.x=t.x+(e.x-t.x)*i,this.y=t.y+(e.y-t.y)*i,this.z=t.z+(e.z-t.z)*i,this.w=t.w+(e.w-t.w)*i,this}equals(t){return t.x===this.x&&t.y===this.y&&t.z===this.z&&t.w===this.w}fromArray(t,e=0){return this.x=t[e],this.y=t[e+1],this.z=t[e+2],this.w=t[e+3],this}toArray(t=[],e=0){return t[e]=this.x,t[e+1]=this.y,t[e+2]=this.z,t[e+3]=this.w,t}fromBufferAttribute(t,e){return this.x=t.getX(e),this.y=t.getY(e),this.z=t.getZ(e),this.w=t.getW(e),this}random(){return this.x=Math.random(),this.y=Math.random(),this.z=Math.random(),this.w=Math.random(),this}*[Symbol.iterator](){yield this.x,yield this.y,yield this.z,yield this.w}}class Ju extends Ai{constructor(t=1,e=1,i={}){super(),this.isRenderTarget=!0,this.width=t,this.height=e,this.depth=1,this.scissor=new se(0,0,t,e),this.scissorTest=!1,this.viewport=new se(0,0,t,e);const r={width:t,height:e,depth:1};i=Object.assign({generateMipmaps:!1,internalFormat:null,minFilter:Be,depthBuffer:!0,stencilBuffer:!1,resolveDepthBuffer:!0,resolveStencilBuffer:!0,depthTexture:null,samples:0,count:1},i);const s=new Ee(r,i.mapping,i.wrapS,i.wrapT,i.magFilter,i.minFilter,i.format,i.type,i.anisotropy,i.colorSpace);s.flipY=!1,s.generateMipmaps=i.generateMipmaps,s.internalFormat=i.internalFormat,this.textures=[];const o=i.count;for(let a=0;a<o;a++)this.textures[a]=s.clone(),this.textures[a].isRenderTargetTexture=!0;this.depthBuffer=i.depthBuffer,this.stencilBuffer=i.stencilBuffer,this.resolveDepthBuffer=i.resolveDepthBuffer,this.resolveStencilBuffer=i.resolveStencilBuffer,this.depthTexture=i.depthTexture,this.samples=i.samples}get texture(){return this.textures[0]}set texture(t){this.textures[0]=t}setSize(t,e,i=1){if(this.width!==t||this.height!==e||this.depth!==i){this.width=t,this.height=e,this.depth=i;for(let r=0,s=this.textures.length;r<s;r++)this.textures[r].image.width=t,this.textures[r].image.height=e,this.textures[r].image.depth=i;this.dispose()}this.viewport.set(0,0,t,e),this.scissor.set(0,0,t,e)}clone(){return new this.constructor().copy(this)}copy(t){this.width=t.width,this.height=t.height,this.depth=t.depth,this.scissor.copy(t.scissor),this.scissorTest=t.scissorTest,this.viewport.copy(t.viewport),this.textures.length=0;for(let i=0,r=t.textures.length;i<r;i++)this.textures[i]=t.textures[i].clone(),this.textures[i].isRenderTargetTexture=!0;const e=Object.assign({},t.texture.image);return this.texture.source=new Wc(e),this.depthBuffer=t.depthBuffer,this.stencilBuffer=t.stencilBuffer,this.resolveDepthBuffer=t.resolveDepthBuffer,this.resolveStencilBuffer=t.resolveStencilBuffer,t.depthTexture!==null&&(this.depthTexture=t.depthTexture.clone()),this.samples=t.samples,this}dispose(){this.dispatchEvent({type:"dispose"})}}class qn extends Ju{constructor(t=1,e=1,i={}){super(t,e,i),this.isWebGLRenderTarget=!0}}class qc extends Ee{constructor(t=null,e=1,i=1,r=1){super(null),this.isDataArrayTexture=!0,this.image={data:t,width:e,height:i,depth:r},this.magFilter=De,this.minFilter=De,this.wrapR=Gn,this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1,this.layerUpdates=new Set}addLayerUpdate(t){this.layerUpdates.add(t)}clearLayerUpdates(){this.layerUpdates.clear()}}class Qu extends Ee{constructor(t=null,e=1,i=1,r=1){super(null),this.isData3DTexture=!0,this.image={data:t,width:e,height:i,depth:r},this.magFilter=De,this.minFilter=De,this.wrapR=Gn,this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1}}class $i{constructor(t=0,e=0,i=0,r=1){this.isQuaternion=!0,this._x=t,this._y=e,this._z=i,this._w=r}static slerpFlat(t,e,i,r,s,o,a){let c=i[r+0],u=i[r+1],f=i[r+2],p=i[r+3];const m=s[o+0],g=s[o+1],v=s[o+2],M=s[o+3];if(a===0){t[e+0]=c,t[e+1]=u,t[e+2]=f,t[e+3]=p;return}if(a===1){t[e+0]=m,t[e+1]=g,t[e+2]=v,t[e+3]=M;return}if(p!==M||c!==m||u!==g||f!==v){let _=1-a;const d=c*m+u*g+f*v+p*M,C=d>=0?1:-1,w=1-d*d;if(w>Number.EPSILON){const z=Math.sqrt(w),P=Math.atan2(z,d*C);_=Math.sin(_*P)/z,a=Math.sin(a*P)/z}const b=a*C;if(c=c*_+m*b,u=u*_+g*b,f=f*_+v*b,p=p*_+M*b,_===1-a){const z=1/Math.sqrt(c*c+u*u+f*f+p*p);c*=z,u*=z,f*=z,p*=z}}t[e]=c,t[e+1]=u,t[e+2]=f,t[e+3]=p}static multiplyQuaternionsFlat(t,e,i,r,s,o){const a=i[r],c=i[r+1],u=i[r+2],f=i[r+3],p=s[o],m=s[o+1],g=s[o+2],v=s[o+3];return t[e]=a*v+f*p+c*g-u*m,t[e+1]=c*v+f*m+u*p-a*g,t[e+2]=u*v+f*g+a*m-c*p,t[e+3]=f*v-a*p-c*m-u*g,t}get x(){return this._x}set x(t){this._x=t,this._onChangeCallback()}get y(){return this._y}set y(t){this._y=t,this._onChangeCallback()}get z(){return this._z}set z(t){this._z=t,this._onChangeCallback()}get w(){return this._w}set w(t){this._w=t,this._onChangeCallback()}set(t,e,i,r){return this._x=t,this._y=e,this._z=i,this._w=r,this._onChangeCallback(),this}clone(){return new this.constructor(this._x,this._y,this._z,this._w)}copy(t){return this._x=t.x,this._y=t.y,this._z=t.z,this._w=t.w,this._onChangeCallback(),this}setFromEuler(t,e=!0){const i=t._x,r=t._y,s=t._z,o=t._order,a=Math.cos,c=Math.sin,u=a(i/2),f=a(r/2),p=a(s/2),m=c(i/2),g=c(r/2),v=c(s/2);switch(o){case"XYZ":this._x=m*f*p+u*g*v,this._y=u*g*p-m*f*v,this._z=u*f*v+m*g*p,this._w=u*f*p-m*g*v;break;case"YXZ":this._x=m*f*p+u*g*v,this._y=u*g*p-m*f*v,this._z=u*f*v-m*g*p,this._w=u*f*p+m*g*v;break;case"ZXY":this._x=m*f*p-u*g*v,this._y=u*g*p+m*f*v,this._z=u*f*v+m*g*p,this._w=u*f*p-m*g*v;break;case"ZYX":this._x=m*f*p-u*g*v,this._y=u*g*p+m*f*v,this._z=u*f*v-m*g*p,this._w=u*f*p+m*g*v;break;case"YZX":this._x=m*f*p+u*g*v,this._y=u*g*p+m*f*v,this._z=u*f*v-m*g*p,this._w=u*f*p-m*g*v;break;case"XZY":this._x=m*f*p-u*g*v,this._y=u*g*p-m*f*v,this._z=u*f*v+m*g*p,this._w=u*f*p+m*g*v;break;default:console.warn("THREE.Quaternion: .setFromEuler() encountered an unknown order: "+o)}return e===!0&&this._onChangeCallback(),this}setFromAxisAngle(t,e){const i=e/2,r=Math.sin(i);return this._x=t.x*r,this._y=t.y*r,this._z=t.z*r,this._w=Math.cos(i),this._onChangeCallback(),this}setFromRotationMatrix(t){const e=t.elements,i=e[0],r=e[4],s=e[8],o=e[1],a=e[5],c=e[9],u=e[2],f=e[6],p=e[10],m=i+a+p;if(m>0){const g=.5/Math.sqrt(m+1);this._w=.25/g,this._x=(f-c)*g,this._y=(s-u)*g,this._z=(o-r)*g}else if(i>a&&i>p){const g=2*Math.sqrt(1+i-a-p);this._w=(f-c)/g,this._x=.25*g,this._y=(r+o)/g,this._z=(s+u)/g}else if(a>p){const g=2*Math.sqrt(1+a-i-p);this._w=(s-u)/g,this._x=(r+o)/g,this._y=.25*g,this._z=(c+f)/g}else{const g=2*Math.sqrt(1+p-i-a);this._w=(o-r)/g,this._x=(s+u)/g,this._y=(c+f)/g,this._z=.25*g}return this._onChangeCallback(),this}setFromUnitVectors(t,e){let i=t.dot(e)+1;return i<Number.EPSILON?(i=0,Math.abs(t.x)>Math.abs(t.z)?(this._x=-t.y,this._y=t.x,this._z=0,this._w=i):(this._x=0,this._y=-t.z,this._z=t.y,this._w=i)):(this._x=t.y*e.z-t.z*e.y,this._y=t.z*e.x-t.x*e.z,this._z=t.x*e.y-t.y*e.x,this._w=i),this.normalize()}angleTo(t){return 2*Math.acos(Math.abs(Me(this.dot(t),-1,1)))}rotateTowards(t,e){const i=this.angleTo(t);if(i===0)return this;const r=Math.min(1,e/i);return this.slerp(t,r),this}identity(){return this.set(0,0,0,1)}invert(){return this.conjugate()}conjugate(){return this._x*=-1,this._y*=-1,this._z*=-1,this._onChangeCallback(),this}dot(t){return this._x*t._x+this._y*t._y+this._z*t._z+this._w*t._w}lengthSq(){return this._x*this._x+this._y*this._y+this._z*this._z+this._w*this._w}length(){return Math.sqrt(this._x*this._x+this._y*this._y+this._z*this._z+this._w*this._w)}normalize(){let t=this.length();return t===0?(this._x=0,this._y=0,this._z=0,this._w=1):(t=1/t,this._x=this._x*t,this._y=this._y*t,this._z=this._z*t,this._w=this._w*t),this._onChangeCallback(),this}multiply(t){return this.multiplyQuaternions(this,t)}premultiply(t){return this.multiplyQuaternions(t,this)}multiplyQuaternions(t,e){const i=t._x,r=t._y,s=t._z,o=t._w,a=e._x,c=e._y,u=e._z,f=e._w;return this._x=i*f+o*a+r*u-s*c,this._y=r*f+o*c+s*a-i*u,this._z=s*f+o*u+i*c-r*a,this._w=o*f-i*a-r*c-s*u,this._onChangeCallback(),this}slerp(t,e){if(e===0)return this;if(e===1)return this.copy(t);const i=this._x,r=this._y,s=this._z,o=this._w;let a=o*t._w+i*t._x+r*t._y+s*t._z;if(a<0?(this._w=-t._w,this._x=-t._x,this._y=-t._y,this._z=-t._z,a=-a):this.copy(t),a>=1)return this._w=o,this._x=i,this._y=r,this._z=s,this;const c=1-a*a;if(c<=Number.EPSILON){const g=1-e;return this._w=g*o+e*this._w,this._x=g*i+e*this._x,this._y=g*r+e*this._y,this._z=g*s+e*this._z,this.normalize(),this}const u=Math.sqrt(c),f=Math.atan2(u,a),p=Math.sin((1-e)*f)/u,m=Math.sin(e*f)/u;return this._w=o*p+this._w*m,this._x=i*p+this._x*m,this._y=r*p+this._y*m,this._z=s*p+this._z*m,this._onChangeCallback(),this}slerpQuaternions(t,e,i){return this.copy(t).slerp(e,i)}random(){const t=2*Math.PI*Math.random(),e=2*Math.PI*Math.random(),i=Math.random(),r=Math.sqrt(1-i),s=Math.sqrt(i);return this.set(r*Math.sin(t),r*Math.cos(t),s*Math.sin(e),s*Math.cos(e))}equals(t){return t._x===this._x&&t._y===this._y&&t._z===this._z&&t._w===this._w}fromArray(t,e=0){return this._x=t[e],this._y=t[e+1],this._z=t[e+2],this._w=t[e+3],this._onChangeCallback(),this}toArray(t=[],e=0){return t[e]=this._x,t[e+1]=this._y,t[e+2]=this._z,t[e+3]=this._w,t}fromBufferAttribute(t,e){return this._x=t.getX(e),this._y=t.getY(e),this._z=t.getZ(e),this._w=t.getW(e),this._onChangeCallback(),this}toJSON(){return this.toArray()}_onChange(t){return this._onChangeCallback=t,this}_onChangeCallback(){}*[Symbol.iterator](){yield this._x,yield this._y,yield this._z,yield this._w}}class k{constructor(t=0,e=0,i=0){k.prototype.isVector3=!0,this.x=t,this.y=e,this.z=i}set(t,e,i){return i===void 0&&(i=this.z),this.x=t,this.y=e,this.z=i,this}setScalar(t){return this.x=t,this.y=t,this.z=t,this}setX(t){return this.x=t,this}setY(t){return this.y=t,this}setZ(t){return this.z=t,this}setComponent(t,e){switch(t){case 0:this.x=e;break;case 1:this.y=e;break;case 2:this.z=e;break;default:throw new Error("index is out of range: "+t)}return this}getComponent(t){switch(t){case 0:return this.x;case 1:return this.y;case 2:return this.z;default:throw new Error("index is out of range: "+t)}}clone(){return new this.constructor(this.x,this.y,this.z)}copy(t){return this.x=t.x,this.y=t.y,this.z=t.z,this}add(t){return this.x+=t.x,this.y+=t.y,this.z+=t.z,this}addScalar(t){return this.x+=t,this.y+=t,this.z+=t,this}addVectors(t,e){return this.x=t.x+e.x,this.y=t.y+e.y,this.z=t.z+e.z,this}addScaledVector(t,e){return this.x+=t.x*e,this.y+=t.y*e,this.z+=t.z*e,this}sub(t){return this.x-=t.x,this.y-=t.y,this.z-=t.z,this}subScalar(t){return this.x-=t,this.y-=t,this.z-=t,this}subVectors(t,e){return this.x=t.x-e.x,this.y=t.y-e.y,this.z=t.z-e.z,this}multiply(t){return this.x*=t.x,this.y*=t.y,this.z*=t.z,this}multiplyScalar(t){return this.x*=t,this.y*=t,this.z*=t,this}multiplyVectors(t,e){return this.x=t.x*e.x,this.y=t.y*e.y,this.z=t.z*e.z,this}applyEuler(t){return this.applyQuaternion(oo.setFromEuler(t))}applyAxisAngle(t,e){return this.applyQuaternion(oo.setFromAxisAngle(t,e))}applyMatrix3(t){const e=this.x,i=this.y,r=this.z,s=t.elements;return this.x=s[0]*e+s[3]*i+s[6]*r,this.y=s[1]*e+s[4]*i+s[7]*r,this.z=s[2]*e+s[5]*i+s[8]*r,this}applyNormalMatrix(t){return this.applyMatrix3(t).normalize()}applyMatrix4(t){const e=this.x,i=this.y,r=this.z,s=t.elements,o=1/(s[3]*e+s[7]*i+s[11]*r+s[15]);return this.x=(s[0]*e+s[4]*i+s[8]*r+s[12])*o,this.y=(s[1]*e+s[5]*i+s[9]*r+s[13])*o,this.z=(s[2]*e+s[6]*i+s[10]*r+s[14])*o,this}applyQuaternion(t){const e=this.x,i=this.y,r=this.z,s=t.x,o=t.y,a=t.z,c=t.w,u=2*(o*r-a*i),f=2*(a*e-s*r),p=2*(s*i-o*e);return this.x=e+c*u+o*p-a*f,this.y=i+c*f+a*u-s*p,this.z=r+c*p+s*f-o*u,this}project(t){return this.applyMatrix4(t.matrixWorldInverse).applyMatrix4(t.projectionMatrix)}unproject(t){return this.applyMatrix4(t.projectionMatrixInverse).applyMatrix4(t.matrixWorld)}transformDirection(t){const e=this.x,i=this.y,r=this.z,s=t.elements;return this.x=s[0]*e+s[4]*i+s[8]*r,this.y=s[1]*e+s[5]*i+s[9]*r,this.z=s[2]*e+s[6]*i+s[10]*r,this.normalize()}divide(t){return this.x/=t.x,this.y/=t.y,this.z/=t.z,this}divideScalar(t){return this.multiplyScalar(1/t)}min(t){return this.x=Math.min(this.x,t.x),this.y=Math.min(this.y,t.y),this.z=Math.min(this.z,t.z),this}max(t){return this.x=Math.max(this.x,t.x),this.y=Math.max(this.y,t.y),this.z=Math.max(this.z,t.z),this}clamp(t,e){return this.x=Math.max(t.x,Math.min(e.x,this.x)),this.y=Math.max(t.y,Math.min(e.y,this.y)),this.z=Math.max(t.z,Math.min(e.z,this.z)),this}clampScalar(t,e){return this.x=Math.max(t,Math.min(e,this.x)),this.y=Math.max(t,Math.min(e,this.y)),this.z=Math.max(t,Math.min(e,this.z)),this}clampLength(t,e){const i=this.length();return this.divideScalar(i||1).multiplyScalar(Math.max(t,Math.min(e,i)))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this.z=Math.floor(this.z),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this.z=Math.ceil(this.z),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this.z=Math.round(this.z),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this.z=Math.trunc(this.z),this}negate(){return this.x=-this.x,this.y=-this.y,this.z=-this.z,this}dot(t){return this.x*t.x+this.y*t.y+this.z*t.z}lengthSq(){return this.x*this.x+this.y*this.y+this.z*this.z}length(){return Math.sqrt(this.x*this.x+this.y*this.y+this.z*this.z)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)+Math.abs(this.z)}normalize(){return this.divideScalar(this.length()||1)}setLength(t){return this.normalize().multiplyScalar(t)}lerp(t,e){return this.x+=(t.x-this.x)*e,this.y+=(t.y-this.y)*e,this.z+=(t.z-this.z)*e,this}lerpVectors(t,e,i){return this.x=t.x+(e.x-t.x)*i,this.y=t.y+(e.y-t.y)*i,this.z=t.z+(e.z-t.z)*i,this}cross(t){return this.crossVectors(this,t)}crossVectors(t,e){const i=t.x,r=t.y,s=t.z,o=e.x,a=e.y,c=e.z;return this.x=r*c-s*a,this.y=s*o-i*c,this.z=i*a-r*o,this}projectOnVector(t){const e=t.lengthSq();if(e===0)return this.set(0,0,0);const i=t.dot(this)/e;return this.copy(t).multiplyScalar(i)}projectOnPlane(t){return fs.copy(this).projectOnVector(t),this.sub(fs)}reflect(t){return this.sub(fs.copy(t).multiplyScalar(2*this.dot(t)))}angleTo(t){const e=Math.sqrt(this.lengthSq()*t.lengthSq());if(e===0)return Math.PI/2;const i=this.dot(t)/e;return Math.acos(Me(i,-1,1))}distanceTo(t){return Math.sqrt(this.distanceToSquared(t))}distanceToSquared(t){const e=this.x-t.x,i=this.y-t.y,r=this.z-t.z;return e*e+i*i+r*r}manhattanDistanceTo(t){return Math.abs(this.x-t.x)+Math.abs(this.y-t.y)+Math.abs(this.z-t.z)}setFromSpherical(t){return this.setFromSphericalCoords(t.radius,t.phi,t.theta)}setFromSphericalCoords(t,e,i){const r=Math.sin(e)*t;return this.x=r*Math.sin(i),this.y=Math.cos(e)*t,this.z=r*Math.cos(i),this}setFromCylindrical(t){return this.setFromCylindricalCoords(t.radius,t.theta,t.y)}setFromCylindricalCoords(t,e,i){return this.x=t*Math.sin(e),this.y=i,this.z=t*Math.cos(e),this}setFromMatrixPosition(t){const e=t.elements;return this.x=e[12],this.y=e[13],this.z=e[14],this}setFromMatrixScale(t){const e=this.setFromMatrixColumn(t,0).length(),i=this.setFromMatrixColumn(t,1).length(),r=this.setFromMatrixColumn(t,2).length();return this.x=e,this.y=i,this.z=r,this}setFromMatrixColumn(t,e){return this.fromArray(t.elements,e*4)}setFromMatrix3Column(t,e){return this.fromArray(t.elements,e*3)}setFromEuler(t){return this.x=t._x,this.y=t._y,this.z=t._z,this}setFromColor(t){return this.x=t.r,this.y=t.g,this.z=t.b,this}equals(t){return t.x===this.x&&t.y===this.y&&t.z===this.z}fromArray(t,e=0){return this.x=t[e],this.y=t[e+1],this.z=t[e+2],this}toArray(t=[],e=0){return t[e]=this.x,t[e+1]=this.y,t[e+2]=this.z,t}fromBufferAttribute(t,e){return this.x=t.getX(e),this.y=t.getY(e),this.z=t.getZ(e),this}random(){return this.x=Math.random(),this.y=Math.random(),this.z=Math.random(),this}randomDirection(){const t=Math.random()*Math.PI*2,e=Math.random()*2-1,i=Math.sqrt(1-e*e);return this.x=i*Math.cos(t),this.y=e,this.z=i*Math.sin(t),this}*[Symbol.iterator](){yield this.x,yield this.y,yield this.z}}const fs=new k,oo=new $i;class Yi{constructor(t=new k(1/0,1/0,1/0),e=new k(-1/0,-1/0,-1/0)){this.isBox3=!0,this.min=t,this.max=e}set(t,e){return this.min.copy(t),this.max.copy(e),this}setFromArray(t){this.makeEmpty();for(let e=0,i=t.length;e<i;e+=3)this.expandByPoint(Ne.fromArray(t,e));return this}setFromBufferAttribute(t){this.makeEmpty();for(let e=0,i=t.count;e<i;e++)this.expandByPoint(Ne.fromBufferAttribute(t,e));return this}setFromPoints(t){this.makeEmpty();for(let e=0,i=t.length;e<i;e++)this.expandByPoint(t[e]);return this}setFromCenterAndSize(t,e){const i=Ne.copy(e).multiplyScalar(.5);return this.min.copy(t).sub(i),this.max.copy(t).add(i),this}setFromObject(t,e=!1){return this.makeEmpty(),this.expandByObject(t,e)}clone(){return new this.constructor().copy(this)}copy(t){return this.min.copy(t.min),this.max.copy(t.max),this}makeEmpty(){return this.min.x=this.min.y=this.min.z=1/0,this.max.x=this.max.y=this.max.z=-1/0,this}isEmpty(){return this.max.x<this.min.x||this.max.y<this.min.y||this.max.z<this.min.z}getCenter(t){return this.isEmpty()?t.set(0,0,0):t.addVectors(this.min,this.max).multiplyScalar(.5)}getSize(t){return this.isEmpty()?t.set(0,0,0):t.subVectors(this.max,this.min)}expandByPoint(t){return this.min.min(t),this.max.max(t),this}expandByVector(t){return this.min.sub(t),this.max.add(t),this}expandByScalar(t){return this.min.addScalar(-t),this.max.addScalar(t),this}expandByObject(t,e=!1){t.updateWorldMatrix(!1,!1);const i=t.geometry;if(i!==void 0){const s=i.getAttribute("position");if(e===!0&&s!==void 0&&t.isInstancedMesh!==!0)for(let o=0,a=s.count;o<a;o++)t.isMesh===!0?t.getVertexPosition(o,Ne):Ne.fromBufferAttribute(s,o),Ne.applyMatrix4(t.matrixWorld),this.expandByPoint(Ne);else t.boundingBox!==void 0?(t.boundingBox===null&&t.computeBoundingBox(),Qi.copy(t.boundingBox)):(i.boundingBox===null&&i.computeBoundingBox(),Qi.copy(i.boundingBox)),Qi.applyMatrix4(t.matrixWorld),this.union(Qi)}const r=t.children;for(let s=0,o=r.length;s<o;s++)this.expandByObject(r[s],e);return this}containsPoint(t){return t.x>=this.min.x&&t.x<=this.max.x&&t.y>=this.min.y&&t.y<=this.max.y&&t.z>=this.min.z&&t.z<=this.max.z}containsBox(t){return this.min.x<=t.min.x&&t.max.x<=this.max.x&&this.min.y<=t.min.y&&t.max.y<=this.max.y&&this.min.z<=t.min.z&&t.max.z<=this.max.z}getParameter(t,e){return e.set((t.x-this.min.x)/(this.max.x-this.min.x),(t.y-this.min.y)/(this.max.y-this.min.y),(t.z-this.min.z)/(this.max.z-this.min.z))}intersectsBox(t){return t.max.x>=this.min.x&&t.min.x<=this.max.x&&t.max.y>=this.min.y&&t.min.y<=this.max.y&&t.max.z>=this.min.z&&t.min.z<=this.max.z}intersectsSphere(t){return this.clampPoint(t.center,Ne),Ne.distanceToSquared(t.center)<=t.radius*t.radius}intersectsPlane(t){let e,i;return t.normal.x>0?(e=t.normal.x*this.min.x,i=t.normal.x*this.max.x):(e=t.normal.x*this.max.x,i=t.normal.x*this.min.x),t.normal.y>0?(e+=t.normal.y*this.min.y,i+=t.normal.y*this.max.y):(e+=t.normal.y*this.max.y,i+=t.normal.y*this.min.y),t.normal.z>0?(e+=t.normal.z*this.min.z,i+=t.normal.z*this.max.z):(e+=t.normal.z*this.max.z,i+=t.normal.z*this.min.z),e<=-t.constant&&i>=-t.constant}intersectsTriangle(t){if(this.isEmpty())return!1;this.getCenter(Ii),tr.subVectors(this.max,Ii),ei.subVectors(t.a,Ii),ni.subVectors(t.b,Ii),ii.subVectors(t.c,Ii),mn.subVectors(ni,ei),_n.subVectors(ii,ni),Ln.subVectors(ei,ii);let e=[0,-mn.z,mn.y,0,-_n.z,_n.y,0,-Ln.z,Ln.y,mn.z,0,-mn.x,_n.z,0,-_n.x,Ln.z,0,-Ln.x,-mn.y,mn.x,0,-_n.y,_n.x,0,-Ln.y,Ln.x,0];return!ds(e,ei,ni,ii,tr)||(e=[1,0,0,0,1,0,0,0,1],!ds(e,ei,ni,ii,tr))?!1:(er.crossVectors(mn,_n),e=[er.x,er.y,er.z],ds(e,ei,ni,ii,tr))}clampPoint(t,e){return e.copy(t).clamp(this.min,this.max)}distanceToPoint(t){return this.clampPoint(t,Ne).distanceTo(t)}getBoundingSphere(t){return this.isEmpty()?t.makeEmpty():(this.getCenter(t.center),t.radius=this.getSize(Ne).length()*.5),t}intersect(t){return this.min.max(t.min),this.max.min(t.max),this.isEmpty()&&this.makeEmpty(),this}union(t){return this.min.min(t.min),this.max.max(t.max),this}applyMatrix4(t){return this.isEmpty()?this:(je[0].set(this.min.x,this.min.y,this.min.z).applyMatrix4(t),je[1].set(this.min.x,this.min.y,this.max.z).applyMatrix4(t),je[2].set(this.min.x,this.max.y,this.min.z).applyMatrix4(t),je[3].set(this.min.x,this.max.y,this.max.z).applyMatrix4(t),je[4].set(this.max.x,this.min.y,this.min.z).applyMatrix4(t),je[5].set(this.max.x,this.min.y,this.max.z).applyMatrix4(t),je[6].set(this.max.x,this.max.y,this.min.z).applyMatrix4(t),je[7].set(this.max.x,this.max.y,this.max.z).applyMatrix4(t),this.setFromPoints(je),this)}translate(t){return this.min.add(t),this.max.add(t),this}equals(t){return t.min.equals(this.min)&&t.max.equals(this.max)}}const je=[new k,new k,new k,new k,new k,new k,new k,new k],Ne=new k,Qi=new Yi,ei=new k,ni=new k,ii=new k,mn=new k,_n=new k,Ln=new k,Ii=new k,tr=new k,er=new k,In=new k;function ds(n,t,e,i,r){for(let s=0,o=n.length-3;s<=o;s+=3){In.fromArray(n,s);const a=r.x*Math.abs(In.x)+r.y*Math.abs(In.y)+r.z*Math.abs(In.z),c=t.dot(In),u=e.dot(In),f=i.dot(In);if(Math.max(-Math.max(c,u,f),Math.min(c,u,f))>a)return!1}return!0}const th=new Yi,Di=new k,ps=new k;class Yr{constructor(t=new k,e=-1){this.isSphere=!0,this.center=t,this.radius=e}set(t,e){return this.center.copy(t),this.radius=e,this}setFromPoints(t,e){const i=this.center;e!==void 0?i.copy(e):th.setFromPoints(t).getCenter(i);let r=0;for(let s=0,o=t.length;s<o;s++)r=Math.max(r,i.distanceToSquared(t[s]));return this.radius=Math.sqrt(r),this}copy(t){return this.center.copy(t.center),this.radius=t.radius,this}isEmpty(){return this.radius<0}makeEmpty(){return this.center.set(0,0,0),this.radius=-1,this}containsPoint(t){return t.distanceToSquared(this.center)<=this.radius*this.radius}distanceToPoint(t){return t.distanceTo(this.center)-this.radius}intersectsSphere(t){const e=this.radius+t.radius;return t.center.distanceToSquared(this.center)<=e*e}intersectsBox(t){return t.intersectsSphere(this)}intersectsPlane(t){return Math.abs(t.distanceToPoint(this.center))<=this.radius}clampPoint(t,e){const i=this.center.distanceToSquared(t);return e.copy(t),i>this.radius*this.radius&&(e.sub(this.center).normalize(),e.multiplyScalar(this.radius).add(this.center)),e}getBoundingBox(t){return this.isEmpty()?(t.makeEmpty(),t):(t.set(this.center,this.center),t.expandByScalar(this.radius),t)}applyMatrix4(t){return this.center.applyMatrix4(t),this.radius=this.radius*t.getMaxScaleOnAxis(),this}translate(t){return this.center.add(t),this}expandByPoint(t){if(this.isEmpty())return this.center.copy(t),this.radius=0,this;Di.subVectors(t,this.center);const e=Di.lengthSq();if(e>this.radius*this.radius){const i=Math.sqrt(e),r=(i-this.radius)*.5;this.center.addScaledVector(Di,r/i),this.radius+=r}return this}union(t){return t.isEmpty()?this:this.isEmpty()?(this.copy(t),this):(this.center.equals(t.center)===!0?this.radius=Math.max(this.radius,t.radius):(ps.subVectors(t.center,this.center).setLength(t.radius),this.expandByPoint(Di.copy(t.center).add(ps)),this.expandByPoint(Di.copy(t.center).sub(ps))),this)}equals(t){return t.center.equals(this.center)&&t.radius===this.radius}clone(){return new this.constructor().copy(this)}}const Ze=new k,ms=new k,nr=new k,gn=new k,_s=new k,ir=new k,gs=new k;class Xc{constructor(t=new k,e=new k(0,0,-1)){this.origin=t,this.direction=e}set(t,e){return this.origin.copy(t),this.direction.copy(e),this}copy(t){return this.origin.copy(t.origin),this.direction.copy(t.direction),this}at(t,e){return e.copy(this.origin).addScaledVector(this.direction,t)}lookAt(t){return this.direction.copy(t).sub(this.origin).normalize(),this}recast(t){return this.origin.copy(this.at(t,Ze)),this}closestPointToPoint(t,e){e.subVectors(t,this.origin);const i=e.dot(this.direction);return i<0?e.copy(this.origin):e.copy(this.origin).addScaledVector(this.direction,i)}distanceToPoint(t){return Math.sqrt(this.distanceSqToPoint(t))}distanceSqToPoint(t){const e=Ze.subVectors(t,this.origin).dot(this.direction);return e<0?this.origin.distanceToSquared(t):(Ze.copy(this.origin).addScaledVector(this.direction,e),Ze.distanceToSquared(t))}distanceSqToSegment(t,e,i,r){ms.copy(t).add(e).multiplyScalar(.5),nr.copy(e).sub(t).normalize(),gn.copy(this.origin).sub(ms);const s=t.distanceTo(e)*.5,o=-this.direction.dot(nr),a=gn.dot(this.direction),c=-gn.dot(nr),u=gn.lengthSq(),f=Math.abs(1-o*o);let p,m,g,v;if(f>0)if(p=o*c-a,m=o*a-c,v=s*f,p>=0)if(m>=-v)if(m<=v){const M=1/f;p*=M,m*=M,g=p*(p+o*m+2*a)+m*(o*p+m+2*c)+u}else m=s,p=Math.max(0,-(o*m+a)),g=-p*p+m*(m+2*c)+u;else m=-s,p=Math.max(0,-(o*m+a)),g=-p*p+m*(m+2*c)+u;else m<=-v?(p=Math.max(0,-(-o*s+a)),m=p>0?-s:Math.min(Math.max(-s,-c),s),g=-p*p+m*(m+2*c)+u):m<=v?(p=0,m=Math.min(Math.max(-s,-c),s),g=m*(m+2*c)+u):(p=Math.max(0,-(o*s+a)),m=p>0?s:Math.min(Math.max(-s,-c),s),g=-p*p+m*(m+2*c)+u);else m=o>0?-s:s,p=Math.max(0,-(o*m+a)),g=-p*p+m*(m+2*c)+u;return i&&i.copy(this.origin).addScaledVector(this.direction,p),r&&r.copy(ms).addScaledVector(nr,m),g}intersectSphere(t,e){Ze.subVectors(t.center,this.origin);const i=Ze.dot(this.direction),r=Ze.dot(Ze)-i*i,s=t.radius*t.radius;if(r>s)return null;const o=Math.sqrt(s-r),a=i-o,c=i+o;return c<0?null:a<0?this.at(c,e):this.at(a,e)}intersectsSphere(t){return this.distanceSqToPoint(t.center)<=t.radius*t.radius}distanceToPlane(t){const e=t.normal.dot(this.direction);if(e===0)return t.distanceToPoint(this.origin)===0?0:null;const i=-(this.origin.dot(t.normal)+t.constant)/e;return i>=0?i:null}intersectPlane(t,e){const i=this.distanceToPlane(t);return i===null?null:this.at(i,e)}intersectsPlane(t){const e=t.distanceToPoint(this.origin);return e===0||t.normal.dot(this.direction)*e<0}intersectBox(t,e){let i,r,s,o,a,c;const u=1/this.direction.x,f=1/this.direction.y,p=1/this.direction.z,m=this.origin;return u>=0?(i=(t.min.x-m.x)*u,r=(t.max.x-m.x)*u):(i=(t.max.x-m.x)*u,r=(t.min.x-m.x)*u),f>=0?(s=(t.min.y-m.y)*f,o=(t.max.y-m.y)*f):(s=(t.max.y-m.y)*f,o=(t.min.y-m.y)*f),i>o||s>r||((s>i||isNaN(i))&&(i=s),(o<r||isNaN(r))&&(r=o),p>=0?(a=(t.min.z-m.z)*p,c=(t.max.z-m.z)*p):(a=(t.max.z-m.z)*p,c=(t.min.z-m.z)*p),i>c||a>r)||((a>i||i!==i)&&(i=a),(c<r||r!==r)&&(r=c),r<0)?null:this.at(i>=0?i:r,e)}intersectsBox(t){return this.intersectBox(t,Ze)!==null}intersectTriangle(t,e,i,r,s){_s.subVectors(e,t),ir.subVectors(i,t),gs.crossVectors(_s,ir);let o=this.direction.dot(gs),a;if(o>0){if(r)return null;a=1}else if(o<0)a=-1,o=-o;else return null;gn.subVectors(this.origin,t);const c=a*this.direction.dot(ir.crossVectors(gn,ir));if(c<0)return null;const u=a*this.direction.dot(_s.cross(gn));if(u<0||c+u>o)return null;const f=-a*gn.dot(gs);return f<0?null:this.at(f/o,s)}applyMatrix4(t){return this.origin.applyMatrix4(t),this.direction.transformDirection(t),this}equals(t){return t.origin.equals(this.origin)&&t.direction.equals(this.direction)}clone(){return new this.constructor().copy(this)}}class Qt{constructor(t,e,i,r,s,o,a,c,u,f,p,m,g,v,M,_){Qt.prototype.isMatrix4=!0,this.elements=[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1],t!==void 0&&this.set(t,e,i,r,s,o,a,c,u,f,p,m,g,v,M,_)}set(t,e,i,r,s,o,a,c,u,f,p,m,g,v,M,_){const d=this.elements;return d[0]=t,d[4]=e,d[8]=i,d[12]=r,d[1]=s,d[5]=o,d[9]=a,d[13]=c,d[2]=u,d[6]=f,d[10]=p,d[14]=m,d[3]=g,d[7]=v,d[11]=M,d[15]=_,this}identity(){return this.set(1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1),this}clone(){return new Qt().fromArray(this.elements)}copy(t){const e=this.elements,i=t.elements;return e[0]=i[0],e[1]=i[1],e[2]=i[2],e[3]=i[3],e[4]=i[4],e[5]=i[5],e[6]=i[6],e[7]=i[7],e[8]=i[8],e[9]=i[9],e[10]=i[10],e[11]=i[11],e[12]=i[12],e[13]=i[13],e[14]=i[14],e[15]=i[15],this}copyPosition(t){const e=this.elements,i=t.elements;return e[12]=i[12],e[13]=i[13],e[14]=i[14],this}setFromMatrix3(t){const e=t.elements;return this.set(e[0],e[3],e[6],0,e[1],e[4],e[7],0,e[2],e[5],e[8],0,0,0,0,1),this}extractBasis(t,e,i){return t.setFromMatrixColumn(this,0),e.setFromMatrixColumn(this,1),i.setFromMatrixColumn(this,2),this}makeBasis(t,e,i){return this.set(t.x,e.x,i.x,0,t.y,e.y,i.y,0,t.z,e.z,i.z,0,0,0,0,1),this}extractRotation(t){const e=this.elements,i=t.elements,r=1/ri.setFromMatrixColumn(t,0).length(),s=1/ri.setFromMatrixColumn(t,1).length(),o=1/ri.setFromMatrixColumn(t,2).length();return e[0]=i[0]*r,e[1]=i[1]*r,e[2]=i[2]*r,e[3]=0,e[4]=i[4]*s,e[5]=i[5]*s,e[6]=i[6]*s,e[7]=0,e[8]=i[8]*o,e[9]=i[9]*o,e[10]=i[10]*o,e[11]=0,e[12]=0,e[13]=0,e[14]=0,e[15]=1,this}makeRotationFromEuler(t){const e=this.elements,i=t.x,r=t.y,s=t.z,o=Math.cos(i),a=Math.sin(i),c=Math.cos(r),u=Math.sin(r),f=Math.cos(s),p=Math.sin(s);if(t.order==="XYZ"){const m=o*f,g=o*p,v=a*f,M=a*p;e[0]=c*f,e[4]=-c*p,e[8]=u,e[1]=g+v*u,e[5]=m-M*u,e[9]=-a*c,e[2]=M-m*u,e[6]=v+g*u,e[10]=o*c}else if(t.order==="YXZ"){const m=c*f,g=c*p,v=u*f,M=u*p;e[0]=m+M*a,e[4]=v*a-g,e[8]=o*u,e[1]=o*p,e[5]=o*f,e[9]=-a,e[2]=g*a-v,e[6]=M+m*a,e[10]=o*c}else if(t.order==="ZXY"){const m=c*f,g=c*p,v=u*f,M=u*p;e[0]=m-M*a,e[4]=-o*p,e[8]=v+g*a,e[1]=g+v*a,e[5]=o*f,e[9]=M-m*a,e[2]=-o*u,e[6]=a,e[10]=o*c}else if(t.order==="ZYX"){const m=o*f,g=o*p,v=a*f,M=a*p;e[0]=c*f,e[4]=v*u-g,e[8]=m*u+M,e[1]=c*p,e[5]=M*u+m,e[9]=g*u-v,e[2]=-u,e[6]=a*c,e[10]=o*c}else if(t.order==="YZX"){const m=o*c,g=o*u,v=a*c,M=a*u;e[0]=c*f,e[4]=M-m*p,e[8]=v*p+g,e[1]=p,e[5]=o*f,e[9]=-a*f,e[2]=-u*f,e[6]=g*p+v,e[10]=m-M*p}else if(t.order==="XZY"){const m=o*c,g=o*u,v=a*c,M=a*u;e[0]=c*f,e[4]=-p,e[8]=u*f,e[1]=m*p+M,e[5]=o*f,e[9]=g*p-v,e[2]=v*p-g,e[6]=a*f,e[10]=M*p+m}return e[3]=0,e[7]=0,e[11]=0,e[12]=0,e[13]=0,e[14]=0,e[15]=1,this}makeRotationFromQuaternion(t){return this.compose(eh,t,nh)}lookAt(t,e,i){const r=this.elements;return Ae.subVectors(t,e),Ae.lengthSq()===0&&(Ae.z=1),Ae.normalize(),xn.crossVectors(i,Ae),xn.lengthSq()===0&&(Math.abs(i.z)===1?Ae.x+=1e-4:Ae.z+=1e-4,Ae.normalize(),xn.crossVectors(i,Ae)),xn.normalize(),rr.crossVectors(Ae,xn),r[0]=xn.x,r[4]=rr.x,r[8]=Ae.x,r[1]=xn.y,r[5]=rr.y,r[9]=Ae.y,r[2]=xn.z,r[6]=rr.z,r[10]=Ae.z,this}multiply(t){return this.multiplyMatrices(this,t)}premultiply(t){return this.multiplyMatrices(t,this)}multiplyMatrices(t,e){const i=t.elements,r=e.elements,s=this.elements,o=i[0],a=i[4],c=i[8],u=i[12],f=i[1],p=i[5],m=i[9],g=i[13],v=i[2],M=i[6],_=i[10],d=i[14],C=i[3],w=i[7],b=i[11],z=i[15],P=r[0],I=r[4],F=r[8],A=r[12],E=r[1],N=r[5],$=r[9],W=r[13],Y=r[2],tt=r[6],K=r[10],st=r[14],j=r[3],dt=r[7],_t=r[11],ut=r[15];return s[0]=o*P+a*E+c*Y+u*j,s[4]=o*I+a*N+c*tt+u*dt,s[8]=o*F+a*$+c*K+u*_t,s[12]=o*A+a*W+c*st+u*ut,s[1]=f*P+p*E+m*Y+g*j,s[5]=f*I+p*N+m*tt+g*dt,s[9]=f*F+p*$+m*K+g*_t,s[13]=f*A+p*W+m*st+g*ut,s[2]=v*P+M*E+_*Y+d*j,s[6]=v*I+M*N+_*tt+d*dt,s[10]=v*F+M*$+_*K+d*_t,s[14]=v*A+M*W+_*st+d*ut,s[3]=C*P+w*E+b*Y+z*j,s[7]=C*I+w*N+b*tt+z*dt,s[11]=C*F+w*$+b*K+z*_t,s[15]=C*A+w*W+b*st+z*ut,this}multiplyScalar(t){const e=this.elements;return e[0]*=t,e[4]*=t,e[8]*=t,e[12]*=t,e[1]*=t,e[5]*=t,e[9]*=t,e[13]*=t,e[2]*=t,e[6]*=t,e[10]*=t,e[14]*=t,e[3]*=t,e[7]*=t,e[11]*=t,e[15]*=t,this}determinant(){const t=this.elements,e=t[0],i=t[4],r=t[8],s=t[12],o=t[1],a=t[5],c=t[9],u=t[13],f=t[2],p=t[6],m=t[10],g=t[14],v=t[3],M=t[7],_=t[11],d=t[15];return v*(+s*c*p-r*u*p-s*a*m+i*u*m+r*a*g-i*c*g)+M*(+e*c*g-e*u*m+s*o*m-r*o*g+r*u*f-s*c*f)+_*(+e*u*p-e*a*g-s*o*p+i*o*g+s*a*f-i*u*f)+d*(-r*a*f-e*c*p+e*a*m+r*o*p-i*o*m+i*c*f)}transpose(){const t=this.elements;let e;return e=t[1],t[1]=t[4],t[4]=e,e=t[2],t[2]=t[8],t[8]=e,e=t[6],t[6]=t[9],t[9]=e,e=t[3],t[3]=t[12],t[12]=e,e=t[7],t[7]=t[13],t[13]=e,e=t[11],t[11]=t[14],t[14]=e,this}setPosition(t,e,i){const r=this.elements;return t.isVector3?(r[12]=t.x,r[13]=t.y,r[14]=t.z):(r[12]=t,r[13]=e,r[14]=i),this}invert(){const t=this.elements,e=t[0],i=t[1],r=t[2],s=t[3],o=t[4],a=t[5],c=t[6],u=t[7],f=t[8],p=t[9],m=t[10],g=t[11],v=t[12],M=t[13],_=t[14],d=t[15],C=p*_*u-M*m*u+M*c*g-a*_*g-p*c*d+a*m*d,w=v*m*u-f*_*u-v*c*g+o*_*g+f*c*d-o*m*d,b=f*M*u-v*p*u+v*a*g-o*M*g-f*a*d+o*p*d,z=v*p*c-f*M*c-v*a*m+o*M*m+f*a*_-o*p*_,P=e*C+i*w+r*b+s*z;if(P===0)return this.set(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);const I=1/P;return t[0]=C*I,t[1]=(M*m*s-p*_*s-M*r*g+i*_*g+p*r*d-i*m*d)*I,t[2]=(a*_*s-M*c*s+M*r*u-i*_*u-a*r*d+i*c*d)*I,t[3]=(p*c*s-a*m*s-p*r*u+i*m*u+a*r*g-i*c*g)*I,t[4]=w*I,t[5]=(f*_*s-v*m*s+v*r*g-e*_*g-f*r*d+e*m*d)*I,t[6]=(v*c*s-o*_*s-v*r*u+e*_*u+o*r*d-e*c*d)*I,t[7]=(o*m*s-f*c*s+f*r*u-e*m*u-o*r*g+e*c*g)*I,t[8]=b*I,t[9]=(v*p*s-f*M*s-v*i*g+e*M*g+f*i*d-e*p*d)*I,t[10]=(o*M*s-v*a*s+v*i*u-e*M*u-o*i*d+e*a*d)*I,t[11]=(f*a*s-o*p*s-f*i*u+e*p*u+o*i*g-e*a*g)*I,t[12]=z*I,t[13]=(f*M*r-v*p*r+v*i*m-e*M*m-f*i*_+e*p*_)*I,t[14]=(v*a*r-o*M*r-v*i*c+e*M*c+o*i*_-e*a*_)*I,t[15]=(o*p*r-f*a*r+f*i*c-e*p*c-o*i*m+e*a*m)*I,this}scale(t){const e=this.elements,i=t.x,r=t.y,s=t.z;return e[0]*=i,e[4]*=r,e[8]*=s,e[1]*=i,e[5]*=r,e[9]*=s,e[2]*=i,e[6]*=r,e[10]*=s,e[3]*=i,e[7]*=r,e[11]*=s,this}getMaxScaleOnAxis(){const t=this.elements,e=t[0]*t[0]+t[1]*t[1]+t[2]*t[2],i=t[4]*t[4]+t[5]*t[5]+t[6]*t[6],r=t[8]*t[8]+t[9]*t[9]+t[10]*t[10];return Math.sqrt(Math.max(e,i,r))}makeTranslation(t,e,i){return t.isVector3?this.set(1,0,0,t.x,0,1,0,t.y,0,0,1,t.z,0,0,0,1):this.set(1,0,0,t,0,1,0,e,0,0,1,i,0,0,0,1),this}makeRotationX(t){const e=Math.cos(t),i=Math.sin(t);return this.set(1,0,0,0,0,e,-i,0,0,i,e,0,0,0,0,1),this}makeRotationY(t){const e=Math.cos(t),i=Math.sin(t);return this.set(e,0,i,0,0,1,0,0,-i,0,e,0,0,0,0,1),this}makeRotationZ(t){const e=Math.cos(t),i=Math.sin(t);return this.set(e,-i,0,0,i,e,0,0,0,0,1,0,0,0,0,1),this}makeRotationAxis(t,e){const i=Math.cos(e),r=Math.sin(e),s=1-i,o=t.x,a=t.y,c=t.z,u=s*o,f=s*a;return this.set(u*o+i,u*a-r*c,u*c+r*a,0,u*a+r*c,f*a+i,f*c-r*o,0,u*c-r*a,f*c+r*o,s*c*c+i,0,0,0,0,1),this}makeScale(t,e,i){return this.set(t,0,0,0,0,e,0,0,0,0,i,0,0,0,0,1),this}makeShear(t,e,i,r,s,o){return this.set(1,i,s,0,t,1,o,0,e,r,1,0,0,0,0,1),this}compose(t,e,i){const r=this.elements,s=e._x,o=e._y,a=e._z,c=e._w,u=s+s,f=o+o,p=a+a,m=s*u,g=s*f,v=s*p,M=o*f,_=o*p,d=a*p,C=c*u,w=c*f,b=c*p,z=i.x,P=i.y,I=i.z;return r[0]=(1-(M+d))*z,r[1]=(g+b)*z,r[2]=(v-w)*z,r[3]=0,r[4]=(g-b)*P,r[5]=(1-(m+d))*P,r[6]=(_+C)*P,r[7]=0,r[8]=(v+w)*I,r[9]=(_-C)*I,r[10]=(1-(m+M))*I,r[11]=0,r[12]=t.x,r[13]=t.y,r[14]=t.z,r[15]=1,this}decompose(t,e,i){const r=this.elements;let s=ri.set(r[0],r[1],r[2]).length();const o=ri.set(r[4],r[5],r[6]).length(),a=ri.set(r[8],r[9],r[10]).length();this.determinant()<0&&(s=-s),t.x=r[12],t.y=r[13],t.z=r[14],Fe.copy(this);const u=1/s,f=1/o,p=1/a;return Fe.elements[0]*=u,Fe.elements[1]*=u,Fe.elements[2]*=u,Fe.elements[4]*=f,Fe.elements[5]*=f,Fe.elements[6]*=f,Fe.elements[8]*=p,Fe.elements[9]*=p,Fe.elements[10]*=p,e.setFromRotationMatrix(Fe),i.x=s,i.y=o,i.z=a,this}makePerspective(t,e,i,r,s,o,a=rn){const c=this.elements,u=2*s/(e-t),f=2*s/(i-r),p=(e+t)/(e-t),m=(i+r)/(i-r);let g,v;if(a===rn)g=-(o+s)/(o-s),v=-2*o*s/(o-s);else if(a===zr)g=-o/(o-s),v=-o*s/(o-s);else throw new Error("THREE.Matrix4.makePerspective(): Invalid coordinate system: "+a);return c[0]=u,c[4]=0,c[8]=p,c[12]=0,c[1]=0,c[5]=f,c[9]=m,c[13]=0,c[2]=0,c[6]=0,c[10]=g,c[14]=v,c[3]=0,c[7]=0,c[11]=-1,c[15]=0,this}makeOrthographic(t,e,i,r,s,o,a=rn){const c=this.elements,u=1/(e-t),f=1/(i-r),p=1/(o-s),m=(e+t)*u,g=(i+r)*f;let v,M;if(a===rn)v=(o+s)*p,M=-2*p;else if(a===zr)v=s*p,M=-1*p;else throw new Error("THREE.Matrix4.makeOrthographic(): Invalid coordinate system: "+a);return c[0]=2*u,c[4]=0,c[8]=0,c[12]=-m,c[1]=0,c[5]=2*f,c[9]=0,c[13]=-g,c[2]=0,c[6]=0,c[10]=M,c[14]=-v,c[3]=0,c[7]=0,c[11]=0,c[15]=1,this}equals(t){const e=this.elements,i=t.elements;for(let r=0;r<16;r++)if(e[r]!==i[r])return!1;return!0}fromArray(t,e=0){for(let i=0;i<16;i++)this.elements[i]=t[i+e];return this}toArray(t=[],e=0){const i=this.elements;return t[e]=i[0],t[e+1]=i[1],t[e+2]=i[2],t[e+3]=i[3],t[e+4]=i[4],t[e+5]=i[5],t[e+6]=i[6],t[e+7]=i[7],t[e+8]=i[8],t[e+9]=i[9],t[e+10]=i[10],t[e+11]=i[11],t[e+12]=i[12],t[e+13]=i[13],t[e+14]=i[14],t[e+15]=i[15],t}}const ri=new k,Fe=new Qt,eh=new k(0,0,0),nh=new k(1,1,1),xn=new k,rr=new k,Ae=new k,co=new Qt,lo=new $i;class Ge{constructor(t=0,e=0,i=0,r=Ge.DEFAULT_ORDER){this.isEuler=!0,this._x=t,this._y=e,this._z=i,this._order=r}get x(){return this._x}set x(t){this._x=t,this._onChangeCallback()}get y(){return this._y}set y(t){this._y=t,this._onChangeCallback()}get z(){return this._z}set z(t){this._z=t,this._onChangeCallback()}get order(){return this._order}set order(t){this._order=t,this._onChangeCallback()}set(t,e,i,r=this._order){return this._x=t,this._y=e,this._z=i,this._order=r,this._onChangeCallback(),this}clone(){return new this.constructor(this._x,this._y,this._z,this._order)}copy(t){return this._x=t._x,this._y=t._y,this._z=t._z,this._order=t._order,this._onChangeCallback(),this}setFromRotationMatrix(t,e=this._order,i=!0){const r=t.elements,s=r[0],o=r[4],a=r[8],c=r[1],u=r[5],f=r[9],p=r[2],m=r[6],g=r[10];switch(e){case"XYZ":this._y=Math.asin(Me(a,-1,1)),Math.abs(a)<.9999999?(this._x=Math.atan2(-f,g),this._z=Math.atan2(-o,s)):(this._x=Math.atan2(m,u),this._z=0);break;case"YXZ":this._x=Math.asin(-Me(f,-1,1)),Math.abs(f)<.9999999?(this._y=Math.atan2(a,g),this._z=Math.atan2(c,u)):(this._y=Math.atan2(-p,s),this._z=0);break;case"ZXY":this._x=Math.asin(Me(m,-1,1)),Math.abs(m)<.9999999?(this._y=Math.atan2(-p,g),this._z=Math.atan2(-o,u)):(this._y=0,this._z=Math.atan2(c,s));break;case"ZYX":this._y=Math.asin(-Me(p,-1,1)),Math.abs(p)<.9999999?(this._x=Math.atan2(m,g),this._z=Math.atan2(c,s)):(this._x=0,this._z=Math.atan2(-o,u));break;case"YZX":this._z=Math.asin(Me(c,-1,1)),Math.abs(c)<.9999999?(this._x=Math.atan2(-f,u),this._y=Math.atan2(-p,s)):(this._x=0,this._y=Math.atan2(a,g));break;case"XZY":this._z=Math.asin(-Me(o,-1,1)),Math.abs(o)<.9999999?(this._x=Math.atan2(m,u),this._y=Math.atan2(a,s)):(this._x=Math.atan2(-f,g),this._y=0);break;default:console.warn("THREE.Euler: .setFromRotationMatrix() encountered an unknown order: "+e)}return this._order=e,i===!0&&this._onChangeCallback(),this}setFromQuaternion(t,e,i){return co.makeRotationFromQuaternion(t),this.setFromRotationMatrix(co,e,i)}setFromVector3(t,e=this._order){return this.set(t.x,t.y,t.z,e)}reorder(t){return lo.setFromEuler(this),this.setFromQuaternion(lo,t)}equals(t){return t._x===this._x&&t._y===this._y&&t._z===this._z&&t._order===this._order}fromArray(t){return this._x=t[0],this._y=t[1],this._z=t[2],t[3]!==void 0&&(this._order=t[3]),this._onChangeCallback(),this}toArray(t=[],e=0){return t[e]=this._x,t[e+1]=this._y,t[e+2]=this._z,t[e+3]=this._order,t}_onChange(t){return this._onChangeCallback=t,this}_onChangeCallback(){}*[Symbol.iterator](){yield this._x,yield this._y,yield this._z,yield this._order}}Ge.DEFAULT_ORDER="XYZ";class $c{constructor(){this.mask=1}set(t){this.mask=(1<<t|0)>>>0}enable(t){this.mask|=1<<t|0}enableAll(){this.mask=-1}toggle(t){this.mask^=1<<t|0}disable(t){this.mask&=~(1<<t|0)}disableAll(){this.mask=0}test(t){return(this.mask&t.mask)!==0}isEnabled(t){return(this.mask&(1<<t|0))!==0}}let ih=0;const uo=new k,si=new $i,Je=new Qt,sr=new k,Ui=new k,rh=new k,sh=new $i,ho=new k(1,0,0),fo=new k(0,1,0),po=new k(0,0,1),mo={type:"added"},ah={type:"removed"},ai={type:"childadded",child:null},xs={type:"childremoved",child:null};class fe extends Ai{constructor(){super(),this.isObject3D=!0,Object.defineProperty(this,"id",{value:ih++}),this.uuid=Xi(),this.name="",this.type="Object3D",this.parent=null,this.children=[],this.up=fe.DEFAULT_UP.clone();const t=new k,e=new Ge,i=new $i,r=new k(1,1,1);function s(){i.setFromEuler(e,!1)}function o(){e.setFromQuaternion(i,void 0,!1)}e._onChange(s),i._onChange(o),Object.defineProperties(this,{position:{configurable:!0,enumerable:!0,value:t},rotation:{configurable:!0,enumerable:!0,value:e},quaternion:{configurable:!0,enumerable:!0,value:i},scale:{configurable:!0,enumerable:!0,value:r},modelViewMatrix:{value:new Qt},normalMatrix:{value:new Nt}}),this.matrix=new Qt,this.matrixWorld=new Qt,this.matrixAutoUpdate=fe.DEFAULT_MATRIX_AUTO_UPDATE,this.matrixWorldAutoUpdate=fe.DEFAULT_MATRIX_WORLD_AUTO_UPDATE,this.matrixWorldNeedsUpdate=!1,this.layers=new $c,this.visible=!0,this.castShadow=!1,this.receiveShadow=!1,this.frustumCulled=!0,this.renderOrder=0,this.animations=[],this.userData={}}onBeforeShadow(){}onAfterShadow(){}onBeforeRender(){}onAfterRender(){}applyMatrix4(t){this.matrixAutoUpdate&&this.updateMatrix(),this.matrix.premultiply(t),this.matrix.decompose(this.position,this.quaternion,this.scale)}applyQuaternion(t){return this.quaternion.premultiply(t),this}setRotationFromAxisAngle(t,e){this.quaternion.setFromAxisAngle(t,e)}setRotationFromEuler(t){this.quaternion.setFromEuler(t,!0)}setRotationFromMatrix(t){this.quaternion.setFromRotationMatrix(t)}setRotationFromQuaternion(t){this.quaternion.copy(t)}rotateOnAxis(t,e){return si.setFromAxisAngle(t,e),this.quaternion.multiply(si),this}rotateOnWorldAxis(t,e){return si.setFromAxisAngle(t,e),this.quaternion.premultiply(si),this}rotateX(t){return this.rotateOnAxis(ho,t)}rotateY(t){return this.rotateOnAxis(fo,t)}rotateZ(t){return this.rotateOnAxis(po,t)}translateOnAxis(t,e){return uo.copy(t).applyQuaternion(this.quaternion),this.position.add(uo.multiplyScalar(e)),this}translateX(t){return this.translateOnAxis(ho,t)}translateY(t){return this.translateOnAxis(fo,t)}translateZ(t){return this.translateOnAxis(po,t)}localToWorld(t){return this.updateWorldMatrix(!0,!1),t.applyMatrix4(this.matrixWorld)}worldToLocal(t){return this.updateWorldMatrix(!0,!1),t.applyMatrix4(Je.copy(this.matrixWorld).invert())}lookAt(t,e,i){t.isVector3?sr.copy(t):sr.set(t,e,i);const r=this.parent;this.updateWorldMatrix(!0,!1),Ui.setFromMatrixPosition(this.matrixWorld),this.isCamera||this.isLight?Je.lookAt(Ui,sr,this.up):Je.lookAt(sr,Ui,this.up),this.quaternion.setFromRotationMatrix(Je),r&&(Je.extractRotation(r.matrixWorld),si.setFromRotationMatrix(Je),this.quaternion.premultiply(si.invert()))}add(t){if(arguments.length>1){for(let e=0;e<arguments.length;e++)this.add(arguments[e]);return this}return t===this?(console.error("THREE.Object3D.add: object can't be added as a child of itself.",t),this):(t&&t.isObject3D?(t.removeFromParent(),t.parent=this,this.children.push(t),t.dispatchEvent(mo),ai.child=t,this.dispatchEvent(ai),ai.child=null):console.error("THREE.Object3D.add: object not an instance of THREE.Object3D.",t),this)}remove(t){if(arguments.length>1){for(let i=0;i<arguments.length;i++)this.remove(arguments[i]);return this}const e=this.children.indexOf(t);return e!==-1&&(t.parent=null,this.children.splice(e,1),t.dispatchEvent(ah),xs.child=t,this.dispatchEvent(xs),xs.child=null),this}removeFromParent(){const t=this.parent;return t!==null&&t.remove(this),this}clear(){return this.remove(...this.children)}attach(t){return this.updateWorldMatrix(!0,!1),Je.copy(this.matrixWorld).invert(),t.parent!==null&&(t.parent.updateWorldMatrix(!0,!1),Je.multiply(t.parent.matrixWorld)),t.applyMatrix4(Je),t.removeFromParent(),t.parent=this,this.children.push(t),t.updateWorldMatrix(!1,!0),t.dispatchEvent(mo),ai.child=t,this.dispatchEvent(ai),ai.child=null,this}getObjectById(t){return this.getObjectByProperty("id",t)}getObjectByName(t){return this.getObjectByProperty("name",t)}getObjectByProperty(t,e){if(this[t]===e)return this;for(let i=0,r=this.children.length;i<r;i++){const o=this.children[i].getObjectByProperty(t,e);if(o!==void 0)return o}}getObjectsByProperty(t,e,i=[]){this[t]===e&&i.push(this);const r=this.children;for(let s=0,o=r.length;s<o;s++)r[s].getObjectsByProperty(t,e,i);return i}getWorldPosition(t){return this.updateWorldMatrix(!0,!1),t.setFromMatrixPosition(this.matrixWorld)}getWorldQuaternion(t){return this.updateWorldMatrix(!0,!1),this.matrixWorld.decompose(Ui,t,rh),t}getWorldScale(t){return this.updateWorldMatrix(!0,!1),this.matrixWorld.decompose(Ui,sh,t),t}getWorldDirection(t){this.updateWorldMatrix(!0,!1);const e=this.matrixWorld.elements;return t.set(e[8],e[9],e[10]).normalize()}raycast(){}traverse(t){t(this);const e=this.children;for(let i=0,r=e.length;i<r;i++)e[i].traverse(t)}traverseVisible(t){if(this.visible===!1)return;t(this);const e=this.children;for(let i=0,r=e.length;i<r;i++)e[i].traverseVisible(t)}traverseAncestors(t){const e=this.parent;e!==null&&(t(e),e.traverseAncestors(t))}updateMatrix(){this.matrix.compose(this.position,this.quaternion,this.scale),this.matrixWorldNeedsUpdate=!0}updateMatrixWorld(t){this.matrixAutoUpdate&&this.updateMatrix(),(this.matrixWorldNeedsUpdate||t)&&(this.matrixWorldAutoUpdate===!0&&(this.parent===null?this.matrixWorld.copy(this.matrix):this.matrixWorld.multiplyMatrices(this.parent.matrixWorld,this.matrix)),this.matrixWorldNeedsUpdate=!1,t=!0);const e=this.children;for(let i=0,r=e.length;i<r;i++)e[i].updateMatrixWorld(t)}updateWorldMatrix(t,e){const i=this.parent;if(t===!0&&i!==null&&i.updateWorldMatrix(!0,!1),this.matrixAutoUpdate&&this.updateMatrix(),this.matrixWorldAutoUpdate===!0&&(this.parent===null?this.matrixWorld.copy(this.matrix):this.matrixWorld.multiplyMatrices(this.parent.matrixWorld,this.matrix)),e===!0){const r=this.children;for(let s=0,o=r.length;s<o;s++)r[s].updateWorldMatrix(!1,!0)}}toJSON(t){const e=t===void 0||typeof t=="string",i={};e&&(t={geometries:{},materials:{},textures:{},images:{},shapes:{},skeletons:{},animations:{},nodes:{}},i.metadata={version:4.6,type:"Object",generator:"Object3D.toJSON"});const r={};r.uuid=this.uuid,r.type=this.type,this.name!==""&&(r.name=this.name),this.castShadow===!0&&(r.castShadow=!0),this.receiveShadow===!0&&(r.receiveShadow=!0),this.visible===!1&&(r.visible=!1),this.frustumCulled===!1&&(r.frustumCulled=!1),this.renderOrder!==0&&(r.renderOrder=this.renderOrder),Object.keys(this.userData).length>0&&(r.userData=this.userData),r.layers=this.layers.mask,r.matrix=this.matrix.toArray(),r.up=this.up.toArray(),this.matrixAutoUpdate===!1&&(r.matrixAutoUpdate=!1),this.isInstancedMesh&&(r.type="InstancedMesh",r.count=this.count,r.instanceMatrix=this.instanceMatrix.toJSON(),this.instanceColor!==null&&(r.instanceColor=this.instanceColor.toJSON())),this.isBatchedMesh&&(r.type="BatchedMesh",r.perObjectFrustumCulled=this.perObjectFrustumCulled,r.sortObjects=this.sortObjects,r.drawRanges=this._drawRanges,r.reservedRanges=this._reservedRanges,r.visibility=this._visibility,r.active=this._active,r.bounds=this._bounds.map(a=>({boxInitialized:a.boxInitialized,boxMin:a.box.min.toArray(),boxMax:a.box.max.toArray(),sphereInitialized:a.sphereInitialized,sphereRadius:a.sphere.radius,sphereCenter:a.sphere.center.toArray()})),r.maxInstanceCount=this._maxInstanceCount,r.maxVertexCount=this._maxVertexCount,r.maxIndexCount=this._maxIndexCount,r.geometryInitialized=this._geometryInitialized,r.geometryCount=this._geometryCount,r.matricesTexture=this._matricesTexture.toJSON(t),this._colorsTexture!==null&&(r.colorsTexture=this._colorsTexture.toJSON(t)),this.boundingSphere!==null&&(r.boundingSphere={center:r.boundingSphere.center.toArray(),radius:r.boundingSphere.radius}),this.boundingBox!==null&&(r.boundingBox={min:r.boundingBox.min.toArray(),max:r.boundingBox.max.toArray()}));function s(a,c){return a[c.uuid]===void 0&&(a[c.uuid]=c.toJSON(t)),c.uuid}if(this.isScene)this.background&&(this.background.isColor?r.background=this.background.toJSON():this.background.isTexture&&(r.background=this.background.toJSON(t).uuid)),this.environment&&this.environment.isTexture&&this.environment.isRenderTargetTexture!==!0&&(r.environment=this.environment.toJSON(t).uuid);else if(this.isMesh||this.isLine||this.isPoints){r.geometry=s(t.geometries,this.geometry);const a=this.geometry.parameters;if(a!==void 0&&a.shapes!==void 0){const c=a.shapes;if(Array.isArray(c))for(let u=0,f=c.length;u<f;u++){const p=c[u];s(t.shapes,p)}else s(t.shapes,c)}}if(this.isSkinnedMesh&&(r.bindMode=this.bindMode,r.bindMatrix=this.bindMatrix.toArray(),this.skeleton!==void 0&&(s(t.skeletons,this.skeleton),r.skeleton=this.skeleton.uuid)),this.material!==void 0)if(Array.isArray(this.material)){const a=[];for(let c=0,u=this.material.length;c<u;c++)a.push(s(t.materials,this.material[c]));r.material=a}else r.material=s(t.materials,this.material);if(this.children.length>0){r.children=[];for(let a=0;a<this.children.length;a++)r.children.push(this.children[a].toJSON(t).object)}if(this.animations.length>0){r.animations=[];for(let a=0;a<this.animations.length;a++){const c=this.animations[a];r.animations.push(s(t.animations,c))}}if(e){const a=o(t.geometries),c=o(t.materials),u=o(t.textures),f=o(t.images),p=o(t.shapes),m=o(t.skeletons),g=o(t.animations),v=o(t.nodes);a.length>0&&(i.geometries=a),c.length>0&&(i.materials=c),u.length>0&&(i.textures=u),f.length>0&&(i.images=f),p.length>0&&(i.shapes=p),m.length>0&&(i.skeletons=m),g.length>0&&(i.animations=g),v.length>0&&(i.nodes=v)}return i.object=r,i;function o(a){const c=[];for(const u in a){const f=a[u];delete f.metadata,c.push(f)}return c}}clone(t){return new this.constructor().copy(this,t)}copy(t,e=!0){if(this.name=t.name,this.up.copy(t.up),this.position.copy(t.position),this.rotation.order=t.rotation.order,this.quaternion.copy(t.quaternion),this.scale.copy(t.scale),this.matrix.copy(t.matrix),this.matrixWorld.copy(t.matrixWorld),this.matrixAutoUpdate=t.matrixAutoUpdate,this.matrixWorldAutoUpdate=t.matrixWorldAutoUpdate,this.matrixWorldNeedsUpdate=t.matrixWorldNeedsUpdate,this.layers.mask=t.layers.mask,this.visible=t.visible,this.castShadow=t.castShadow,this.receiveShadow=t.receiveShadow,this.frustumCulled=t.frustumCulled,this.renderOrder=t.renderOrder,this.animations=t.animations.slice(),this.userData=JSON.parse(JSON.stringify(t.userData)),e===!0)for(let i=0;i<t.children.length;i++){const r=t.children[i];this.add(r.clone())}return this}}fe.DEFAULT_UP=new k(0,1,0);fe.DEFAULT_MATRIX_AUTO_UPDATE=!0;fe.DEFAULT_MATRIX_WORLD_AUTO_UPDATE=!0;const Oe=new k,Qe=new k,vs=new k,tn=new k,oi=new k,ci=new k,_o=new k,ys=new k,Ms=new k,Ss=new k;class $e{constructor(t=new k,e=new k,i=new k){this.a=t,this.b=e,this.c=i}static getNormal(t,e,i,r){r.subVectors(i,e),Oe.subVectors(t,e),r.cross(Oe);const s=r.lengthSq();return s>0?r.multiplyScalar(1/Math.sqrt(s)):r.set(0,0,0)}static getBarycoord(t,e,i,r,s){Oe.subVectors(r,e),Qe.subVectors(i,e),vs.subVectors(t,e);const o=Oe.dot(Oe),a=Oe.dot(Qe),c=Oe.dot(vs),u=Qe.dot(Qe),f=Qe.dot(vs),p=o*u-a*a;if(p===0)return s.set(0,0,0),null;const m=1/p,g=(u*c-a*f)*m,v=(o*f-a*c)*m;return s.set(1-g-v,v,g)}static containsPoint(t,e,i,r){return this.getBarycoord(t,e,i,r,tn)===null?!1:tn.x>=0&&tn.y>=0&&tn.x+tn.y<=1}static getInterpolation(t,e,i,r,s,o,a,c){return this.getBarycoord(t,e,i,r,tn)===null?(c.x=0,c.y=0,"z"in c&&(c.z=0),"w"in c&&(c.w=0),null):(c.setScalar(0),c.addScaledVector(s,tn.x),c.addScaledVector(o,tn.y),c.addScaledVector(a,tn.z),c)}static isFrontFacing(t,e,i,r){return Oe.subVectors(i,e),Qe.subVectors(t,e),Oe.cross(Qe).dot(r)<0}set(t,e,i){return this.a.copy(t),this.b.copy(e),this.c.copy(i),this}setFromPointsAndIndices(t,e,i,r){return this.a.copy(t[e]),this.b.copy(t[i]),this.c.copy(t[r]),this}setFromAttributeAndIndices(t,e,i,r){return this.a.fromBufferAttribute(t,e),this.b.fromBufferAttribute(t,i),this.c.fromBufferAttribute(t,r),this}clone(){return new this.constructor().copy(this)}copy(t){return this.a.copy(t.a),this.b.copy(t.b),this.c.copy(t.c),this}getArea(){return Oe.subVectors(this.c,this.b),Qe.subVectors(this.a,this.b),Oe.cross(Qe).length()*.5}getMidpoint(t){return t.addVectors(this.a,this.b).add(this.c).multiplyScalar(1/3)}getNormal(t){return $e.getNormal(this.a,this.b,this.c,t)}getPlane(t){return t.setFromCoplanarPoints(this.a,this.b,this.c)}getBarycoord(t,e){return $e.getBarycoord(t,this.a,this.b,this.c,e)}getInterpolation(t,e,i,r,s){return $e.getInterpolation(t,this.a,this.b,this.c,e,i,r,s)}containsPoint(t){return $e.containsPoint(t,this.a,this.b,this.c)}isFrontFacing(t){return $e.isFrontFacing(this.a,this.b,this.c,t)}intersectsBox(t){return t.intersectsTriangle(this)}closestPointToPoint(t,e){const i=this.a,r=this.b,s=this.c;let o,a;oi.subVectors(r,i),ci.subVectors(s,i),ys.subVectors(t,i);const c=oi.dot(ys),u=ci.dot(ys);if(c<=0&&u<=0)return e.copy(i);Ms.subVectors(t,r);const f=oi.dot(Ms),p=ci.dot(Ms);if(f>=0&&p<=f)return e.copy(r);const m=c*p-f*u;if(m<=0&&c>=0&&f<=0)return o=c/(c-f),e.copy(i).addScaledVector(oi,o);Ss.subVectors(t,s);const g=oi.dot(Ss),v=ci.dot(Ss);if(v>=0&&g<=v)return e.copy(s);const M=g*u-c*v;if(M<=0&&u>=0&&v<=0)return a=u/(u-v),e.copy(i).addScaledVector(ci,a);const _=f*v-g*p;if(_<=0&&p-f>=0&&g-v>=0)return _o.subVectors(s,r),a=(p-f)/(p-f+(g-v)),e.copy(r).addScaledVector(_o,a);const d=1/(_+M+m);return o=M*d,a=m*d,e.copy(i).addScaledVector(oi,o).addScaledVector(ci,a)}equals(t){return t.a.equals(this.a)&&t.b.equals(this.b)&&t.c.equals(this.c)}}const Yc={aliceblue:15792383,antiquewhite:16444375,aqua:65535,aquamarine:8388564,azure:15794175,beige:16119260,bisque:16770244,black:0,blanchedalmond:16772045,blue:255,blueviolet:9055202,brown:10824234,burlywood:14596231,cadetblue:6266528,chartreuse:8388352,chocolate:13789470,coral:16744272,cornflowerblue:6591981,cornsilk:16775388,crimson:14423100,cyan:65535,darkblue:139,darkcyan:35723,darkgoldenrod:12092939,darkgray:11119017,darkgreen:25600,darkgrey:11119017,darkkhaki:12433259,darkmagenta:9109643,darkolivegreen:5597999,darkorange:16747520,darkorchid:10040012,darkred:9109504,darksalmon:15308410,darkseagreen:9419919,darkslateblue:4734347,darkslategray:3100495,darkslategrey:3100495,darkturquoise:52945,darkviolet:9699539,deeppink:16716947,deepskyblue:49151,dimgray:6908265,dimgrey:6908265,dodgerblue:2003199,firebrick:11674146,floralwhite:16775920,forestgreen:2263842,fuchsia:16711935,gainsboro:14474460,ghostwhite:16316671,gold:16766720,goldenrod:14329120,gray:8421504,green:32768,greenyellow:11403055,grey:8421504,honeydew:15794160,hotpink:16738740,indianred:13458524,indigo:4915330,ivory:16777200,khaki:15787660,lavender:15132410,lavenderblush:16773365,lawngreen:8190976,lemonchiffon:16775885,lightblue:11393254,lightcoral:15761536,lightcyan:14745599,lightgoldenrodyellow:16448210,lightgray:13882323,lightgreen:9498256,lightgrey:13882323,lightpink:16758465,lightsalmon:16752762,lightseagreen:2142890,lightskyblue:8900346,lightslategray:7833753,lightslategrey:7833753,lightsteelblue:11584734,lightyellow:16777184,lime:65280,limegreen:3329330,linen:16445670,magenta:16711935,maroon:8388608,mediumaquamarine:6737322,mediumblue:205,mediumorchid:12211667,mediumpurple:9662683,mediumseagreen:3978097,mediumslateblue:8087790,mediumspringgreen:64154,mediumturquoise:4772300,mediumvioletred:13047173,midnightblue:1644912,mintcream:16121850,mistyrose:16770273,moccasin:16770229,navajowhite:16768685,navy:128,oldlace:16643558,olive:8421376,olivedrab:7048739,orange:16753920,orangered:16729344,orchid:14315734,palegoldenrod:15657130,palegreen:10025880,paleturquoise:11529966,palevioletred:14381203,papayawhip:16773077,peachpuff:16767673,peru:13468991,pink:16761035,plum:14524637,powderblue:11591910,purple:8388736,rebeccapurple:6697881,red:16711680,rosybrown:12357519,royalblue:4286945,saddlebrown:9127187,salmon:16416882,sandybrown:16032864,seagreen:3050327,seashell:16774638,sienna:10506797,silver:12632256,skyblue:8900331,slateblue:6970061,slategray:7372944,slategrey:7372944,snow:16775930,springgreen:65407,steelblue:4620980,tan:13808780,teal:32896,thistle:14204888,tomato:16737095,turquoise:4251856,violet:15631086,wheat:16113331,white:16777215,whitesmoke:16119285,yellow:16776960,yellowgreen:10145074},vn={h:0,s:0,l:0},ar={h:0,s:0,l:0};function Es(n,t,e){return e<0&&(e+=1),e>1&&(e-=1),e<1/6?n+(t-n)*6*e:e<1/2?t:e<2/3?n+(t-n)*6*(2/3-e):n}class Ht{constructor(t,e,i){return this.isColor=!0,this.r=1,this.g=1,this.b=1,this.set(t,e,i)}set(t,e,i){if(e===void 0&&i===void 0){const r=t;r&&r.isColor?this.copy(r):typeof r=="number"?this.setHex(r):typeof r=="string"&&this.setStyle(r)}else this.setRGB(t,e,i);return this}setScalar(t){return this.r=t,this.g=t,this.b=t,this}setHex(t,e=We){return t=Math.floor(t),this.r=(t>>16&255)/255,this.g=(t>>8&255)/255,this.b=(t&255)/255,$t.toWorkingColorSpace(this,e),this}setRGB(t,e,i,r=$t.workingColorSpace){return this.r=t,this.g=e,this.b=i,$t.toWorkingColorSpace(this,r),this}setHSL(t,e,i,r=$t.workingColorSpace){if(t=qu(t,1),e=Me(e,0,1),i=Me(i,0,1),e===0)this.r=this.g=this.b=i;else{const s=i<=.5?i*(1+e):i+e-i*e,o=2*i-s;this.r=Es(o,s,t+1/3),this.g=Es(o,s,t),this.b=Es(o,s,t-1/3)}return $t.toWorkingColorSpace(this,r),this}setStyle(t,e=We){function i(s){s!==void 0&&parseFloat(s)<1&&console.warn("THREE.Color: Alpha component of "+t+" will be ignored.")}let r;if(r=/^(\w+)\(([^\)]*)\)/.exec(t)){let s;const o=r[1],a=r[2];switch(o){case"rgb":case"rgba":if(s=/^\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(a))return i(s[4]),this.setRGB(Math.min(255,parseInt(s[1],10))/255,Math.min(255,parseInt(s[2],10))/255,Math.min(255,parseInt(s[3],10))/255,e);if(s=/^\s*(\d+)\%\s*,\s*(\d+)\%\s*,\s*(\d+)\%\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(a))return i(s[4]),this.setRGB(Math.min(100,parseInt(s[1],10))/100,Math.min(100,parseInt(s[2],10))/100,Math.min(100,parseInt(s[3],10))/100,e);break;case"hsl":case"hsla":if(s=/^\s*(\d*\.?\d+)\s*,\s*(\d*\.?\d+)\%\s*,\s*(\d*\.?\d+)\%\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(a))return i(s[4]),this.setHSL(parseFloat(s[1])/360,parseFloat(s[2])/100,parseFloat(s[3])/100,e);break;default:console.warn("THREE.Color: Unknown color model "+t)}}else if(r=/^\#([A-Fa-f\d]+)$/.exec(t)){const s=r[1],o=s.length;if(o===3)return this.setRGB(parseInt(s.charAt(0),16)/15,parseInt(s.charAt(1),16)/15,parseInt(s.charAt(2),16)/15,e);if(o===6)return this.setHex(parseInt(s,16),e);console.warn("THREE.Color: Invalid hex color "+t)}else if(t&&t.length>0)return this.setColorName(t,e);return this}setColorName(t,e=We){const i=Yc[t.toLowerCase()];return i!==void 0?this.setHex(i,e):console.warn("THREE.Color: Unknown color "+t),this}clone(){return new this.constructor(this.r,this.g,this.b)}copy(t){return this.r=t.r,this.g=t.g,this.b=t.b,this}copySRGBToLinear(t){return this.r=vi(t.r),this.g=vi(t.g),this.b=vi(t.b),this}copyLinearToSRGB(t){return this.r=us(t.r),this.g=us(t.g),this.b=us(t.b),this}convertSRGBToLinear(){return this.copySRGBToLinear(this),this}convertLinearToSRGB(){return this.copyLinearToSRGB(this),this}getHex(t=We){return $t.fromWorkingColorSpace(pe.copy(this),t),Math.round(Me(pe.r*255,0,255))*65536+Math.round(Me(pe.g*255,0,255))*256+Math.round(Me(pe.b*255,0,255))}getHexString(t=We){return("000000"+this.getHex(t).toString(16)).slice(-6)}getHSL(t,e=$t.workingColorSpace){$t.fromWorkingColorSpace(pe.copy(this),e);const i=pe.r,r=pe.g,s=pe.b,o=Math.max(i,r,s),a=Math.min(i,r,s);let c,u;const f=(a+o)/2;if(a===o)c=0,u=0;else{const p=o-a;switch(u=f<=.5?p/(o+a):p/(2-o-a),o){case i:c=(r-s)/p+(r<s?6:0);break;case r:c=(s-i)/p+2;break;case s:c=(i-r)/p+4;break}c/=6}return t.h=c,t.s=u,t.l=f,t}getRGB(t,e=$t.workingColorSpace){return $t.fromWorkingColorSpace(pe.copy(this),e),t.r=pe.r,t.g=pe.g,t.b=pe.b,t}getStyle(t=We){$t.fromWorkingColorSpace(pe.copy(this),t);const e=pe.r,i=pe.g,r=pe.b;return t!==We?`color(${t} ${e.toFixed(3)} ${i.toFixed(3)} ${r.toFixed(3)})`:`rgb(${Math.round(e*255)},${Math.round(i*255)},${Math.round(r*255)})`}offsetHSL(t,e,i){return this.getHSL(vn),this.setHSL(vn.h+t,vn.s+e,vn.l+i)}add(t){return this.r+=t.r,this.g+=t.g,this.b+=t.b,this}addColors(t,e){return this.r=t.r+e.r,this.g=t.g+e.g,this.b=t.b+e.b,this}addScalar(t){return this.r+=t,this.g+=t,this.b+=t,this}sub(t){return this.r=Math.max(0,this.r-t.r),this.g=Math.max(0,this.g-t.g),this.b=Math.max(0,this.b-t.b),this}multiply(t){return this.r*=t.r,this.g*=t.g,this.b*=t.b,this}multiplyScalar(t){return this.r*=t,this.g*=t,this.b*=t,this}lerp(t,e){return this.r+=(t.r-this.r)*e,this.g+=(t.g-this.g)*e,this.b+=(t.b-this.b)*e,this}lerpColors(t,e,i){return this.r=t.r+(e.r-t.r)*i,this.g=t.g+(e.g-t.g)*i,this.b=t.b+(e.b-t.b)*i,this}lerpHSL(t,e){this.getHSL(vn),t.getHSL(ar);const i=cs(vn.h,ar.h,e),r=cs(vn.s,ar.s,e),s=cs(vn.l,ar.l,e);return this.setHSL(i,r,s),this}setFromVector3(t){return this.r=t.x,this.g=t.y,this.b=t.z,this}applyMatrix3(t){const e=this.r,i=this.g,r=this.b,s=t.elements;return this.r=s[0]*e+s[3]*i+s[6]*r,this.g=s[1]*e+s[4]*i+s[7]*r,this.b=s[2]*e+s[5]*i+s[8]*r,this}equals(t){return t.r===this.r&&t.g===this.g&&t.b===this.b}fromArray(t,e=0){return this.r=t[e],this.g=t[e+1],this.b=t[e+2],this}toArray(t=[],e=0){return t[e]=this.r,t[e+1]=this.g,t[e+2]=this.b,t}fromBufferAttribute(t,e){return this.r=t.getX(e),this.g=t.getY(e),this.b=t.getZ(e),this}toJSON(){return this.getHex()}*[Symbol.iterator](){yield this.r,yield this.g,yield this.b}}const pe=new Ht;Ht.NAMES=Yc;let oh=0;class Kn extends Ai{constructor(){super(),this.isMaterial=!0,Object.defineProperty(this,"id",{value:oh++}),this.uuid=Xi(),this.name="",this.type="Material",this.blending=gi,this.side=An,this.vertexColors=!1,this.opacity=1,this.transparent=!1,this.alphaHash=!1,this.blendSrc=ks,this.blendDst=Gs,this.blendEquation=Hn,this.blendSrcAlpha=null,this.blendDstAlpha=null,this.blendEquationAlpha=null,this.blendColor=new Ht(0,0,0),this.blendAlpha=0,this.depthFunc=Nr,this.depthTest=!0,this.depthWrite=!0,this.stencilWriteMask=255,this.stencilFunc=eo,this.stencilRef=0,this.stencilFuncMask=255,this.stencilFail=Qn,this.stencilZFail=Qn,this.stencilZPass=Qn,this.stencilWrite=!1,this.clippingPlanes=null,this.clipIntersection=!1,this.clipShadows=!1,this.shadowSide=null,this.colorWrite=!0,this.precision=null,this.polygonOffset=!1,this.polygonOffsetFactor=0,this.polygonOffsetUnits=0,this.dithering=!1,this.alphaToCoverage=!1,this.premultipliedAlpha=!1,this.forceSinglePass=!1,this.visible=!0,this.toneMapped=!0,this.userData={},this.version=0,this._alphaTest=0}get alphaTest(){return this._alphaTest}set alphaTest(t){this._alphaTest>0!=t>0&&this.version++,this._alphaTest=t}onBeforeCompile(){}customProgramCacheKey(){return this.onBeforeCompile.toString()}setValues(t){if(t!==void 0)for(const e in t){const i=t[e];if(i===void 0){console.warn(`THREE.Material: parameter '${e}' has value of undefined.`);continue}const r=this[e];if(r===void 0){console.warn(`THREE.Material: '${e}' is not a property of THREE.${this.type}.`);continue}r&&r.isColor?r.set(i):r&&r.isVector3&&i&&i.isVector3?r.copy(i):this[e]=i}}toJSON(t){const e=t===void 0||typeof t=="string";e&&(t={textures:{},images:{}});const i={metadata:{version:4.6,type:"Material",generator:"Material.toJSON"}};i.uuid=this.uuid,i.type=this.type,this.name!==""&&(i.name=this.name),this.color&&this.color.isColor&&(i.color=this.color.getHex()),this.roughness!==void 0&&(i.roughness=this.roughness),this.metalness!==void 0&&(i.metalness=this.metalness),this.sheen!==void 0&&(i.sheen=this.sheen),this.sheenColor&&this.sheenColor.isColor&&(i.sheenColor=this.sheenColor.getHex()),this.sheenRoughness!==void 0&&(i.sheenRoughness=this.sheenRoughness),this.emissive&&this.emissive.isColor&&(i.emissive=this.emissive.getHex()),this.emissiveIntensity!==void 0&&this.emissiveIntensity!==1&&(i.emissiveIntensity=this.emissiveIntensity),this.specular&&this.specular.isColor&&(i.specular=this.specular.getHex()),this.specularIntensity!==void 0&&(i.specularIntensity=this.specularIntensity),this.specularColor&&this.specularColor.isColor&&(i.specularColor=this.specularColor.getHex()),this.shininess!==void 0&&(i.shininess=this.shininess),this.clearcoat!==void 0&&(i.clearcoat=this.clearcoat),this.clearcoatRoughness!==void 0&&(i.clearcoatRoughness=this.clearcoatRoughness),this.clearcoatMap&&this.clearcoatMap.isTexture&&(i.clearcoatMap=this.clearcoatMap.toJSON(t).uuid),this.clearcoatRoughnessMap&&this.clearcoatRoughnessMap.isTexture&&(i.clearcoatRoughnessMap=this.clearcoatRoughnessMap.toJSON(t).uuid),this.clearcoatNormalMap&&this.clearcoatNormalMap.isTexture&&(i.clearcoatNormalMap=this.clearcoatNormalMap.toJSON(t).uuid,i.clearcoatNormalScale=this.clearcoatNormalScale.toArray()),this.dispersion!==void 0&&(i.dispersion=this.dispersion),this.iridescence!==void 0&&(i.iridescence=this.iridescence),this.iridescenceIOR!==void 0&&(i.iridescenceIOR=this.iridescenceIOR),this.iridescenceThicknessRange!==void 0&&(i.iridescenceThicknessRange=this.iridescenceThicknessRange),this.iridescenceMap&&this.iridescenceMap.isTexture&&(i.iridescenceMap=this.iridescenceMap.toJSON(t).uuid),this.iridescenceThicknessMap&&this.iridescenceThicknessMap.isTexture&&(i.iridescenceThicknessMap=this.iridescenceThicknessMap.toJSON(t).uuid),this.anisotropy!==void 0&&(i.anisotropy=this.anisotropy),this.anisotropyRotation!==void 0&&(i.anisotropyRotation=this.anisotropyRotation),this.anisotropyMap&&this.anisotropyMap.isTexture&&(i.anisotropyMap=this.anisotropyMap.toJSON(t).uuid),this.map&&this.map.isTexture&&(i.map=this.map.toJSON(t).uuid),this.matcap&&this.matcap.isTexture&&(i.matcap=this.matcap.toJSON(t).uuid),this.alphaMap&&this.alphaMap.isTexture&&(i.alphaMap=this.alphaMap.toJSON(t).uuid),this.lightMap&&this.lightMap.isTexture&&(i.lightMap=this.lightMap.toJSON(t).uuid,i.lightMapIntensity=this.lightMapIntensity),this.aoMap&&this.aoMap.isTexture&&(i.aoMap=this.aoMap.toJSON(t).uuid,i.aoMapIntensity=this.aoMapIntensity),this.bumpMap&&this.bumpMap.isTexture&&(i.bumpMap=this.bumpMap.toJSON(t).uuid,i.bumpScale=this.bumpScale),this.normalMap&&this.normalMap.isTexture&&(i.normalMap=this.normalMap.toJSON(t).uuid,i.normalMapType=this.normalMapType,i.normalScale=this.normalScale.toArray()),this.displacementMap&&this.displacementMap.isTexture&&(i.displacementMap=this.displacementMap.toJSON(t).uuid,i.displacementScale=this.displacementScale,i.displacementBias=this.displacementBias),this.roughnessMap&&this.roughnessMap.isTexture&&(i.roughnessMap=this.roughnessMap.toJSON(t).uuid),this.metalnessMap&&this.metalnessMap.isTexture&&(i.metalnessMap=this.metalnessMap.toJSON(t).uuid),this.emissiveMap&&this.emissiveMap.isTexture&&(i.emissiveMap=this.emissiveMap.toJSON(t).uuid),this.specularMap&&this.specularMap.isTexture&&(i.specularMap=this.specularMap.toJSON(t).uuid),this.specularIntensityMap&&this.specularIntensityMap.isTexture&&(i.specularIntensityMap=this.specularIntensityMap.toJSON(t).uuid),this.specularColorMap&&this.specularColorMap.isTexture&&(i.specularColorMap=this.specularColorMap.toJSON(t).uuid),this.envMap&&this.envMap.isTexture&&(i.envMap=this.envMap.toJSON(t).uuid,this.combine!==void 0&&(i.combine=this.combine)),this.envMapRotation!==void 0&&(i.envMapRotation=this.envMapRotation.toArray()),this.envMapIntensity!==void 0&&(i.envMapIntensity=this.envMapIntensity),this.reflectivity!==void 0&&(i.reflectivity=this.reflectivity),this.refractionRatio!==void 0&&(i.refractionRatio=this.refractionRatio),this.gradientMap&&this.gradientMap.isTexture&&(i.gradientMap=this.gradientMap.toJSON(t).uuid),this.transmission!==void 0&&(i.transmission=this.transmission),this.transmissionMap&&this.transmissionMap.isTexture&&(i.transmissionMap=this.transmissionMap.toJSON(t).uuid),this.thickness!==void 0&&(i.thickness=this.thickness),this.thicknessMap&&this.thicknessMap.isTexture&&(i.thicknessMap=this.thicknessMap.toJSON(t).uuid),this.attenuationDistance!==void 0&&this.attenuationDistance!==1/0&&(i.attenuationDistance=this.attenuationDistance),this.attenuationColor!==void 0&&(i.attenuationColor=this.attenuationColor.getHex()),this.size!==void 0&&(i.size=this.size),this.shadowSide!==null&&(i.shadowSide=this.shadowSide),this.sizeAttenuation!==void 0&&(i.sizeAttenuation=this.sizeAttenuation),this.blending!==gi&&(i.blending=this.blending),this.side!==An&&(i.side=this.side),this.vertexColors===!0&&(i.vertexColors=!0),this.opacity<1&&(i.opacity=this.opacity),this.transparent===!0&&(i.transparent=!0),this.blendSrc!==ks&&(i.blendSrc=this.blendSrc),this.blendDst!==Gs&&(i.blendDst=this.blendDst),this.blendEquation!==Hn&&(i.blendEquation=this.blendEquation),this.blendSrcAlpha!==null&&(i.blendSrcAlpha=this.blendSrcAlpha),this.blendDstAlpha!==null&&(i.blendDstAlpha=this.blendDstAlpha),this.blendEquationAlpha!==null&&(i.blendEquationAlpha=this.blendEquationAlpha),this.blendColor&&this.blendColor.isColor&&(i.blendColor=this.blendColor.getHex()),this.blendAlpha!==0&&(i.blendAlpha=this.blendAlpha),this.depthFunc!==Nr&&(i.depthFunc=this.depthFunc),this.depthTest===!1&&(i.depthTest=this.depthTest),this.depthWrite===!1&&(i.depthWrite=this.depthWrite),this.colorWrite===!1&&(i.colorWrite=this.colorWrite),this.stencilWriteMask!==255&&(i.stencilWriteMask=this.stencilWriteMask),this.stencilFunc!==eo&&(i.stencilFunc=this.stencilFunc),this.stencilRef!==0&&(i.stencilRef=this.stencilRef),this.stencilFuncMask!==255&&(i.stencilFuncMask=this.stencilFuncMask),this.stencilFail!==Qn&&(i.stencilFail=this.stencilFail),this.stencilZFail!==Qn&&(i.stencilZFail=this.stencilZFail),this.stencilZPass!==Qn&&(i.stencilZPass=this.stencilZPass),this.stencilWrite===!0&&(i.stencilWrite=this.stencilWrite),this.rotation!==void 0&&this.rotation!==0&&(i.rotation=this.rotation),this.polygonOffset===!0&&(i.polygonOffset=!0),this.polygonOffsetFactor!==0&&(i.polygonOffsetFactor=this.polygonOffsetFactor),this.polygonOffsetUnits!==0&&(i.polygonOffsetUnits=this.polygonOffsetUnits),this.linewidth!==void 0&&this.linewidth!==1&&(i.linewidth=this.linewidth),this.dashSize!==void 0&&(i.dashSize=this.dashSize),this.gapSize!==void 0&&(i.gapSize=this.gapSize),this.scale!==void 0&&(i.scale=this.scale),this.dithering===!0&&(i.dithering=!0),this.alphaTest>0&&(i.alphaTest=this.alphaTest),this.alphaHash===!0&&(i.alphaHash=!0),this.alphaToCoverage===!0&&(i.alphaToCoverage=!0),this.premultipliedAlpha===!0&&(i.premultipliedAlpha=!0),this.forceSinglePass===!0&&(i.forceSinglePass=!0),this.wireframe===!0&&(i.wireframe=!0),this.wireframeLinewidth>1&&(i.wireframeLinewidth=this.wireframeLinewidth),this.wireframeLinecap!=="round"&&(i.wireframeLinecap=this.wireframeLinecap),this.wireframeLinejoin!=="round"&&(i.wireframeLinejoin=this.wireframeLinejoin),this.flatShading===!0&&(i.flatShading=!0),this.visible===!1&&(i.visible=!1),this.toneMapped===!1&&(i.toneMapped=!1),this.fog===!1&&(i.fog=!1),Object.keys(this.userData).length>0&&(i.userData=this.userData);function r(s){const o=[];for(const a in s){const c=s[a];delete c.metadata,o.push(c)}return o}if(e){const s=r(t.textures),o=r(t.images);s.length>0&&(i.textures=s),o.length>0&&(i.images=o)}return i}clone(){return new this.constructor().copy(this)}copy(t){this.name=t.name,this.blending=t.blending,this.side=t.side,this.vertexColors=t.vertexColors,this.opacity=t.opacity,this.transparent=t.transparent,this.blendSrc=t.blendSrc,this.blendDst=t.blendDst,this.blendEquation=t.blendEquation,this.blendSrcAlpha=t.blendSrcAlpha,this.blendDstAlpha=t.blendDstAlpha,this.blendEquationAlpha=t.blendEquationAlpha,this.blendColor.copy(t.blendColor),this.blendAlpha=t.blendAlpha,this.depthFunc=t.depthFunc,this.depthTest=t.depthTest,this.depthWrite=t.depthWrite,this.stencilWriteMask=t.stencilWriteMask,this.stencilFunc=t.stencilFunc,this.stencilRef=t.stencilRef,this.stencilFuncMask=t.stencilFuncMask,this.stencilFail=t.stencilFail,this.stencilZFail=t.stencilZFail,this.stencilZPass=t.stencilZPass,this.stencilWrite=t.stencilWrite;const e=t.clippingPlanes;let i=null;if(e!==null){const r=e.length;i=new Array(r);for(let s=0;s!==r;++s)i[s]=e[s].clone()}return this.clippingPlanes=i,this.clipIntersection=t.clipIntersection,this.clipShadows=t.clipShadows,this.shadowSide=t.shadowSide,this.colorWrite=t.colorWrite,this.precision=t.precision,this.polygonOffset=t.polygonOffset,this.polygonOffsetFactor=t.polygonOffsetFactor,this.polygonOffsetUnits=t.polygonOffsetUnits,this.dithering=t.dithering,this.alphaTest=t.alphaTest,this.alphaHash=t.alphaHash,this.alphaToCoverage=t.alphaToCoverage,this.premultipliedAlpha=t.premultipliedAlpha,this.forceSinglePass=t.forceSinglePass,this.visible=t.visible,this.toneMapped=t.toneMapped,this.userData=JSON.parse(JSON.stringify(t.userData)),this}dispose(){this.dispatchEvent({type:"dispose"})}set needsUpdate(t){t===!0&&this.version++}onBuild(){console.warn("Material: onBuild() has been removed.")}onBeforeRender(){console.warn("Material: onBeforeRender() has been removed.")}}class sn extends Kn{constructor(t){super(),this.isMeshBasicMaterial=!0,this.type="MeshBasicMaterial",this.color=new Ht(16777215),this.map=null,this.lightMap=null,this.lightMapIntensity=1,this.aoMap=null,this.aoMapIntensity=1,this.specularMap=null,this.alphaMap=null,this.envMap=null,this.envMapRotation=new Ge,this.combine=Pc,this.reflectivity=1,this.refractionRatio=.98,this.wireframe=!1,this.wireframeLinewidth=1,this.wireframeLinecap="round",this.wireframeLinejoin="round",this.fog=!0,this.setValues(t)}copy(t){return super.copy(t),this.color.copy(t.color),this.map=t.map,this.lightMap=t.lightMap,this.lightMapIntensity=t.lightMapIntensity,this.aoMap=t.aoMap,this.aoMapIntensity=t.aoMapIntensity,this.specularMap=t.specularMap,this.alphaMap=t.alphaMap,this.envMap=t.envMap,this.envMapRotation.copy(t.envMapRotation),this.combine=t.combine,this.reflectivity=t.reflectivity,this.refractionRatio=t.refractionRatio,this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this.wireframeLinecap=t.wireframeLinecap,this.wireframeLinejoin=t.wireframeLinejoin,this.fog=t.fog,this}}const re=new k,or=new Bt;class ke{constructor(t,e,i=!1){if(Array.isArray(t))throw new TypeError("THREE.BufferAttribute: array should be a Typed Array.");this.isBufferAttribute=!0,this.name="",this.array=t,this.itemSize=e,this.count=t!==void 0?t.length/e:0,this.normalized=i,this.usage=no,this._updateRange={offset:0,count:-1},this.updateRanges=[],this.gpuType=nn,this.version=0}onUploadCallback(){}set needsUpdate(t){t===!0&&this.version++}get updateRange(){return Hi("THREE.BufferAttribute: updateRange() is deprecated and will be removed in r169. Use addUpdateRange() instead."),this._updateRange}setUsage(t){return this.usage=t,this}addUpdateRange(t,e){this.updateRanges.push({start:t,count:e})}clearUpdateRanges(){this.updateRanges.length=0}copy(t){return this.name=t.name,this.array=new t.array.constructor(t.array),this.itemSize=t.itemSize,this.count=t.count,this.normalized=t.normalized,this.usage=t.usage,this.gpuType=t.gpuType,this}copyAt(t,e,i){t*=this.itemSize,i*=e.itemSize;for(let r=0,s=this.itemSize;r<s;r++)this.array[t+r]=e.array[i+r];return this}copyArray(t){return this.array.set(t),this}applyMatrix3(t){if(this.itemSize===2)for(let e=0,i=this.count;e<i;e++)or.fromBufferAttribute(this,e),or.applyMatrix3(t),this.setXY(e,or.x,or.y);else if(this.itemSize===3)for(let e=0,i=this.count;e<i;e++)re.fromBufferAttribute(this,e),re.applyMatrix3(t),this.setXYZ(e,re.x,re.y,re.z);return this}applyMatrix4(t){for(let e=0,i=this.count;e<i;e++)re.fromBufferAttribute(this,e),re.applyMatrix4(t),this.setXYZ(e,re.x,re.y,re.z);return this}applyNormalMatrix(t){for(let e=0,i=this.count;e<i;e++)re.fromBufferAttribute(this,e),re.applyNormalMatrix(t),this.setXYZ(e,re.x,re.y,re.z);return this}transformDirection(t){for(let e=0,i=this.count;e<i;e++)re.fromBufferAttribute(this,e),re.transformDirection(t),this.setXYZ(e,re.x,re.y,re.z);return this}set(t,e=0){return this.array.set(t,e),this}getComponent(t,e){let i=this.array[t*this.itemSize+e];return this.normalized&&(i=Pi(i,this.array)),i}setComponent(t,e,i){return this.normalized&&(i=ye(i,this.array)),this.array[t*this.itemSize+e]=i,this}getX(t){let e=this.array[t*this.itemSize];return this.normalized&&(e=Pi(e,this.array)),e}setX(t,e){return this.normalized&&(e=ye(e,this.array)),this.array[t*this.itemSize]=e,this}getY(t){let e=this.array[t*this.itemSize+1];return this.normalized&&(e=Pi(e,this.array)),e}setY(t,e){return this.normalized&&(e=ye(e,this.array)),this.array[t*this.itemSize+1]=e,this}getZ(t){let e=this.array[t*this.itemSize+2];return this.normalized&&(e=Pi(e,this.array)),e}setZ(t,e){return this.normalized&&(e=ye(e,this.array)),this.array[t*this.itemSize+2]=e,this}getW(t){let e=this.array[t*this.itemSize+3];return this.normalized&&(e=Pi(e,this.array)),e}setW(t,e){return this.normalized&&(e=ye(e,this.array)),this.array[t*this.itemSize+3]=e,this}setXY(t,e,i){return t*=this.itemSize,this.normalized&&(e=ye(e,this.array),i=ye(i,this.array)),this.array[t+0]=e,this.array[t+1]=i,this}setXYZ(t,e,i,r){return t*=this.itemSize,this.normalized&&(e=ye(e,this.array),i=ye(i,this.array),r=ye(r,this.array)),this.array[t+0]=e,this.array[t+1]=i,this.array[t+2]=r,this}setXYZW(t,e,i,r,s){return t*=this.itemSize,this.normalized&&(e=ye(e,this.array),i=ye(i,this.array),r=ye(r,this.array),s=ye(s,this.array)),this.array[t+0]=e,this.array[t+1]=i,this.array[t+2]=r,this.array[t+3]=s,this}onUpload(t){return this.onUploadCallback=t,this}clone(){return new this.constructor(this.array,this.itemSize).copy(this)}toJSON(){const t={itemSize:this.itemSize,type:this.array.constructor.name,array:Array.from(this.array),normalized:this.normalized};return this.name!==""&&(t.name=this.name),this.usage!==no&&(t.usage=this.usage),t}}class Kc extends ke{constructor(t,e,i){super(new Uint16Array(t),e,i)}}class jc extends ke{constructor(t,e,i){super(new Uint32Array(t),e,i)}}class xe extends ke{constructor(t,e,i){super(new Float32Array(t),e,i)}}let ch=0;const Pe=new Qt,Ts=new fe,li=new k,be=new Yi,Ni=new Yi,le=new k;class Ce extends Ai{constructor(){super(),this.isBufferGeometry=!0,Object.defineProperty(this,"id",{value:ch++}),this.uuid=Xi(),this.name="",this.type="BufferGeometry",this.index=null,this.attributes={},this.morphAttributes={},this.morphTargetsRelative=!1,this.groups=[],this.boundingBox=null,this.boundingSphere=null,this.drawRange={start:0,count:1/0},this.userData={}}getIndex(){return this.index}setIndex(t){return Array.isArray(t)?this.index=new(Vc(t)?jc:Kc)(t,1):this.index=t,this}getAttribute(t){return this.attributes[t]}setAttribute(t,e){return this.attributes[t]=e,this}deleteAttribute(t){return delete this.attributes[t],this}hasAttribute(t){return this.attributes[t]!==void 0}addGroup(t,e,i=0){this.groups.push({start:t,count:e,materialIndex:i})}clearGroups(){this.groups=[]}setDrawRange(t,e){this.drawRange.start=t,this.drawRange.count=e}applyMatrix4(t){const e=this.attributes.position;e!==void 0&&(e.applyMatrix4(t),e.needsUpdate=!0);const i=this.attributes.normal;if(i!==void 0){const s=new Nt().getNormalMatrix(t);i.applyNormalMatrix(s),i.needsUpdate=!0}const r=this.attributes.tangent;return r!==void 0&&(r.transformDirection(t),r.needsUpdate=!0),this.boundingBox!==null&&this.computeBoundingBox(),this.boundingSphere!==null&&this.computeBoundingSphere(),this}applyQuaternion(t){return Pe.makeRotationFromQuaternion(t),this.applyMatrix4(Pe),this}rotateX(t){return Pe.makeRotationX(t),this.applyMatrix4(Pe),this}rotateY(t){return Pe.makeRotationY(t),this.applyMatrix4(Pe),this}rotateZ(t){return Pe.makeRotationZ(t),this.applyMatrix4(Pe),this}translate(t,e,i){return Pe.makeTranslation(t,e,i),this.applyMatrix4(Pe),this}scale(t,e,i){return Pe.makeScale(t,e,i),this.applyMatrix4(Pe),this}lookAt(t){return Ts.lookAt(t),Ts.updateMatrix(),this.applyMatrix4(Ts.matrix),this}center(){return this.computeBoundingBox(),this.boundingBox.getCenter(li).negate(),this.translate(li.x,li.y,li.z),this}setFromPoints(t){const e=[];for(let i=0,r=t.length;i<r;i++){const s=t[i];e.push(s.x,s.y,s.z||0)}return this.setAttribute("position",new xe(e,3)),this}computeBoundingBox(){this.boundingBox===null&&(this.boundingBox=new Yi);const t=this.attributes.position,e=this.morphAttributes.position;if(t&&t.isGLBufferAttribute){console.error("THREE.BufferGeometry.computeBoundingBox(): GLBufferAttribute requires a manual bounding box.",this),this.boundingBox.set(new k(-1/0,-1/0,-1/0),new k(1/0,1/0,1/0));return}if(t!==void 0){if(this.boundingBox.setFromBufferAttribute(t),e)for(let i=0,r=e.length;i<r;i++){const s=e[i];be.setFromBufferAttribute(s),this.morphTargetsRelative?(le.addVectors(this.boundingBox.min,be.min),this.boundingBox.expandByPoint(le),le.addVectors(this.boundingBox.max,be.max),this.boundingBox.expandByPoint(le)):(this.boundingBox.expandByPoint(be.min),this.boundingBox.expandByPoint(be.max))}}else this.boundingBox.makeEmpty();(isNaN(this.boundingBox.min.x)||isNaN(this.boundingBox.min.y)||isNaN(this.boundingBox.min.z))&&console.error('THREE.BufferGeometry.computeBoundingBox(): Computed min/max have NaN values. The "position" attribute is likely to have NaN values.',this)}computeBoundingSphere(){this.boundingSphere===null&&(this.boundingSphere=new Yr);const t=this.attributes.position,e=this.morphAttributes.position;if(t&&t.isGLBufferAttribute){console.error("THREE.BufferGeometry.computeBoundingSphere(): GLBufferAttribute requires a manual bounding sphere.",this),this.boundingSphere.set(new k,1/0);return}if(t){const i=this.boundingSphere.center;if(be.setFromBufferAttribute(t),e)for(let s=0,o=e.length;s<o;s++){const a=e[s];Ni.setFromBufferAttribute(a),this.morphTargetsRelative?(le.addVectors(be.min,Ni.min),be.expandByPoint(le),le.addVectors(be.max,Ni.max),be.expandByPoint(le)):(be.expandByPoint(Ni.min),be.expandByPoint(Ni.max))}be.getCenter(i);let r=0;for(let s=0,o=t.count;s<o;s++)le.fromBufferAttribute(t,s),r=Math.max(r,i.distanceToSquared(le));if(e)for(let s=0,o=e.length;s<o;s++){const a=e[s],c=this.morphTargetsRelative;for(let u=0,f=a.count;u<f;u++)le.fromBufferAttribute(a,u),c&&(li.fromBufferAttribute(t,u),le.add(li)),r=Math.max(r,i.distanceToSquared(le))}this.boundingSphere.radius=Math.sqrt(r),isNaN(this.boundingSphere.radius)&&console.error('THREE.BufferGeometry.computeBoundingSphere(): Computed radius is NaN. The "position" attribute is likely to have NaN values.',this)}}computeTangents(){const t=this.index,e=this.attributes;if(t===null||e.position===void 0||e.normal===void 0||e.uv===void 0){console.error("THREE.BufferGeometry: .computeTangents() failed. Missing required attributes (index, position, normal or uv)");return}const i=e.position,r=e.normal,s=e.uv;this.hasAttribute("tangent")===!1&&this.setAttribute("tangent",new ke(new Float32Array(4*i.count),4));const o=this.getAttribute("tangent"),a=[],c=[];for(let F=0;F<i.count;F++)a[F]=new k,c[F]=new k;const u=new k,f=new k,p=new k,m=new Bt,g=new Bt,v=new Bt,M=new k,_=new k;function d(F,A,E){u.fromBufferAttribute(i,F),f.fromBufferAttribute(i,A),p.fromBufferAttribute(i,E),m.fromBufferAttribute(s,F),g.fromBufferAttribute(s,A),v.fromBufferAttribute(s,E),f.sub(u),p.sub(u),g.sub(m),v.sub(m);const N=1/(g.x*v.y-v.x*g.y);isFinite(N)&&(M.copy(f).multiplyScalar(v.y).addScaledVector(p,-g.y).multiplyScalar(N),_.copy(p).multiplyScalar(g.x).addScaledVector(f,-v.x).multiplyScalar(N),a[F].add(M),a[A].add(M),a[E].add(M),c[F].add(_),c[A].add(_),c[E].add(_))}let C=this.groups;C.length===0&&(C=[{start:0,count:t.count}]);for(let F=0,A=C.length;F<A;++F){const E=C[F],N=E.start,$=E.count;for(let W=N,Y=N+$;W<Y;W+=3)d(t.getX(W+0),t.getX(W+1),t.getX(W+2))}const w=new k,b=new k,z=new k,P=new k;function I(F){z.fromBufferAttribute(r,F),P.copy(z);const A=a[F];w.copy(A),w.sub(z.multiplyScalar(z.dot(A))).normalize(),b.crossVectors(P,A);const N=b.dot(c[F])<0?-1:1;o.setXYZW(F,w.x,w.y,w.z,N)}for(let F=0,A=C.length;F<A;++F){const E=C[F],N=E.start,$=E.count;for(let W=N,Y=N+$;W<Y;W+=3)I(t.getX(W+0)),I(t.getX(W+1)),I(t.getX(W+2))}}computeVertexNormals(){const t=this.index,e=this.getAttribute("position");if(e!==void 0){let i=this.getAttribute("normal");if(i===void 0)i=new ke(new Float32Array(e.count*3),3),this.setAttribute("normal",i);else for(let m=0,g=i.count;m<g;m++)i.setXYZ(m,0,0,0);const r=new k,s=new k,o=new k,a=new k,c=new k,u=new k,f=new k,p=new k;if(t)for(let m=0,g=t.count;m<g;m+=3){const v=t.getX(m+0),M=t.getX(m+1),_=t.getX(m+2);r.fromBufferAttribute(e,v),s.fromBufferAttribute(e,M),o.fromBufferAttribute(e,_),f.subVectors(o,s),p.subVectors(r,s),f.cross(p),a.fromBufferAttribute(i,v),c.fromBufferAttribute(i,M),u.fromBufferAttribute(i,_),a.add(f),c.add(f),u.add(f),i.setXYZ(v,a.x,a.y,a.z),i.setXYZ(M,c.x,c.y,c.z),i.setXYZ(_,u.x,u.y,u.z)}else for(let m=0,g=e.count;m<g;m+=3)r.fromBufferAttribute(e,m+0),s.fromBufferAttribute(e,m+1),o.fromBufferAttribute(e,m+2),f.subVectors(o,s),p.subVectors(r,s),f.cross(p),i.setXYZ(m+0,f.x,f.y,f.z),i.setXYZ(m+1,f.x,f.y,f.z),i.setXYZ(m+2,f.x,f.y,f.z);this.normalizeNormals(),i.needsUpdate=!0}}normalizeNormals(){const t=this.attributes.normal;for(let e=0,i=t.count;e<i;e++)le.fromBufferAttribute(t,e),le.normalize(),t.setXYZ(e,le.x,le.y,le.z)}toNonIndexed(){function t(a,c){const u=a.array,f=a.itemSize,p=a.normalized,m=new u.constructor(c.length*f);let g=0,v=0;for(let M=0,_=c.length;M<_;M++){a.isInterleavedBufferAttribute?g=c[M]*a.data.stride+a.offset:g=c[M]*f;for(let d=0;d<f;d++)m[v++]=u[g++]}return new ke(m,f,p)}if(this.index===null)return console.warn("THREE.BufferGeometry.toNonIndexed(): BufferGeometry is already non-indexed."),this;const e=new Ce,i=this.index.array,r=this.attributes;for(const a in r){const c=r[a],u=t(c,i);e.setAttribute(a,u)}const s=this.morphAttributes;for(const a in s){const c=[],u=s[a];for(let f=0,p=u.length;f<p;f++){const m=u[f],g=t(m,i);c.push(g)}e.morphAttributes[a]=c}e.morphTargetsRelative=this.morphTargetsRelative;const o=this.groups;for(let a=0,c=o.length;a<c;a++){const u=o[a];e.addGroup(u.start,u.count,u.materialIndex)}return e}toJSON(){const t={metadata:{version:4.6,type:"BufferGeometry",generator:"BufferGeometry.toJSON"}};if(t.uuid=this.uuid,t.type=this.type,this.name!==""&&(t.name=this.name),Object.keys(this.userData).length>0&&(t.userData=this.userData),this.parameters!==void 0){const c=this.parameters;for(const u in c)c[u]!==void 0&&(t[u]=c[u]);return t}t.data={attributes:{}};const e=this.index;e!==null&&(t.data.index={type:e.array.constructor.name,array:Array.prototype.slice.call(e.array)});const i=this.attributes;for(const c in i){const u=i[c];t.data.attributes[c]=u.toJSON(t.data)}const r={};let s=!1;for(const c in this.morphAttributes){const u=this.morphAttributes[c],f=[];for(let p=0,m=u.length;p<m;p++){const g=u[p];f.push(g.toJSON(t.data))}f.length>0&&(r[c]=f,s=!0)}s&&(t.data.morphAttributes=r,t.data.morphTargetsRelative=this.morphTargetsRelative);const o=this.groups;o.length>0&&(t.data.groups=JSON.parse(JSON.stringify(o)));const a=this.boundingSphere;return a!==null&&(t.data.boundingSphere={center:a.center.toArray(),radius:a.radius}),t}clone(){return new this.constructor().copy(this)}copy(t){this.index=null,this.attributes={},this.morphAttributes={},this.groups=[],this.boundingBox=null,this.boundingSphere=null;const e={};this.name=t.name;const i=t.index;i!==null&&this.setIndex(i.clone(e));const r=t.attributes;for(const u in r){const f=r[u];this.setAttribute(u,f.clone(e))}const s=t.morphAttributes;for(const u in s){const f=[],p=s[u];for(let m=0,g=p.length;m<g;m++)f.push(p[m].clone(e));this.morphAttributes[u]=f}this.morphTargetsRelative=t.morphTargetsRelative;const o=t.groups;for(let u=0,f=o.length;u<f;u++){const p=o[u];this.addGroup(p.start,p.count,p.materialIndex)}const a=t.boundingBox;a!==null&&(this.boundingBox=a.clone());const c=t.boundingSphere;return c!==null&&(this.boundingSphere=c.clone()),this.drawRange.start=t.drawRange.start,this.drawRange.count=t.drawRange.count,this.userData=t.userData,this}dispose(){this.dispatchEvent({type:"dispose"})}}const go=new Qt,Dn=new Xc,cr=new Yr,xo=new k,ui=new k,hi=new k,fi=new k,ws=new k,lr=new k,ur=new Bt,hr=new Bt,fr=new Bt,vo=new k,yo=new k,Mo=new k,dr=new k,pr=new k;class ae extends fe{constructor(t=new Ce,e=new sn){super(),this.isMesh=!0,this.type="Mesh",this.geometry=t,this.material=e,this.updateMorphTargets()}copy(t,e){return super.copy(t,e),t.morphTargetInfluences!==void 0&&(this.morphTargetInfluences=t.morphTargetInfluences.slice()),t.morphTargetDictionary!==void 0&&(this.morphTargetDictionary=Object.assign({},t.morphTargetDictionary)),this.material=Array.isArray(t.material)?t.material.slice():t.material,this.geometry=t.geometry,this}updateMorphTargets(){const e=this.geometry.morphAttributes,i=Object.keys(e);if(i.length>0){const r=e[i[0]];if(r!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let s=0,o=r.length;s<o;s++){const a=r[s].name||String(s);this.morphTargetInfluences.push(0),this.morphTargetDictionary[a]=s}}}}getVertexPosition(t,e){const i=this.geometry,r=i.attributes.position,s=i.morphAttributes.position,o=i.morphTargetsRelative;e.fromBufferAttribute(r,t);const a=this.morphTargetInfluences;if(s&&a){lr.set(0,0,0);for(let c=0,u=s.length;c<u;c++){const f=a[c],p=s[c];f!==0&&(ws.fromBufferAttribute(p,t),o?lr.addScaledVector(ws,f):lr.addScaledVector(ws.sub(e),f))}e.add(lr)}return e}raycast(t,e){const i=this.geometry,r=this.material,s=this.matrixWorld;r!==void 0&&(i.boundingSphere===null&&i.computeBoundingSphere(),cr.copy(i.boundingSphere),cr.applyMatrix4(s),Dn.copy(t.ray).recast(t.near),!(cr.containsPoint(Dn.origin)===!1&&(Dn.intersectSphere(cr,xo)===null||Dn.origin.distanceToSquared(xo)>(t.far-t.near)**2))&&(go.copy(s).invert(),Dn.copy(t.ray).applyMatrix4(go),!(i.boundingBox!==null&&Dn.intersectsBox(i.boundingBox)===!1)&&this._computeIntersections(t,e,Dn)))}_computeIntersections(t,e,i){let r;const s=this.geometry,o=this.material,a=s.index,c=s.attributes.position,u=s.attributes.uv,f=s.attributes.uv1,p=s.attributes.normal,m=s.groups,g=s.drawRange;if(a!==null)if(Array.isArray(o))for(let v=0,M=m.length;v<M;v++){const _=m[v],d=o[_.materialIndex],C=Math.max(_.start,g.start),w=Math.min(a.count,Math.min(_.start+_.count,g.start+g.count));for(let b=C,z=w;b<z;b+=3){const P=a.getX(b),I=a.getX(b+1),F=a.getX(b+2);r=mr(this,d,t,i,u,f,p,P,I,F),r&&(r.faceIndex=Math.floor(b/3),r.face.materialIndex=_.materialIndex,e.push(r))}}else{const v=Math.max(0,g.start),M=Math.min(a.count,g.start+g.count);for(let _=v,d=M;_<d;_+=3){const C=a.getX(_),w=a.getX(_+1),b=a.getX(_+2);r=mr(this,o,t,i,u,f,p,C,w,b),r&&(r.faceIndex=Math.floor(_/3),e.push(r))}}else if(c!==void 0)if(Array.isArray(o))for(let v=0,M=m.length;v<M;v++){const _=m[v],d=o[_.materialIndex],C=Math.max(_.start,g.start),w=Math.min(c.count,Math.min(_.start+_.count,g.start+g.count));for(let b=C,z=w;b<z;b+=3){const P=b,I=b+1,F=b+2;r=mr(this,d,t,i,u,f,p,P,I,F),r&&(r.faceIndex=Math.floor(b/3),r.face.materialIndex=_.materialIndex,e.push(r))}}else{const v=Math.max(0,g.start),M=Math.min(c.count,g.start+g.count);for(let _=v,d=M;_<d;_+=3){const C=_,w=_+1,b=_+2;r=mr(this,o,t,i,u,f,p,C,w,b),r&&(r.faceIndex=Math.floor(_/3),e.push(r))}}}}function lh(n,t,e,i,r,s,o,a){let c;if(t.side===Se?c=i.intersectTriangle(o,s,r,!0,a):c=i.intersectTriangle(r,s,o,t.side===An,a),c===null)return null;pr.copy(a),pr.applyMatrix4(n.matrixWorld);const u=e.ray.origin.distanceTo(pr);return u<e.near||u>e.far?null:{distance:u,point:pr.clone(),object:n}}function mr(n,t,e,i,r,s,o,a,c,u){n.getVertexPosition(a,ui),n.getVertexPosition(c,hi),n.getVertexPosition(u,fi);const f=lh(n,t,e,i,ui,hi,fi,dr);if(f){r&&(ur.fromBufferAttribute(r,a),hr.fromBufferAttribute(r,c),fr.fromBufferAttribute(r,u),f.uv=$e.getInterpolation(dr,ui,hi,fi,ur,hr,fr,new Bt)),s&&(ur.fromBufferAttribute(s,a),hr.fromBufferAttribute(s,c),fr.fromBufferAttribute(s,u),f.uv1=$e.getInterpolation(dr,ui,hi,fi,ur,hr,fr,new Bt)),o&&(vo.fromBufferAttribute(o,a),yo.fromBufferAttribute(o,c),Mo.fromBufferAttribute(o,u),f.normal=$e.getInterpolation(dr,ui,hi,fi,vo,yo,Mo,new k),f.normal.dot(i.direction)>0&&f.normal.multiplyScalar(-1));const p={a,b:c,c:u,normal:new k,materialIndex:0};$e.getNormal(ui,hi,fi,p.normal),f.face=p}return f}class Xn extends Ce{constructor(t=1,e=1,i=1,r=1,s=1,o=1){super(),this.type="BoxGeometry",this.parameters={width:t,height:e,depth:i,widthSegments:r,heightSegments:s,depthSegments:o};const a=this;r=Math.floor(r),s=Math.floor(s),o=Math.floor(o);const c=[],u=[],f=[],p=[];let m=0,g=0;v("z","y","x",-1,-1,i,e,t,o,s,0),v("z","y","x",1,-1,i,e,-t,o,s,1),v("x","z","y",1,1,t,i,e,r,o,2),v("x","z","y",1,-1,t,i,-e,r,o,3),v("x","y","z",1,-1,t,e,i,r,s,4),v("x","y","z",-1,-1,t,e,-i,r,s,5),this.setIndex(c),this.setAttribute("position",new xe(u,3)),this.setAttribute("normal",new xe(f,3)),this.setAttribute("uv",new xe(p,2));function v(M,_,d,C,w,b,z,P,I,F,A){const E=b/I,N=z/F,$=b/2,W=z/2,Y=P/2,tt=I+1,K=F+1;let st=0,j=0;const dt=new k;for(let _t=0;_t<K;_t++){const ut=_t*N-W;for(let wt=0;wt<tt;wt++){const kt=wt*E-$;dt[M]=kt*C,dt[_]=ut*w,dt[d]=Y,u.push(dt.x,dt.y,dt.z),dt[M]=0,dt[_]=0,dt[d]=P>0?1:-1,f.push(dt.x,dt.y,dt.z),p.push(wt/I),p.push(1-_t/F),st+=1}}for(let _t=0;_t<F;_t++)for(let ut=0;ut<I;ut++){const wt=m+ut+tt*_t,kt=m+ut+tt*(_t+1),Z=m+(ut+1)+tt*(_t+1),at=m+(ut+1)+tt*_t;c.push(wt,kt,at),c.push(kt,Z,at),j+=6}a.addGroup(g,j,A),g+=j,m+=st}}copy(t){return super.copy(t),this.parameters=Object.assign({},t.parameters),this}static fromJSON(t){return new Xn(t.width,t.height,t.depth,t.widthSegments,t.heightSegments,t.depthSegments)}}function Ti(n){const t={};for(const e in n){t[e]={};for(const i in n[e]){const r=n[e][i];r&&(r.isColor||r.isMatrix3||r.isMatrix4||r.isVector2||r.isVector3||r.isVector4||r.isTexture||r.isQuaternion)?r.isRenderTargetTexture?(console.warn("UniformsUtils: Textures of render targets cannot be cloned via cloneUniforms() or mergeUniforms()."),t[e][i]=null):t[e][i]=r.clone():Array.isArray(r)?t[e][i]=r.slice():t[e][i]=r}}return t}function ve(n){const t={};for(let e=0;e<n.length;e++){const i=Ti(n[e]);for(const r in i)t[r]=i[r]}return t}function uh(n){const t=[];for(let e=0;e<n.length;e++)t.push(n[e].clone());return t}function Zc(n){const t=n.getRenderTarget();return t===null?n.outputColorSpace:t.isXRRenderTarget===!0?t.texture.colorSpace:$t.workingColorSpace}const hh={clone:Ti,merge:ve};var fh=`void main() {
	gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
}`,dh=`void main() {
	gl_FragColor = vec4( 1.0, 0.0, 0.0, 1.0 );
}`;class bn extends Kn{constructor(t){super(),this.isShaderMaterial=!0,this.type="ShaderMaterial",this.defines={},this.uniforms={},this.uniformsGroups=[],this.vertexShader=fh,this.fragmentShader=dh,this.linewidth=1,this.wireframe=!1,this.wireframeLinewidth=1,this.fog=!1,this.lights=!1,this.clipping=!1,this.forceSinglePass=!0,this.extensions={clipCullDistance:!1,multiDraw:!1},this.defaultAttributeValues={color:[1,1,1],uv:[0,0],uv1:[0,0]},this.index0AttributeName=void 0,this.uniformsNeedUpdate=!1,this.glslVersion=null,t!==void 0&&this.setValues(t)}copy(t){return super.copy(t),this.fragmentShader=t.fragmentShader,this.vertexShader=t.vertexShader,this.uniforms=Ti(t.uniforms),this.uniformsGroups=uh(t.uniformsGroups),this.defines=Object.assign({},t.defines),this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this.fog=t.fog,this.lights=t.lights,this.clipping=t.clipping,this.extensions=Object.assign({},t.extensions),this.glslVersion=t.glslVersion,this}toJSON(t){const e=super.toJSON(t);e.glslVersion=this.glslVersion,e.uniforms={};for(const r in this.uniforms){const o=this.uniforms[r].value;o&&o.isTexture?e.uniforms[r]={type:"t",value:o.toJSON(t).uuid}:o&&o.isColor?e.uniforms[r]={type:"c",value:o.getHex()}:o&&o.isVector2?e.uniforms[r]={type:"v2",value:o.toArray()}:o&&o.isVector3?e.uniforms[r]={type:"v3",value:o.toArray()}:o&&o.isVector4?e.uniforms[r]={type:"v4",value:o.toArray()}:o&&o.isMatrix3?e.uniforms[r]={type:"m3",value:o.toArray()}:o&&o.isMatrix4?e.uniforms[r]={type:"m4",value:o.toArray()}:e.uniforms[r]={value:o}}Object.keys(this.defines).length>0&&(e.defines=this.defines),e.vertexShader=this.vertexShader,e.fragmentShader=this.fragmentShader,e.lights=this.lights,e.clipping=this.clipping;const i={};for(const r in this.extensions)this.extensions[r]===!0&&(i[r]=!0);return Object.keys(i).length>0&&(e.extensions=i),e}}class Jc extends fe{constructor(){super(),this.isCamera=!0,this.type="Camera",this.matrixWorldInverse=new Qt,this.projectionMatrix=new Qt,this.projectionMatrixInverse=new Qt,this.coordinateSystem=rn}copy(t,e){return super.copy(t,e),this.matrixWorldInverse.copy(t.matrixWorldInverse),this.projectionMatrix.copy(t.projectionMatrix),this.projectionMatrixInverse.copy(t.projectionMatrixInverse),this.coordinateSystem=t.coordinateSystem,this}getWorldDirection(t){return super.getWorldDirection(t).negate()}updateMatrixWorld(t){super.updateMatrixWorld(t),this.matrixWorldInverse.copy(this.matrixWorld).invert()}updateWorldMatrix(t,e){super.updateWorldMatrix(t,e),this.matrixWorldInverse.copy(this.matrixWorld).invert()}clone(){return new this.constructor().copy(this)}}const yn=new k,So=new Bt,Eo=new Bt;class Ie extends Jc{constructor(t=50,e=1,i=.1,r=2e3){super(),this.isPerspectiveCamera=!0,this.type="PerspectiveCamera",this.fov=t,this.zoom=1,this.near=i,this.far=r,this.focus=10,this.aspect=e,this.view=null,this.filmGauge=35,this.filmOffset=0,this.updateProjectionMatrix()}copy(t,e){return super.copy(t,e),this.fov=t.fov,this.zoom=t.zoom,this.near=t.near,this.far=t.far,this.focus=t.focus,this.aspect=t.aspect,this.view=t.view===null?null:Object.assign({},t.view),this.filmGauge=t.filmGauge,this.filmOffset=t.filmOffset,this}setFocalLength(t){const e=.5*this.getFilmHeight()/t;this.fov=va*2*Math.atan(e),this.updateProjectionMatrix()}getFocalLength(){const t=Math.tan(os*.5*this.fov);return .5*this.getFilmHeight()/t}getEffectiveFOV(){return va*2*Math.atan(Math.tan(os*.5*this.fov)/this.zoom)}getFilmWidth(){return this.filmGauge*Math.min(this.aspect,1)}getFilmHeight(){return this.filmGauge/Math.max(this.aspect,1)}getViewBounds(t,e,i){yn.set(-1,-1,.5).applyMatrix4(this.projectionMatrixInverse),e.set(yn.x,yn.y).multiplyScalar(-t/yn.z),yn.set(1,1,.5).applyMatrix4(this.projectionMatrixInverse),i.set(yn.x,yn.y).multiplyScalar(-t/yn.z)}getViewSize(t,e){return this.getViewBounds(t,So,Eo),e.subVectors(Eo,So)}setViewOffset(t,e,i,r,s,o){this.aspect=t/e,this.view===null&&(this.view={enabled:!0,fullWidth:1,fullHeight:1,offsetX:0,offsetY:0,width:1,height:1}),this.view.enabled=!0,this.view.fullWidth=t,this.view.fullHeight=e,this.view.offsetX=i,this.view.offsetY=r,this.view.width=s,this.view.height=o,this.updateProjectionMatrix()}clearViewOffset(){this.view!==null&&(this.view.enabled=!1),this.updateProjectionMatrix()}updateProjectionMatrix(){const t=this.near;let e=t*Math.tan(os*.5*this.fov)/this.zoom,i=2*e,r=this.aspect*i,s=-.5*r;const o=this.view;if(this.view!==null&&this.view.enabled){const c=o.fullWidth,u=o.fullHeight;s+=o.offsetX*r/c,e-=o.offsetY*i/u,r*=o.width/c,i*=o.height/u}const a=this.filmOffset;a!==0&&(s+=t*a/this.getFilmWidth()),this.projectionMatrix.makePerspective(s,s+r,e,e-i,t,this.far,this.coordinateSystem),this.projectionMatrixInverse.copy(this.projectionMatrix).invert()}toJSON(t){const e=super.toJSON(t);return e.object.fov=this.fov,e.object.zoom=this.zoom,e.object.near=this.near,e.object.far=this.far,e.object.focus=this.focus,e.object.aspect=this.aspect,this.view!==null&&(e.object.view=Object.assign({},this.view)),e.object.filmGauge=this.filmGauge,e.object.filmOffset=this.filmOffset,e}}const di=-90,pi=1;class ph extends fe{constructor(t,e,i){super(),this.type="CubeCamera",this.renderTarget=i,this.coordinateSystem=null,this.activeMipmapLevel=0;const r=new Ie(di,pi,t,e);r.layers=this.layers,this.add(r);const s=new Ie(di,pi,t,e);s.layers=this.layers,this.add(s);const o=new Ie(di,pi,t,e);o.layers=this.layers,this.add(o);const a=new Ie(di,pi,t,e);a.layers=this.layers,this.add(a);const c=new Ie(di,pi,t,e);c.layers=this.layers,this.add(c);const u=new Ie(di,pi,t,e);u.layers=this.layers,this.add(u)}updateCoordinateSystem(){const t=this.coordinateSystem,e=this.children.concat(),[i,r,s,o,a,c]=e;for(const u of e)this.remove(u);if(t===rn)i.up.set(0,1,0),i.lookAt(1,0,0),r.up.set(0,1,0),r.lookAt(-1,0,0),s.up.set(0,0,-1),s.lookAt(0,1,0),o.up.set(0,0,1),o.lookAt(0,-1,0),a.up.set(0,1,0),a.lookAt(0,0,1),c.up.set(0,1,0),c.lookAt(0,0,-1);else if(t===zr)i.up.set(0,-1,0),i.lookAt(-1,0,0),r.up.set(0,-1,0),r.lookAt(1,0,0),s.up.set(0,0,1),s.lookAt(0,1,0),o.up.set(0,0,-1),o.lookAt(0,-1,0),a.up.set(0,-1,0),a.lookAt(0,0,1),c.up.set(0,-1,0),c.lookAt(0,0,-1);else throw new Error("THREE.CubeCamera.updateCoordinateSystem(): Invalid coordinate system: "+t);for(const u of e)this.add(u),u.updateMatrixWorld()}update(t,e){this.parent===null&&this.updateMatrixWorld();const{renderTarget:i,activeMipmapLevel:r}=this;this.coordinateSystem!==t.coordinateSystem&&(this.coordinateSystem=t.coordinateSystem,this.updateCoordinateSystem());const[s,o,a,c,u,f]=this.children,p=t.getRenderTarget(),m=t.getActiveCubeFace(),g=t.getActiveMipmapLevel(),v=t.xr.enabled;t.xr.enabled=!1;const M=i.texture.generateMipmaps;i.texture.generateMipmaps=!1,t.setRenderTarget(i,0,r),t.render(e,s),t.setRenderTarget(i,1,r),t.render(e,o),t.setRenderTarget(i,2,r),t.render(e,a),t.setRenderTarget(i,3,r),t.render(e,c),t.setRenderTarget(i,4,r),t.render(e,u),i.texture.generateMipmaps=M,t.setRenderTarget(i,5,r),t.render(e,f),t.setRenderTarget(p,m,g),t.xr.enabled=v,i.texture.needsPMREMUpdate=!0}}class Qc extends Ee{constructor(t,e,i,r,s,o,a,c,u,f){t=t!==void 0?t:[],e=e!==void 0?e:yi,super(t,e,i,r,s,o,a,c,u,f),this.isCubeTexture=!0,this.flipY=!1}get images(){return this.image}set images(t){this.image=t}}class mh extends qn{constructor(t=1,e={}){super(t,t,e),this.isWebGLCubeRenderTarget=!0;const i={width:t,height:t,depth:1},r=[i,i,i,i,i,i];this.texture=new Qc(r,e.mapping,e.wrapS,e.wrapT,e.magFilter,e.minFilter,e.format,e.type,e.anisotropy,e.colorSpace),this.texture.isRenderTargetTexture=!0,this.texture.generateMipmaps=e.generateMipmaps!==void 0?e.generateMipmaps:!1,this.texture.minFilter=e.minFilter!==void 0?e.minFilter:Be}fromEquirectangularTexture(t,e){this.texture.type=e.type,this.texture.colorSpace=e.colorSpace,this.texture.generateMipmaps=e.generateMipmaps,this.texture.minFilter=e.minFilter,this.texture.magFilter=e.magFilter;const i={uniforms:{tEquirect:{value:null}},vertexShader:`

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
			`},r=new Xn(5,5,5),s=new bn({name:"CubemapFromEquirect",uniforms:Ti(i.uniforms),vertexShader:i.vertexShader,fragmentShader:i.fragmentShader,side:Se,blending:Tn});s.uniforms.tEquirect.value=e;const o=new ae(r,s),a=e.minFilter;return e.minFilter===Vn&&(e.minFilter=Be),new ph(1,10,this).update(t,o),e.minFilter=a,o.geometry.dispose(),o.material.dispose(),this}clear(t,e,i,r){const s=t.getRenderTarget();for(let o=0;o<6;o++)t.setRenderTarget(this,o),t.clear(e,i,r);t.setRenderTarget(s)}}const As=new k,_h=new k,gh=new Nt;class On{constructor(t=new k(1,0,0),e=0){this.isPlane=!0,this.normal=t,this.constant=e}set(t,e){return this.normal.copy(t),this.constant=e,this}setComponents(t,e,i,r){return this.normal.set(t,e,i),this.constant=r,this}setFromNormalAndCoplanarPoint(t,e){return this.normal.copy(t),this.constant=-e.dot(this.normal),this}setFromCoplanarPoints(t,e,i){const r=As.subVectors(i,e).cross(_h.subVectors(t,e)).normalize();return this.setFromNormalAndCoplanarPoint(r,t),this}copy(t){return this.normal.copy(t.normal),this.constant=t.constant,this}normalize(){const t=1/this.normal.length();return this.normal.multiplyScalar(t),this.constant*=t,this}negate(){return this.constant*=-1,this.normal.negate(),this}distanceToPoint(t){return this.normal.dot(t)+this.constant}distanceToSphere(t){return this.distanceToPoint(t.center)-t.radius}projectPoint(t,e){return e.copy(t).addScaledVector(this.normal,-this.distanceToPoint(t))}intersectLine(t,e){const i=t.delta(As),r=this.normal.dot(i);if(r===0)return this.distanceToPoint(t.start)===0?e.copy(t.start):null;const s=-(t.start.dot(this.normal)+this.constant)/r;return s<0||s>1?null:e.copy(t.start).addScaledVector(i,s)}intersectsLine(t){const e=this.distanceToPoint(t.start),i=this.distanceToPoint(t.end);return e<0&&i>0||i<0&&e>0}intersectsBox(t){return t.intersectsPlane(this)}intersectsSphere(t){return t.intersectsPlane(this)}coplanarPoint(t){return t.copy(this.normal).multiplyScalar(-this.constant)}applyMatrix4(t,e){const i=e||gh.getNormalMatrix(t),r=this.coplanarPoint(As).applyMatrix4(t),s=this.normal.applyMatrix3(i).normalize();return this.constant=-r.dot(s),this}translate(t){return this.constant-=t.dot(this.normal),this}equals(t){return t.normal.equals(this.normal)&&t.constant===this.constant}clone(){return new this.constructor().copy(this)}}const Un=new Yr,_r=new k;class Fa{constructor(t=new On,e=new On,i=new On,r=new On,s=new On,o=new On){this.planes=[t,e,i,r,s,o]}set(t,e,i,r,s,o){const a=this.planes;return a[0].copy(t),a[1].copy(e),a[2].copy(i),a[3].copy(r),a[4].copy(s),a[5].copy(o),this}copy(t){const e=this.planes;for(let i=0;i<6;i++)e[i].copy(t.planes[i]);return this}setFromProjectionMatrix(t,e=rn){const i=this.planes,r=t.elements,s=r[0],o=r[1],a=r[2],c=r[3],u=r[4],f=r[5],p=r[6],m=r[7],g=r[8],v=r[9],M=r[10],_=r[11],d=r[12],C=r[13],w=r[14],b=r[15];if(i[0].setComponents(c-s,m-u,_-g,b-d).normalize(),i[1].setComponents(c+s,m+u,_+g,b+d).normalize(),i[2].setComponents(c+o,m+f,_+v,b+C).normalize(),i[3].setComponents(c-o,m-f,_-v,b-C).normalize(),i[4].setComponents(c-a,m-p,_-M,b-w).normalize(),e===rn)i[5].setComponents(c+a,m+p,_+M,b+w).normalize();else if(e===zr)i[5].setComponents(a,p,M,w).normalize();else throw new Error("THREE.Frustum.setFromProjectionMatrix(): Invalid coordinate system: "+e);return this}intersectsObject(t){if(t.boundingSphere!==void 0)t.boundingSphere===null&&t.computeBoundingSphere(),Un.copy(t.boundingSphere).applyMatrix4(t.matrixWorld);else{const e=t.geometry;e.boundingSphere===null&&e.computeBoundingSphere(),Un.copy(e.boundingSphere).applyMatrix4(t.matrixWorld)}return this.intersectsSphere(Un)}intersectsSprite(t){return Un.center.set(0,0,0),Un.radius=.7071067811865476,Un.applyMatrix4(t.matrixWorld),this.intersectsSphere(Un)}intersectsSphere(t){const e=this.planes,i=t.center,r=-t.radius;for(let s=0;s<6;s++)if(e[s].distanceToPoint(i)<r)return!1;return!0}intersectsBox(t){const e=this.planes;for(let i=0;i<6;i++){const r=e[i];if(_r.x=r.normal.x>0?t.max.x:t.min.x,_r.y=r.normal.y>0?t.max.y:t.min.y,_r.z=r.normal.z>0?t.max.z:t.min.z,r.distanceToPoint(_r)<0)return!1}return!0}containsPoint(t){const e=this.planes;for(let i=0;i<6;i++)if(e[i].distanceToPoint(t)<0)return!1;return!0}clone(){return new this.constructor().copy(this)}}function tl(){let n=null,t=!1,e=null,i=null;function r(s,o){e(s,o),i=n.requestAnimationFrame(r)}return{start:function(){t!==!0&&e!==null&&(i=n.requestAnimationFrame(r),t=!0)},stop:function(){n.cancelAnimationFrame(i),t=!1},setAnimationLoop:function(s){e=s},setContext:function(s){n=s}}}function xh(n){const t=new WeakMap;function e(a,c){const u=a.array,f=a.usage,p=u.byteLength,m=n.createBuffer();n.bindBuffer(c,m),n.bufferData(c,u,f),a.onUploadCallback();let g;if(u instanceof Float32Array)g=n.FLOAT;else if(u instanceof Uint16Array)a.isFloat16BufferAttribute?g=n.HALF_FLOAT:g=n.UNSIGNED_SHORT;else if(u instanceof Int16Array)g=n.SHORT;else if(u instanceof Uint32Array)g=n.UNSIGNED_INT;else if(u instanceof Int32Array)g=n.INT;else if(u instanceof Int8Array)g=n.BYTE;else if(u instanceof Uint8Array)g=n.UNSIGNED_BYTE;else if(u instanceof Uint8ClampedArray)g=n.UNSIGNED_BYTE;else throw new Error("THREE.WebGLAttributes: Unsupported buffer data format: "+u);return{buffer:m,type:g,bytesPerElement:u.BYTES_PER_ELEMENT,version:a.version,size:p}}function i(a,c,u){const f=c.array,p=c._updateRange,m=c.updateRanges;if(n.bindBuffer(u,a),p.count===-1&&m.length===0&&n.bufferSubData(u,0,f),m.length!==0){for(let g=0,v=m.length;g<v;g++){const M=m[g];n.bufferSubData(u,M.start*f.BYTES_PER_ELEMENT,f,M.start,M.count)}c.clearUpdateRanges()}p.count!==-1&&(n.bufferSubData(u,p.offset*f.BYTES_PER_ELEMENT,f,p.offset,p.count),p.count=-1),c.onUploadCallback()}function r(a){return a.isInterleavedBufferAttribute&&(a=a.data),t.get(a)}function s(a){a.isInterleavedBufferAttribute&&(a=a.data);const c=t.get(a);c&&(n.deleteBuffer(c.buffer),t.delete(a))}function o(a,c){if(a.isInterleavedBufferAttribute&&(a=a.data),a.isGLBufferAttribute){const f=t.get(a);(!f||f.version<a.version)&&t.set(a,{buffer:a.buffer,type:a.type,bytesPerElement:a.elementSize,version:a.version});return}const u=t.get(a);if(u===void 0)t.set(a,e(a,c));else if(u.version<a.version){if(u.size!==a.array.byteLength)throw new Error("THREE.WebGLAttributes: The size of the buffer attribute's array buffer does not match the original size. Resizing buffer attributes is not supported.");i(u.buffer,a,c),u.version=a.version}}return{get:r,remove:s,update:o}}class Ki extends Ce{constructor(t=1,e=1,i=1,r=1){super(),this.type="PlaneGeometry",this.parameters={width:t,height:e,widthSegments:i,heightSegments:r};const s=t/2,o=e/2,a=Math.floor(i),c=Math.floor(r),u=a+1,f=c+1,p=t/a,m=e/c,g=[],v=[],M=[],_=[];for(let d=0;d<f;d++){const C=d*m-o;for(let w=0;w<u;w++){const b=w*p-s;v.push(b,-C,0),M.push(0,0,1),_.push(w/a),_.push(1-d/c)}}for(let d=0;d<c;d++)for(let C=0;C<a;C++){const w=C+u*d,b=C+u*(d+1),z=C+1+u*(d+1),P=C+1+u*d;g.push(w,b,P),g.push(b,z,P)}this.setIndex(g),this.setAttribute("position",new xe(v,3)),this.setAttribute("normal",new xe(M,3)),this.setAttribute("uv",new xe(_,2))}copy(t){return super.copy(t),this.parameters=Object.assign({},t.parameters),this}static fromJSON(t){return new Ki(t.width,t.height,t.widthSegments,t.heightSegments)}}var vh=`#ifdef USE_ALPHAHASH
	if ( diffuseColor.a < getAlphaHashThreshold( vPosition ) ) discard;
#endif`,yh=`#ifdef USE_ALPHAHASH
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
#endif`,Mh=`#ifdef USE_ALPHAMAP
	diffuseColor.a *= texture2D( alphaMap, vAlphaMapUv ).g;
#endif`,Sh=`#ifdef USE_ALPHAMAP
	uniform sampler2D alphaMap;
#endif`,Eh=`#ifdef USE_ALPHATEST
	#ifdef ALPHA_TO_COVERAGE
	diffuseColor.a = smoothstep( alphaTest, alphaTest + fwidth( diffuseColor.a ), diffuseColor.a );
	if ( diffuseColor.a == 0.0 ) discard;
	#else
	if ( diffuseColor.a < alphaTest ) discard;
	#endif
#endif`,Th=`#ifdef USE_ALPHATEST
	uniform float alphaTest;
#endif`,wh=`#ifdef USE_AOMAP
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
#endif`,Ah=`#ifdef USE_AOMAP
	uniform sampler2D aoMap;
	uniform float aoMapIntensity;
#endif`,bh=`#ifdef USE_BATCHING
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
#endif`,Ch=`#ifdef USE_BATCHING
	mat4 batchingMatrix = getBatchingMatrix( getIndirectIndex( gl_DrawID ) );
#endif`,Rh=`vec3 transformed = vec3( position );
#ifdef USE_ALPHAHASH
	vPosition = vec3( position );
#endif`,Ph=`vec3 objectNormal = vec3( normal );
#ifdef USE_TANGENT
	vec3 objectTangent = vec3( tangent.xyz );
#endif`,Lh=`float G_BlinnPhong_Implicit( ) {
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
} // validated`,Ih=`#ifdef USE_IRIDESCENCE
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
#endif`,Dh=`#ifdef USE_BUMPMAP
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
#endif`,Uh=`#if NUM_CLIPPING_PLANES > 0
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
#endif`,Nh=`#if NUM_CLIPPING_PLANES > 0
	varying vec3 vClipPosition;
	uniform vec4 clippingPlanes[ NUM_CLIPPING_PLANES ];
#endif`,Fh=`#if NUM_CLIPPING_PLANES > 0
	varying vec3 vClipPosition;
#endif`,Oh=`#if NUM_CLIPPING_PLANES > 0
	vClipPosition = - mvPosition.xyz;
#endif`,Bh=`#if defined( USE_COLOR_ALPHA )
	diffuseColor *= vColor;
#elif defined( USE_COLOR )
	diffuseColor.rgb *= vColor;
#endif`,zh=`#if defined( USE_COLOR_ALPHA )
	varying vec4 vColor;
#elif defined( USE_COLOR )
	varying vec3 vColor;
#endif`,Hh=`#if defined( USE_COLOR_ALPHA )
	varying vec4 vColor;
#elif defined( USE_COLOR ) || defined( USE_INSTANCING_COLOR ) || defined( USE_BATCHING_COLOR )
	varying vec3 vColor;
#endif`,kh=`#if defined( USE_COLOR_ALPHA )
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
#endif`,Gh=`#define PI 3.141592653589793
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
} // validated`,Vh=`#ifdef ENVMAP_TYPE_CUBE_UV
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
#endif`,Wh=`vec3 transformedNormal = objectNormal;
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
#endif`,qh=`#ifdef USE_DISPLACEMENTMAP
	uniform sampler2D displacementMap;
	uniform float displacementScale;
	uniform float displacementBias;
#endif`,Xh=`#ifdef USE_DISPLACEMENTMAP
	transformed += normalize( objectNormal ) * ( texture2D( displacementMap, vDisplacementMapUv ).x * displacementScale + displacementBias );
#endif`,$h=`#ifdef USE_EMISSIVEMAP
	vec4 emissiveColor = texture2D( emissiveMap, vEmissiveMapUv );
	totalEmissiveRadiance *= emissiveColor.rgb;
#endif`,Yh=`#ifdef USE_EMISSIVEMAP
	uniform sampler2D emissiveMap;
#endif`,Kh="gl_FragColor = linearToOutputTexel( gl_FragColor );",jh=`
const mat3 LINEAR_SRGB_TO_LINEAR_DISPLAY_P3 = mat3(
	vec3( 0.8224621, 0.177538, 0.0 ),
	vec3( 0.0331941, 0.9668058, 0.0 ),
	vec3( 0.0170827, 0.0723974, 0.9105199 )
);
const mat3 LINEAR_DISPLAY_P3_TO_LINEAR_SRGB = mat3(
	vec3( 1.2249401, - 0.2249404, 0.0 ),
	vec3( - 0.0420569, 1.0420571, 0.0 ),
	vec3( - 0.0196376, - 0.0786361, 1.0982735 )
);
vec4 LinearSRGBToLinearDisplayP3( in vec4 value ) {
	return vec4( value.rgb * LINEAR_SRGB_TO_LINEAR_DISPLAY_P3, value.a );
}
vec4 LinearDisplayP3ToLinearSRGB( in vec4 value ) {
	return vec4( value.rgb * LINEAR_DISPLAY_P3_TO_LINEAR_SRGB, value.a );
}
vec4 LinearTransferOETF( in vec4 value ) {
	return value;
}
vec4 sRGBTransferOETF( in vec4 value ) {
	return vec4( mix( pow( value.rgb, vec3( 0.41666 ) ) * 1.055 - vec3( 0.055 ), value.rgb * 12.92, vec3( lessThanEqual( value.rgb, vec3( 0.0031308 ) ) ) ), value.a );
}`,Zh=`#ifdef USE_ENVMAP
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
#endif`,Jh=`#ifdef USE_ENVMAP
	uniform float envMapIntensity;
	uniform float flipEnvMap;
	uniform mat3 envMapRotation;
	#ifdef ENVMAP_TYPE_CUBE
		uniform samplerCube envMap;
	#else
		uniform sampler2D envMap;
	#endif
	
#endif`,Qh=`#ifdef USE_ENVMAP
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
#endif`,tf=`#ifdef USE_ENVMAP
	#if defined( USE_BUMPMAP ) || defined( USE_NORMALMAP ) || defined( PHONG ) || defined( LAMBERT )
		#define ENV_WORLDPOS
	#endif
	#ifdef ENV_WORLDPOS
		
		varying vec3 vWorldPosition;
	#else
		varying vec3 vReflect;
		uniform float refractionRatio;
	#endif
#endif`,ef=`#ifdef USE_ENVMAP
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
#endif`,nf=`#ifdef USE_FOG
	vFogDepth = - mvPosition.z;
#endif`,rf=`#ifdef USE_FOG
	varying float vFogDepth;
#endif`,sf=`#ifdef USE_FOG
	#ifdef FOG_EXP2
		float fogFactor = 1.0 - exp( - fogDensity * fogDensity * vFogDepth * vFogDepth );
	#else
		float fogFactor = smoothstep( fogNear, fogFar, vFogDepth );
	#endif
	gl_FragColor.rgb = mix( gl_FragColor.rgb, fogColor, fogFactor );
#endif`,af=`#ifdef USE_FOG
	uniform vec3 fogColor;
	varying float vFogDepth;
	#ifdef FOG_EXP2
		uniform float fogDensity;
	#else
		uniform float fogNear;
		uniform float fogFar;
	#endif
#endif`,of=`#ifdef USE_GRADIENTMAP
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
}`,cf=`#ifdef USE_LIGHTMAP
	uniform sampler2D lightMap;
	uniform float lightMapIntensity;
#endif`,lf=`LambertMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.specularStrength = specularStrength;`,uf=`varying vec3 vViewPosition;
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
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Lambert`,hf=`uniform bool receiveShadow;
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
#endif`,ff=`#ifdef USE_ENVMAP
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
#endif`,df=`ToonMaterial material;
material.diffuseColor = diffuseColor.rgb;`,pf=`varying vec3 vViewPosition;
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
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Toon`,mf=`BlinnPhongMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.specularColor = specular;
material.specularShininess = shininess;
material.specularStrength = specularStrength;`,_f=`varying vec3 vViewPosition;
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
#define RE_IndirectDiffuse		RE_IndirectDiffuse_BlinnPhong`,gf=`PhysicalMaterial material;
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
#endif`,xf=`struct PhysicalMaterial {
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
}`,vf=`
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
#endif`,yf=`#if defined( RE_IndirectDiffuse )
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
#endif`,Mf=`#if defined( RE_IndirectDiffuse )
	RE_IndirectDiffuse( irradiance, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
#endif
#if defined( RE_IndirectSpecular )
	RE_IndirectSpecular( radiance, iblIrradiance, clearcoatRadiance, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
#endif`,Sf=`#if defined( USE_LOGDEPTHBUF )
	gl_FragDepth = vIsPerspective == 0.0 ? gl_FragCoord.z : log2( vFragDepth ) * logDepthBufFC * 0.5;
#endif`,Ef=`#if defined( USE_LOGDEPTHBUF )
	uniform float logDepthBufFC;
	varying float vFragDepth;
	varying float vIsPerspective;
#endif`,Tf=`#ifdef USE_LOGDEPTHBUF
	varying float vFragDepth;
	varying float vIsPerspective;
#endif`,wf=`#ifdef USE_LOGDEPTHBUF
	vFragDepth = 1.0 + gl_Position.w;
	vIsPerspective = float( isPerspectiveMatrix( projectionMatrix ) );
#endif`,Af=`#ifdef USE_MAP
	vec4 sampledDiffuseColor = texture2D( map, vMapUv );
	#ifdef DECODE_VIDEO_TEXTURE
		sampledDiffuseColor = vec4( mix( pow( sampledDiffuseColor.rgb * 0.9478672986 + vec3( 0.0521327014 ), vec3( 2.4 ) ), sampledDiffuseColor.rgb * 0.0773993808, vec3( lessThanEqual( sampledDiffuseColor.rgb, vec3( 0.04045 ) ) ) ), sampledDiffuseColor.w );
	
	#endif
	diffuseColor *= sampledDiffuseColor;
#endif`,bf=`#ifdef USE_MAP
	uniform sampler2D map;
#endif`,Cf=`#if defined( USE_MAP ) || defined( USE_ALPHAMAP )
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
#endif`,Rf=`#if defined( USE_POINTS_UV )
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
#endif`,Pf=`float metalnessFactor = metalness;
#ifdef USE_METALNESSMAP
	vec4 texelMetalness = texture2D( metalnessMap, vMetalnessMapUv );
	metalnessFactor *= texelMetalness.b;
#endif`,Lf=`#ifdef USE_METALNESSMAP
	uniform sampler2D metalnessMap;
#endif`,If=`#ifdef USE_INSTANCING_MORPH
	float morphTargetInfluences[ MORPHTARGETS_COUNT ];
	float morphTargetBaseInfluence = texelFetch( morphTexture, ivec2( 0, gl_InstanceID ), 0 ).r;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		morphTargetInfluences[i] =  texelFetch( morphTexture, ivec2( i + 1, gl_InstanceID ), 0 ).r;
	}
#endif`,Df=`#if defined( USE_MORPHCOLORS )
	vColor *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		#if defined( USE_COLOR_ALPHA )
			if ( morphTargetInfluences[ i ] != 0.0 ) vColor += getMorph( gl_VertexID, i, 2 ) * morphTargetInfluences[ i ];
		#elif defined( USE_COLOR )
			if ( morphTargetInfluences[ i ] != 0.0 ) vColor += getMorph( gl_VertexID, i, 2 ).rgb * morphTargetInfluences[ i ];
		#endif
	}
#endif`,Uf=`#ifdef USE_MORPHNORMALS
	objectNormal *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		if ( morphTargetInfluences[ i ] != 0.0 ) objectNormal += getMorph( gl_VertexID, i, 1 ).xyz * morphTargetInfluences[ i ];
	}
#endif`,Nf=`#ifdef USE_MORPHTARGETS
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
#endif`,Ff=`#ifdef USE_MORPHTARGETS
	transformed *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		if ( morphTargetInfluences[ i ] != 0.0 ) transformed += getMorph( gl_VertexID, i, 0 ).xyz * morphTargetInfluences[ i ];
	}
#endif`,Of=`float faceDirection = gl_FrontFacing ? 1.0 : - 1.0;
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
vec3 nonPerturbedNormal = normal;`,Bf=`#ifdef USE_NORMALMAP_OBJECTSPACE
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
#endif`,zf=`#ifndef FLAT_SHADED
	varying vec3 vNormal;
	#ifdef USE_TANGENT
		varying vec3 vTangent;
		varying vec3 vBitangent;
	#endif
#endif`,Hf=`#ifndef FLAT_SHADED
	varying vec3 vNormal;
	#ifdef USE_TANGENT
		varying vec3 vTangent;
		varying vec3 vBitangent;
	#endif
#endif`,kf=`#ifndef FLAT_SHADED
	vNormal = normalize( transformedNormal );
	#ifdef USE_TANGENT
		vTangent = normalize( transformedTangent );
		vBitangent = normalize( cross( vNormal, vTangent ) * tangent.w );
	#endif
#endif`,Gf=`#ifdef USE_NORMALMAP
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
#endif`,Vf=`#ifdef USE_CLEARCOAT
	vec3 clearcoatNormal = nonPerturbedNormal;
#endif`,Wf=`#ifdef USE_CLEARCOAT_NORMALMAP
	vec3 clearcoatMapN = texture2D( clearcoatNormalMap, vClearcoatNormalMapUv ).xyz * 2.0 - 1.0;
	clearcoatMapN.xy *= clearcoatNormalScale;
	clearcoatNormal = normalize( tbn2 * clearcoatMapN );
#endif`,qf=`#ifdef USE_CLEARCOATMAP
	uniform sampler2D clearcoatMap;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	uniform sampler2D clearcoatNormalMap;
	uniform vec2 clearcoatNormalScale;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	uniform sampler2D clearcoatRoughnessMap;
#endif`,Xf=`#ifdef USE_IRIDESCENCEMAP
	uniform sampler2D iridescenceMap;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	uniform sampler2D iridescenceThicknessMap;
#endif`,$f=`#ifdef OPAQUE
diffuseColor.a = 1.0;
#endif
#ifdef USE_TRANSMISSION
diffuseColor.a *= material.transmissionAlpha;
#endif
gl_FragColor = vec4( outgoingLight, diffuseColor.a );`,Yf=`vec3 packNormalToRGB( const in vec3 normal ) {
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
}`,Kf=`#ifdef PREMULTIPLIED_ALPHA
	gl_FragColor.rgb *= gl_FragColor.a;
#endif`,jf=`vec4 mvPosition = vec4( transformed, 1.0 );
#ifdef USE_BATCHING
	mvPosition = batchingMatrix * mvPosition;
#endif
#ifdef USE_INSTANCING
	mvPosition = instanceMatrix * mvPosition;
#endif
mvPosition = modelViewMatrix * mvPosition;
gl_Position = projectionMatrix * mvPosition;`,Zf=`#ifdef DITHERING
	gl_FragColor.rgb = dithering( gl_FragColor.rgb );
#endif`,Jf=`#ifdef DITHERING
	vec3 dithering( vec3 color ) {
		float grid_position = rand( gl_FragCoord.xy );
		vec3 dither_shift_RGB = vec3( 0.25 / 255.0, -0.25 / 255.0, 0.25 / 255.0 );
		dither_shift_RGB = mix( 2.0 * dither_shift_RGB, -2.0 * dither_shift_RGB, grid_position );
		return color + dither_shift_RGB;
	}
#endif`,Qf=`float roughnessFactor = roughness;
#ifdef USE_ROUGHNESSMAP
	vec4 texelRoughness = texture2D( roughnessMap, vRoughnessMapUv );
	roughnessFactor *= texelRoughness.g;
#endif`,td=`#ifdef USE_ROUGHNESSMAP
	uniform sampler2D roughnessMap;
#endif`,ed=`#if NUM_SPOT_LIGHT_COORDS > 0
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
#endif`,nd=`#if NUM_SPOT_LIGHT_COORDS > 0
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
#endif`,id=`#if ( defined( USE_SHADOWMAP ) && ( NUM_DIR_LIGHT_SHADOWS > 0 || NUM_POINT_LIGHT_SHADOWS > 0 ) ) || ( NUM_SPOT_LIGHT_COORDS > 0 )
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
#endif`,rd=`float getShadowMask() {
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
}`,sd=`#ifdef USE_SKINNING
	mat4 boneMatX = getBoneMatrix( skinIndex.x );
	mat4 boneMatY = getBoneMatrix( skinIndex.y );
	mat4 boneMatZ = getBoneMatrix( skinIndex.z );
	mat4 boneMatW = getBoneMatrix( skinIndex.w );
#endif`,ad=`#ifdef USE_SKINNING
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
#endif`,od=`#ifdef USE_SKINNING
	vec4 skinVertex = bindMatrix * vec4( transformed, 1.0 );
	vec4 skinned = vec4( 0.0 );
	skinned += boneMatX * skinVertex * skinWeight.x;
	skinned += boneMatY * skinVertex * skinWeight.y;
	skinned += boneMatZ * skinVertex * skinWeight.z;
	skinned += boneMatW * skinVertex * skinWeight.w;
	transformed = ( bindMatrixInverse * skinned ).xyz;
#endif`,cd=`#ifdef USE_SKINNING
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
#endif`,ld=`float specularStrength;
#ifdef USE_SPECULARMAP
	vec4 texelSpecular = texture2D( specularMap, vSpecularMapUv );
	specularStrength = texelSpecular.r;
#else
	specularStrength = 1.0;
#endif`,ud=`#ifdef USE_SPECULARMAP
	uniform sampler2D specularMap;
#endif`,hd=`#if defined( TONE_MAPPING )
	gl_FragColor.rgb = toneMapping( gl_FragColor.rgb );
#endif`,fd=`#ifndef saturate
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
vec3 OptimizedCineonToneMapping( vec3 color ) {
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
vec3 CustomToneMapping( vec3 color ) { return color; }`,dd=`#ifdef USE_TRANSMISSION
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
#endif`,pd=`#ifdef USE_TRANSMISSION
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
#endif`,md=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
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
#endif`,_d=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
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
#endif`,gd=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
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
#endif`,xd=`#if defined( USE_ENVMAP ) || defined( DISTANCE ) || defined ( USE_SHADOWMAP ) || defined ( USE_TRANSMISSION ) || NUM_SPOT_LIGHT_COORDS > 0
	vec4 worldPosition = vec4( transformed, 1.0 );
	#ifdef USE_BATCHING
		worldPosition = batchingMatrix * worldPosition;
	#endif
	#ifdef USE_INSTANCING
		worldPosition = instanceMatrix * worldPosition;
	#endif
	worldPosition = modelMatrix * worldPosition;
#endif`;const vd=`varying vec2 vUv;
uniform mat3 uvTransform;
void main() {
	vUv = ( uvTransform * vec3( uv, 1 ) ).xy;
	gl_Position = vec4( position.xy, 1.0, 1.0 );
}`,yd=`uniform sampler2D t2D;
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
}`,Md=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
	gl_Position.z = gl_Position.w;
}`,Sd=`#ifdef ENVMAP_TYPE_CUBE
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
}`,Ed=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
	gl_Position.z = gl_Position.w;
}`,Td=`uniform samplerCube tCube;
uniform float tFlip;
uniform float opacity;
varying vec3 vWorldDirection;
void main() {
	vec4 texColor = textureCube( tCube, vec3( tFlip * vWorldDirection.x, vWorldDirection.yz ) );
	gl_FragColor = texColor;
	gl_FragColor.a *= opacity;
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,wd=`#include <common>
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
}`,Ad=`#if DEPTH_PACKING == 3200
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
}`,bd=`#define DISTANCE
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
}`,Cd=`#define DISTANCE
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
}`,Rd=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
}`,Pd=`uniform sampler2D tEquirect;
varying vec3 vWorldDirection;
#include <common>
void main() {
	vec3 direction = normalize( vWorldDirection );
	vec2 sampleUV = equirectUv( direction );
	gl_FragColor = texture2D( tEquirect, sampleUV );
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,Ld=`uniform float scale;
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
}`,Id=`uniform vec3 diffuse;
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
}`,Dd=`#include <common>
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
}`,Ud=`uniform vec3 diffuse;
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
}`,Nd=`#define LAMBERT
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
}`,Fd=`#define LAMBERT
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
}`,Od=`#define MATCAP
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
}`,Bd=`#define MATCAP
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
}`,zd=`#define NORMAL
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
}`,Hd=`#define NORMAL
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
}`,kd=`#define PHONG
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
}`,Gd=`#define PHONG
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
}`,Vd=`#define STANDARD
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
}`,Wd=`#define STANDARD
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
}`,qd=`#define TOON
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
}`,Xd=`#define TOON
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
}`,$d=`uniform float size;
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
}`,Yd=`uniform vec3 diffuse;
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
}`,Kd=`#include <common>
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
}`,jd=`uniform vec3 color;
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
}`,Zd=`uniform float rotation;
uniform vec2 center;
#include <common>
#include <uv_pars_vertex>
#include <fog_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	vec4 mvPosition = modelViewMatrix * vec4( 0.0, 0.0, 0.0, 1.0 );
	vec2 scale;
	scale.x = length( vec3( modelMatrix[ 0 ].x, modelMatrix[ 0 ].y, modelMatrix[ 0 ].z ) );
	scale.y = length( vec3( modelMatrix[ 1 ].x, modelMatrix[ 1 ].y, modelMatrix[ 1 ].z ) );
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
}`,Jd=`uniform vec3 diffuse;
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
}`,Ut={alphahash_fragment:vh,alphahash_pars_fragment:yh,alphamap_fragment:Mh,alphamap_pars_fragment:Sh,alphatest_fragment:Eh,alphatest_pars_fragment:Th,aomap_fragment:wh,aomap_pars_fragment:Ah,batching_pars_vertex:bh,batching_vertex:Ch,begin_vertex:Rh,beginnormal_vertex:Ph,bsdfs:Lh,iridescence_fragment:Ih,bumpmap_pars_fragment:Dh,clipping_planes_fragment:Uh,clipping_planes_pars_fragment:Nh,clipping_planes_pars_vertex:Fh,clipping_planes_vertex:Oh,color_fragment:Bh,color_pars_fragment:zh,color_pars_vertex:Hh,color_vertex:kh,common:Gh,cube_uv_reflection_fragment:Vh,defaultnormal_vertex:Wh,displacementmap_pars_vertex:qh,displacementmap_vertex:Xh,emissivemap_fragment:$h,emissivemap_pars_fragment:Yh,colorspace_fragment:Kh,colorspace_pars_fragment:jh,envmap_fragment:Zh,envmap_common_pars_fragment:Jh,envmap_pars_fragment:Qh,envmap_pars_vertex:tf,envmap_physical_pars_fragment:ff,envmap_vertex:ef,fog_vertex:nf,fog_pars_vertex:rf,fog_fragment:sf,fog_pars_fragment:af,gradientmap_pars_fragment:of,lightmap_pars_fragment:cf,lights_lambert_fragment:lf,lights_lambert_pars_fragment:uf,lights_pars_begin:hf,lights_toon_fragment:df,lights_toon_pars_fragment:pf,lights_phong_fragment:mf,lights_phong_pars_fragment:_f,lights_physical_fragment:gf,lights_physical_pars_fragment:xf,lights_fragment_begin:vf,lights_fragment_maps:yf,lights_fragment_end:Mf,logdepthbuf_fragment:Sf,logdepthbuf_pars_fragment:Ef,logdepthbuf_pars_vertex:Tf,logdepthbuf_vertex:wf,map_fragment:Af,map_pars_fragment:bf,map_particle_fragment:Cf,map_particle_pars_fragment:Rf,metalnessmap_fragment:Pf,metalnessmap_pars_fragment:Lf,morphinstance_vertex:If,morphcolor_vertex:Df,morphnormal_vertex:Uf,morphtarget_pars_vertex:Nf,morphtarget_vertex:Ff,normal_fragment_begin:Of,normal_fragment_maps:Bf,normal_pars_fragment:zf,normal_pars_vertex:Hf,normal_vertex:kf,normalmap_pars_fragment:Gf,clearcoat_normal_fragment_begin:Vf,clearcoat_normal_fragment_maps:Wf,clearcoat_pars_fragment:qf,iridescence_pars_fragment:Xf,opaque_fragment:$f,packing:Yf,premultiplied_alpha_fragment:Kf,project_vertex:jf,dithering_fragment:Zf,dithering_pars_fragment:Jf,roughnessmap_fragment:Qf,roughnessmap_pars_fragment:td,shadowmap_pars_fragment:ed,shadowmap_pars_vertex:nd,shadowmap_vertex:id,shadowmask_pars_fragment:rd,skinbase_vertex:sd,skinning_pars_vertex:ad,skinning_vertex:od,skinnormal_vertex:cd,specularmap_fragment:ld,specularmap_pars_fragment:ud,tonemapping_fragment:hd,tonemapping_pars_fragment:fd,transmission_fragment:dd,transmission_pars_fragment:pd,uv_pars_fragment:md,uv_pars_vertex:_d,uv_vertex:gd,worldpos_vertex:xd,background_vert:vd,background_frag:yd,backgroundCube_vert:Md,backgroundCube_frag:Sd,cube_vert:Ed,cube_frag:Td,depth_vert:wd,depth_frag:Ad,distanceRGBA_vert:bd,distanceRGBA_frag:Cd,equirect_vert:Rd,equirect_frag:Pd,linedashed_vert:Ld,linedashed_frag:Id,meshbasic_vert:Dd,meshbasic_frag:Ud,meshlambert_vert:Nd,meshlambert_frag:Fd,meshmatcap_vert:Od,meshmatcap_frag:Bd,meshnormal_vert:zd,meshnormal_frag:Hd,meshphong_vert:kd,meshphong_frag:Gd,meshphysical_vert:Vd,meshphysical_frag:Wd,meshtoon_vert:qd,meshtoon_frag:Xd,points_vert:$d,points_frag:Yd,shadow_vert:Kd,shadow_frag:jd,sprite_vert:Zd,sprite_frag:Jd},ht={common:{diffuse:{value:new Ht(16777215)},opacity:{value:1},map:{value:null},mapTransform:{value:new Nt},alphaMap:{value:null},alphaMapTransform:{value:new Nt},alphaTest:{value:0}},specularmap:{specularMap:{value:null},specularMapTransform:{value:new Nt}},envmap:{envMap:{value:null},envMapRotation:{value:new Nt},flipEnvMap:{value:-1},reflectivity:{value:1},ior:{value:1.5},refractionRatio:{value:.98}},aomap:{aoMap:{value:null},aoMapIntensity:{value:1},aoMapTransform:{value:new Nt}},lightmap:{lightMap:{value:null},lightMapIntensity:{value:1},lightMapTransform:{value:new Nt}},bumpmap:{bumpMap:{value:null},bumpMapTransform:{value:new Nt},bumpScale:{value:1}},normalmap:{normalMap:{value:null},normalMapTransform:{value:new Nt},normalScale:{value:new Bt(1,1)}},displacementmap:{displacementMap:{value:null},displacementMapTransform:{value:new Nt},displacementScale:{value:1},displacementBias:{value:0}},emissivemap:{emissiveMap:{value:null},emissiveMapTransform:{value:new Nt}},metalnessmap:{metalnessMap:{value:null},metalnessMapTransform:{value:new Nt}},roughnessmap:{roughnessMap:{value:null},roughnessMapTransform:{value:new Nt}},gradientmap:{gradientMap:{value:null}},fog:{fogDensity:{value:25e-5},fogNear:{value:1},fogFar:{value:2e3},fogColor:{value:new Ht(16777215)}},lights:{ambientLightColor:{value:[]},lightProbe:{value:[]},directionalLights:{value:[],properties:{direction:{},color:{}}},directionalLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{}}},directionalShadowMap:{value:[]},directionalShadowMatrix:{value:[]},spotLights:{value:[],properties:{color:{},position:{},direction:{},distance:{},coneCos:{},penumbraCos:{},decay:{}}},spotLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{}}},spotLightMap:{value:[]},spotShadowMap:{value:[]},spotLightMatrix:{value:[]},pointLights:{value:[],properties:{color:{},position:{},decay:{},distance:{}}},pointLightShadows:{value:[],properties:{shadowIntensity:1,shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{},shadowCameraNear:{},shadowCameraFar:{}}},pointShadowMap:{value:[]},pointShadowMatrix:{value:[]},hemisphereLights:{value:[],properties:{direction:{},skyColor:{},groundColor:{}}},rectAreaLights:{value:[],properties:{color:{},position:{},width:{},height:{}}},ltc_1:{value:null},ltc_2:{value:null}},points:{diffuse:{value:new Ht(16777215)},opacity:{value:1},size:{value:1},scale:{value:1},map:{value:null},alphaMap:{value:null},alphaMapTransform:{value:new Nt},alphaTest:{value:0},uvTransform:{value:new Nt}},sprite:{diffuse:{value:new Ht(16777215)},opacity:{value:1},center:{value:new Bt(.5,.5)},rotation:{value:0},map:{value:null},mapTransform:{value:new Nt},alphaMap:{value:null},alphaMapTransform:{value:new Nt},alphaTest:{value:0}}},qe={basic:{uniforms:ve([ht.common,ht.specularmap,ht.envmap,ht.aomap,ht.lightmap,ht.fog]),vertexShader:Ut.meshbasic_vert,fragmentShader:Ut.meshbasic_frag},lambert:{uniforms:ve([ht.common,ht.specularmap,ht.envmap,ht.aomap,ht.lightmap,ht.emissivemap,ht.bumpmap,ht.normalmap,ht.displacementmap,ht.fog,ht.lights,{emissive:{value:new Ht(0)}}]),vertexShader:Ut.meshlambert_vert,fragmentShader:Ut.meshlambert_frag},phong:{uniforms:ve([ht.common,ht.specularmap,ht.envmap,ht.aomap,ht.lightmap,ht.emissivemap,ht.bumpmap,ht.normalmap,ht.displacementmap,ht.fog,ht.lights,{emissive:{value:new Ht(0)},specular:{value:new Ht(1118481)},shininess:{value:30}}]),vertexShader:Ut.meshphong_vert,fragmentShader:Ut.meshphong_frag},standard:{uniforms:ve([ht.common,ht.envmap,ht.aomap,ht.lightmap,ht.emissivemap,ht.bumpmap,ht.normalmap,ht.displacementmap,ht.roughnessmap,ht.metalnessmap,ht.fog,ht.lights,{emissive:{value:new Ht(0)},roughness:{value:1},metalness:{value:0},envMapIntensity:{value:1}}]),vertexShader:Ut.meshphysical_vert,fragmentShader:Ut.meshphysical_frag},toon:{uniforms:ve([ht.common,ht.aomap,ht.lightmap,ht.emissivemap,ht.bumpmap,ht.normalmap,ht.displacementmap,ht.gradientmap,ht.fog,ht.lights,{emissive:{value:new Ht(0)}}]),vertexShader:Ut.meshtoon_vert,fragmentShader:Ut.meshtoon_frag},matcap:{uniforms:ve([ht.common,ht.bumpmap,ht.normalmap,ht.displacementmap,ht.fog,{matcap:{value:null}}]),vertexShader:Ut.meshmatcap_vert,fragmentShader:Ut.meshmatcap_frag},points:{uniforms:ve([ht.points,ht.fog]),vertexShader:Ut.points_vert,fragmentShader:Ut.points_frag},dashed:{uniforms:ve([ht.common,ht.fog,{scale:{value:1},dashSize:{value:1},totalSize:{value:2}}]),vertexShader:Ut.linedashed_vert,fragmentShader:Ut.linedashed_frag},depth:{uniforms:ve([ht.common,ht.displacementmap]),vertexShader:Ut.depth_vert,fragmentShader:Ut.depth_frag},normal:{uniforms:ve([ht.common,ht.bumpmap,ht.normalmap,ht.displacementmap,{opacity:{value:1}}]),vertexShader:Ut.meshnormal_vert,fragmentShader:Ut.meshnormal_frag},sprite:{uniforms:ve([ht.sprite,ht.fog]),vertexShader:Ut.sprite_vert,fragmentShader:Ut.sprite_frag},background:{uniforms:{uvTransform:{value:new Nt},t2D:{value:null},backgroundIntensity:{value:1}},vertexShader:Ut.background_vert,fragmentShader:Ut.background_frag},backgroundCube:{uniforms:{envMap:{value:null},flipEnvMap:{value:-1},backgroundBlurriness:{value:0},backgroundIntensity:{value:1},backgroundRotation:{value:new Nt}},vertexShader:Ut.backgroundCube_vert,fragmentShader:Ut.backgroundCube_frag},cube:{uniforms:{tCube:{value:null},tFlip:{value:-1},opacity:{value:1}},vertexShader:Ut.cube_vert,fragmentShader:Ut.cube_frag},equirect:{uniforms:{tEquirect:{value:null}},vertexShader:Ut.equirect_vert,fragmentShader:Ut.equirect_frag},distanceRGBA:{uniforms:ve([ht.common,ht.displacementmap,{referencePosition:{value:new k},nearDistance:{value:1},farDistance:{value:1e3}}]),vertexShader:Ut.distanceRGBA_vert,fragmentShader:Ut.distanceRGBA_frag},shadow:{uniforms:ve([ht.lights,ht.fog,{color:{value:new Ht(0)},opacity:{value:1}}]),vertexShader:Ut.shadow_vert,fragmentShader:Ut.shadow_frag}};qe.physical={uniforms:ve([qe.standard.uniforms,{clearcoat:{value:0},clearcoatMap:{value:null},clearcoatMapTransform:{value:new Nt},clearcoatNormalMap:{value:null},clearcoatNormalMapTransform:{value:new Nt},clearcoatNormalScale:{value:new Bt(1,1)},clearcoatRoughness:{value:0},clearcoatRoughnessMap:{value:null},clearcoatRoughnessMapTransform:{value:new Nt},dispersion:{value:0},iridescence:{value:0},iridescenceMap:{value:null},iridescenceMapTransform:{value:new Nt},iridescenceIOR:{value:1.3},iridescenceThicknessMinimum:{value:100},iridescenceThicknessMaximum:{value:400},iridescenceThicknessMap:{value:null},iridescenceThicknessMapTransform:{value:new Nt},sheen:{value:0},sheenColor:{value:new Ht(0)},sheenColorMap:{value:null},sheenColorMapTransform:{value:new Nt},sheenRoughness:{value:1},sheenRoughnessMap:{value:null},sheenRoughnessMapTransform:{value:new Nt},transmission:{value:0},transmissionMap:{value:null},transmissionMapTransform:{value:new Nt},transmissionSamplerSize:{value:new Bt},transmissionSamplerMap:{value:null},thickness:{value:0},thicknessMap:{value:null},thicknessMapTransform:{value:new Nt},attenuationDistance:{value:0},attenuationColor:{value:new Ht(0)},specularColor:{value:new Ht(1,1,1)},specularColorMap:{value:null},specularColorMapTransform:{value:new Nt},specularIntensity:{value:1},specularIntensityMap:{value:null},specularIntensityMapTransform:{value:new Nt},anisotropyVector:{value:new Bt},anisotropyMap:{value:null},anisotropyMapTransform:{value:new Nt}}]),vertexShader:Ut.meshphysical_vert,fragmentShader:Ut.meshphysical_frag};const gr={r:0,b:0,g:0},Nn=new Ge,Qd=new Qt;function tp(n,t,e,i,r,s,o){const a=new Ht(0);let c=s===!0?0:1,u,f,p=null,m=0,g=null;function v(C){let w=C.isScene===!0?C.background:null;return w&&w.isTexture&&(w=(C.backgroundBlurriness>0?e:t).get(w)),w}function M(C){let w=!1;const b=v(C);b===null?d(a,c):b&&b.isColor&&(d(b,1),w=!0);const z=n.xr.getEnvironmentBlendMode();z==="additive"?i.buffers.color.setClear(0,0,0,1,o):z==="alpha-blend"&&i.buffers.color.setClear(0,0,0,0,o),(n.autoClear||w)&&(i.buffers.depth.setTest(!0),i.buffers.depth.setMask(!0),i.buffers.color.setMask(!0),n.clear(n.autoClearColor,n.autoClearDepth,n.autoClearStencil))}function _(C,w){const b=v(w);b&&(b.isCubeTexture||b.mapping===Xr)?(f===void 0&&(f=new ae(new Xn(1,1,1),new bn({name:"BackgroundCubeMaterial",uniforms:Ti(qe.backgroundCube.uniforms),vertexShader:qe.backgroundCube.vertexShader,fragmentShader:qe.backgroundCube.fragmentShader,side:Se,depthTest:!1,depthWrite:!1,fog:!1})),f.geometry.deleteAttribute("normal"),f.geometry.deleteAttribute("uv"),f.onBeforeRender=function(z,P,I){this.matrixWorld.copyPosition(I.matrixWorld)},Object.defineProperty(f.material,"envMap",{get:function(){return this.uniforms.envMap.value}}),r.update(f)),Nn.copy(w.backgroundRotation),Nn.x*=-1,Nn.y*=-1,Nn.z*=-1,b.isCubeTexture&&b.isRenderTargetTexture===!1&&(Nn.y*=-1,Nn.z*=-1),f.material.uniforms.envMap.value=b,f.material.uniforms.flipEnvMap.value=b.isCubeTexture&&b.isRenderTargetTexture===!1?-1:1,f.material.uniforms.backgroundBlurriness.value=w.backgroundBlurriness,f.material.uniforms.backgroundIntensity.value=w.backgroundIntensity,f.material.uniforms.backgroundRotation.value.setFromMatrix4(Qd.makeRotationFromEuler(Nn)),f.material.toneMapped=$t.getTransfer(b.colorSpace)!==Kt,(p!==b||m!==b.version||g!==n.toneMapping)&&(f.material.needsUpdate=!0,p=b,m=b.version,g=n.toneMapping),f.layers.enableAll(),C.unshift(f,f.geometry,f.material,0,0,null)):b&&b.isTexture&&(u===void 0&&(u=new ae(new Ki(2,2),new bn({name:"BackgroundMaterial",uniforms:Ti(qe.background.uniforms),vertexShader:qe.background.vertexShader,fragmentShader:qe.background.fragmentShader,side:An,depthTest:!1,depthWrite:!1,fog:!1})),u.geometry.deleteAttribute("normal"),Object.defineProperty(u.material,"map",{get:function(){return this.uniforms.t2D.value}}),r.update(u)),u.material.uniforms.t2D.value=b,u.material.uniforms.backgroundIntensity.value=w.backgroundIntensity,u.material.toneMapped=$t.getTransfer(b.colorSpace)!==Kt,b.matrixAutoUpdate===!0&&b.updateMatrix(),u.material.uniforms.uvTransform.value.copy(b.matrix),(p!==b||m!==b.version||g!==n.toneMapping)&&(u.material.needsUpdate=!0,p=b,m=b.version,g=n.toneMapping),u.layers.enableAll(),C.unshift(u,u.geometry,u.material,0,0,null))}function d(C,w){C.getRGB(gr,Zc(n)),i.buffers.color.setClear(gr.r,gr.g,gr.b,w,o)}return{getClearColor:function(){return a},setClearColor:function(C,w=1){a.set(C),c=w,d(a,c)},getClearAlpha:function(){return c},setClearAlpha:function(C){c=C,d(a,c)},render:M,addToRenderList:_}}function ep(n,t){const e=n.getParameter(n.MAX_VERTEX_ATTRIBS),i={},r=m(null);let s=r,o=!1;function a(E,N,$,W,Y){let tt=!1;const K=p(W,$,N);s!==K&&(s=K,u(s.object)),tt=g(E,W,$,Y),tt&&v(E,W,$,Y),Y!==null&&t.update(Y,n.ELEMENT_ARRAY_BUFFER),(tt||o)&&(o=!1,b(E,N,$,W),Y!==null&&n.bindBuffer(n.ELEMENT_ARRAY_BUFFER,t.get(Y).buffer))}function c(){return n.createVertexArray()}function u(E){return n.bindVertexArray(E)}function f(E){return n.deleteVertexArray(E)}function p(E,N,$){const W=$.wireframe===!0;let Y=i[E.id];Y===void 0&&(Y={},i[E.id]=Y);let tt=Y[N.id];tt===void 0&&(tt={},Y[N.id]=tt);let K=tt[W];return K===void 0&&(K=m(c()),tt[W]=K),K}function m(E){const N=[],$=[],W=[];for(let Y=0;Y<e;Y++)N[Y]=0,$[Y]=0,W[Y]=0;return{geometry:null,program:null,wireframe:!1,newAttributes:N,enabledAttributes:$,attributeDivisors:W,object:E,attributes:{},index:null}}function g(E,N,$,W){const Y=s.attributes,tt=N.attributes;let K=0;const st=$.getAttributes();for(const j in st)if(st[j].location>=0){const _t=Y[j];let ut=tt[j];if(ut===void 0&&(j==="instanceMatrix"&&E.instanceMatrix&&(ut=E.instanceMatrix),j==="instanceColor"&&E.instanceColor&&(ut=E.instanceColor)),_t===void 0||_t.attribute!==ut||ut&&_t.data!==ut.data)return!0;K++}return s.attributesNum!==K||s.index!==W}function v(E,N,$,W){const Y={},tt=N.attributes;let K=0;const st=$.getAttributes();for(const j in st)if(st[j].location>=0){let _t=tt[j];_t===void 0&&(j==="instanceMatrix"&&E.instanceMatrix&&(_t=E.instanceMatrix),j==="instanceColor"&&E.instanceColor&&(_t=E.instanceColor));const ut={};ut.attribute=_t,_t&&_t.data&&(ut.data=_t.data),Y[j]=ut,K++}s.attributes=Y,s.attributesNum=K,s.index=W}function M(){const E=s.newAttributes;for(let N=0,$=E.length;N<$;N++)E[N]=0}function _(E){d(E,0)}function d(E,N){const $=s.newAttributes,W=s.enabledAttributes,Y=s.attributeDivisors;$[E]=1,W[E]===0&&(n.enableVertexAttribArray(E),W[E]=1),Y[E]!==N&&(n.vertexAttribDivisor(E,N),Y[E]=N)}function C(){const E=s.newAttributes,N=s.enabledAttributes;for(let $=0,W=N.length;$<W;$++)N[$]!==E[$]&&(n.disableVertexAttribArray($),N[$]=0)}function w(E,N,$,W,Y,tt,K){K===!0?n.vertexAttribIPointer(E,N,$,Y,tt):n.vertexAttribPointer(E,N,$,W,Y,tt)}function b(E,N,$,W){M();const Y=W.attributes,tt=$.getAttributes(),K=N.defaultAttributeValues;for(const st in tt){const j=tt[st];if(j.location>=0){let dt=Y[st];if(dt===void 0&&(st==="instanceMatrix"&&E.instanceMatrix&&(dt=E.instanceMatrix),st==="instanceColor"&&E.instanceColor&&(dt=E.instanceColor)),dt!==void 0){const _t=dt.normalized,ut=dt.itemSize,wt=t.get(dt);if(wt===void 0)continue;const kt=wt.buffer,Z=wt.type,at=wt.bytesPerElement,gt=Z===n.INT||Z===n.UNSIGNED_INT||dt.gpuType===Ca;if(dt.isInterleavedBufferAttribute){const pt=dt.data,At=pt.stride,Dt=dt.offset;if(pt.isInstancedInterleavedBuffer){for(let Ft=0;Ft<j.locationSize;Ft++)d(j.location+Ft,pt.meshPerAttribute);E.isInstancedMesh!==!0&&W._maxInstanceCount===void 0&&(W._maxInstanceCount=pt.meshPerAttribute*pt.count)}else for(let Ft=0;Ft<j.locationSize;Ft++)_(j.location+Ft);n.bindBuffer(n.ARRAY_BUFFER,kt);for(let Ft=0;Ft<j.locationSize;Ft++)w(j.location+Ft,ut/j.locationSize,Z,_t,At*at,(Dt+ut/j.locationSize*Ft)*at,gt)}else{if(dt.isInstancedBufferAttribute){for(let pt=0;pt<j.locationSize;pt++)d(j.location+pt,dt.meshPerAttribute);E.isInstancedMesh!==!0&&W._maxInstanceCount===void 0&&(W._maxInstanceCount=dt.meshPerAttribute*dt.count)}else for(let pt=0;pt<j.locationSize;pt++)_(j.location+pt);n.bindBuffer(n.ARRAY_BUFFER,kt);for(let pt=0;pt<j.locationSize;pt++)w(j.location+pt,ut/j.locationSize,Z,_t,ut*at,ut/j.locationSize*pt*at,gt)}}else if(K!==void 0){const _t=K[st];if(_t!==void 0)switch(_t.length){case 2:n.vertexAttrib2fv(j.location,_t);break;case 3:n.vertexAttrib3fv(j.location,_t);break;case 4:n.vertexAttrib4fv(j.location,_t);break;default:n.vertexAttrib1fv(j.location,_t)}}}}C()}function z(){F();for(const E in i){const N=i[E];for(const $ in N){const W=N[$];for(const Y in W)f(W[Y].object),delete W[Y];delete N[$]}delete i[E]}}function P(E){if(i[E.id]===void 0)return;const N=i[E.id];for(const $ in N){const W=N[$];for(const Y in W)f(W[Y].object),delete W[Y];delete N[$]}delete i[E.id]}function I(E){for(const N in i){const $=i[N];if($[E.id]===void 0)continue;const W=$[E.id];for(const Y in W)f(W[Y].object),delete W[Y];delete $[E.id]}}function F(){A(),o=!0,s!==r&&(s=r,u(s.object))}function A(){r.geometry=null,r.program=null,r.wireframe=!1}return{setup:a,reset:F,resetDefaultState:A,dispose:z,releaseStatesOfGeometry:P,releaseStatesOfProgram:I,initAttributes:M,enableAttribute:_,disableUnusedAttributes:C}}function np(n,t,e){let i;function r(u){i=u}function s(u,f){n.drawArrays(i,u,f),e.update(f,i,1)}function o(u,f,p){p!==0&&(n.drawArraysInstanced(i,u,f,p),e.update(f,i,p))}function a(u,f,p){if(p===0)return;t.get("WEBGL_multi_draw").multiDrawArraysWEBGL(i,u,0,f,0,p);let g=0;for(let v=0;v<p;v++)g+=f[v];e.update(g,i,1)}function c(u,f,p,m){if(p===0)return;const g=t.get("WEBGL_multi_draw");if(g===null)for(let v=0;v<u.length;v++)o(u[v],f[v],m[v]);else{g.multiDrawArraysInstancedWEBGL(i,u,0,f,0,m,0,p);let v=0;for(let M=0;M<p;M++)v+=f[M];for(let M=0;M<m.length;M++)e.update(v,i,m[M])}}this.setMode=r,this.render=s,this.renderInstances=o,this.renderMultiDraw=a,this.renderMultiDrawInstances=c}function ip(n,t,e,i){let r;function s(){if(r!==void 0)return r;if(t.has("EXT_texture_filter_anisotropic")===!0){const P=t.get("EXT_texture_filter_anisotropic");r=n.getParameter(P.MAX_TEXTURE_MAX_ANISOTROPY_EXT)}else r=0;return r}function o(P){return!(P!==He&&i.convert(P)!==n.getParameter(n.IMPLEMENTATION_COLOR_READ_FORMAT))}function a(P){const I=P===qi&&(t.has("EXT_color_buffer_half_float")||t.has("EXT_color_buffer_float"));return!(P!==ln&&i.convert(P)!==n.getParameter(n.IMPLEMENTATION_COLOR_READ_TYPE)&&P!==nn&&!I)}function c(P){if(P==="highp"){if(n.getShaderPrecisionFormat(n.VERTEX_SHADER,n.HIGH_FLOAT).precision>0&&n.getShaderPrecisionFormat(n.FRAGMENT_SHADER,n.HIGH_FLOAT).precision>0)return"highp";P="mediump"}return P==="mediump"&&n.getShaderPrecisionFormat(n.VERTEX_SHADER,n.MEDIUM_FLOAT).precision>0&&n.getShaderPrecisionFormat(n.FRAGMENT_SHADER,n.MEDIUM_FLOAT).precision>0?"mediump":"lowp"}let u=e.precision!==void 0?e.precision:"highp";const f=c(u);f!==u&&(console.warn("THREE.WebGLRenderer:",u,"not supported, using",f,"instead."),u=f);const p=e.logarithmicDepthBuffer===!0,m=n.getParameter(n.MAX_TEXTURE_IMAGE_UNITS),g=n.getParameter(n.MAX_VERTEX_TEXTURE_IMAGE_UNITS),v=n.getParameter(n.MAX_TEXTURE_SIZE),M=n.getParameter(n.MAX_CUBE_MAP_TEXTURE_SIZE),_=n.getParameter(n.MAX_VERTEX_ATTRIBS),d=n.getParameter(n.MAX_VERTEX_UNIFORM_VECTORS),C=n.getParameter(n.MAX_VARYING_VECTORS),w=n.getParameter(n.MAX_FRAGMENT_UNIFORM_VECTORS),b=g>0,z=n.getParameter(n.MAX_SAMPLES);return{isWebGL2:!0,getMaxAnisotropy:s,getMaxPrecision:c,textureFormatReadable:o,textureTypeReadable:a,precision:u,logarithmicDepthBuffer:p,maxTextures:m,maxVertexTextures:g,maxTextureSize:v,maxCubemapSize:M,maxAttributes:_,maxVertexUniforms:d,maxVaryings:C,maxFragmentUniforms:w,vertexTextures:b,maxSamples:z}}function rp(n){const t=this;let e=null,i=0,r=!1,s=!1;const o=new On,a=new Nt,c={value:null,needsUpdate:!1};this.uniform=c,this.numPlanes=0,this.numIntersection=0,this.init=function(p,m){const g=p.length!==0||m||i!==0||r;return r=m,i=p.length,g},this.beginShadows=function(){s=!0,f(null)},this.endShadows=function(){s=!1},this.setGlobalState=function(p,m){e=f(p,m,0)},this.setState=function(p,m,g){const v=p.clippingPlanes,M=p.clipIntersection,_=p.clipShadows,d=n.get(p);if(!r||v===null||v.length===0||s&&!_)s?f(null):u();else{const C=s?0:i,w=C*4;let b=d.clippingState||null;c.value=b,b=f(v,m,w,g);for(let z=0;z!==w;++z)b[z]=e[z];d.clippingState=b,this.numIntersection=M?this.numPlanes:0,this.numPlanes+=C}};function u(){c.value!==e&&(c.value=e,c.needsUpdate=i>0),t.numPlanes=i,t.numIntersection=0}function f(p,m,g,v){const M=p!==null?p.length:0;let _=null;if(M!==0){if(_=c.value,v!==!0||_===null){const d=g+M*4,C=m.matrixWorldInverse;a.getNormalMatrix(C),(_===null||_.length<d)&&(_=new Float32Array(d));for(let w=0,b=g;w!==M;++w,b+=4)o.copy(p[w]).applyMatrix4(C,a),o.normal.toArray(_,b),_[b+3]=o.constant}c.value=_,c.needsUpdate=!0}return t.numPlanes=M,t.numIntersection=0,_}}function sp(n){let t=new WeakMap;function e(o,a){return a===Vs?o.mapping=yi:a===Ws&&(o.mapping=Mi),o}function i(o){if(o&&o.isTexture){const a=o.mapping;if(a===Vs||a===Ws)if(t.has(o)){const c=t.get(o).texture;return e(c,o.mapping)}else{const c=o.image;if(c&&c.height>0){const u=new mh(c.height);return u.fromEquirectangularTexture(n,o),t.set(o,u),o.addEventListener("dispose",r),e(u.texture,o.mapping)}else return null}}return o}function r(o){const a=o.target;a.removeEventListener("dispose",r);const c=t.get(a);c!==void 0&&(t.delete(a),c.dispose())}function s(){t=new WeakMap}return{get:i,dispose:s}}class el extends Jc{constructor(t=-1,e=1,i=1,r=-1,s=.1,o=2e3){super(),this.isOrthographicCamera=!0,this.type="OrthographicCamera",this.zoom=1,this.view=null,this.left=t,this.right=e,this.top=i,this.bottom=r,this.near=s,this.far=o,this.updateProjectionMatrix()}copy(t,e){return super.copy(t,e),this.left=t.left,this.right=t.right,this.top=t.top,this.bottom=t.bottom,this.near=t.near,this.far=t.far,this.zoom=t.zoom,this.view=t.view===null?null:Object.assign({},t.view),this}setViewOffset(t,e,i,r,s,o){this.view===null&&(this.view={enabled:!0,fullWidth:1,fullHeight:1,offsetX:0,offsetY:0,width:1,height:1}),this.view.enabled=!0,this.view.fullWidth=t,this.view.fullHeight=e,this.view.offsetX=i,this.view.offsetY=r,this.view.width=s,this.view.height=o,this.updateProjectionMatrix()}clearViewOffset(){this.view!==null&&(this.view.enabled=!1),this.updateProjectionMatrix()}updateProjectionMatrix(){const t=(this.right-this.left)/(2*this.zoom),e=(this.top-this.bottom)/(2*this.zoom),i=(this.right+this.left)/2,r=(this.top+this.bottom)/2;let s=i-t,o=i+t,a=r+e,c=r-e;if(this.view!==null&&this.view.enabled){const u=(this.right-this.left)/this.view.fullWidth/this.zoom,f=(this.top-this.bottom)/this.view.fullHeight/this.zoom;s+=u*this.view.offsetX,o=s+u*this.view.width,a-=f*this.view.offsetY,c=a-f*this.view.height}this.projectionMatrix.makeOrthographic(s,o,a,c,this.near,this.far,this.coordinateSystem),this.projectionMatrixInverse.copy(this.projectionMatrix).invert()}toJSON(t){const e=super.toJSON(t);return e.object.zoom=this.zoom,e.object.left=this.left,e.object.right=this.right,e.object.top=this.top,e.object.bottom=this.bottom,e.object.near=this.near,e.object.far=this.far,this.view!==null&&(e.object.view=Object.assign({},this.view)),e}}const _i=4,To=[.125,.215,.35,.446,.526,.582],kn=20,bs=new el,wo=new Ht;let Cs=null,Rs=0,Ps=0,Ls=!1;const Bn=(1+Math.sqrt(5))/2,mi=1/Bn,Ao=[new k(-Bn,mi,0),new k(Bn,mi,0),new k(-mi,0,Bn),new k(mi,0,Bn),new k(0,Bn,-mi),new k(0,Bn,mi),new k(-1,1,-1),new k(1,1,-1),new k(-1,1,1),new k(1,1,1)];class bo{constructor(t){this._renderer=t,this._pingPongRenderTarget=null,this._lodMax=0,this._cubeSize=0,this._lodPlanes=[],this._sizeLods=[],this._sigmas=[],this._blurMaterial=null,this._cubemapMaterial=null,this._equirectMaterial=null,this._compileMaterial(this._blurMaterial)}fromScene(t,e=0,i=.1,r=100){Cs=this._renderer.getRenderTarget(),Rs=this._renderer.getActiveCubeFace(),Ps=this._renderer.getActiveMipmapLevel(),Ls=this._renderer.xr.enabled,this._renderer.xr.enabled=!1,this._setSize(256);const s=this._allocateTargets();return s.depthBuffer=!0,this._sceneToCubeUV(t,i,r,s),e>0&&this._blur(s,0,0,e),this._applyPMREM(s),this._cleanup(s),s}fromEquirectangular(t,e=null){return this._fromTexture(t,e)}fromCubemap(t,e=null){return this._fromTexture(t,e)}compileCubemapShader(){this._cubemapMaterial===null&&(this._cubemapMaterial=Po(),this._compileMaterial(this._cubemapMaterial))}compileEquirectangularShader(){this._equirectMaterial===null&&(this._equirectMaterial=Ro(),this._compileMaterial(this._equirectMaterial))}dispose(){this._dispose(),this._cubemapMaterial!==null&&this._cubemapMaterial.dispose(),this._equirectMaterial!==null&&this._equirectMaterial.dispose()}_setSize(t){this._lodMax=Math.floor(Math.log2(t)),this._cubeSize=Math.pow(2,this._lodMax)}_dispose(){this._blurMaterial!==null&&this._blurMaterial.dispose(),this._pingPongRenderTarget!==null&&this._pingPongRenderTarget.dispose();for(let t=0;t<this._lodPlanes.length;t++)this._lodPlanes[t].dispose()}_cleanup(t){this._renderer.setRenderTarget(Cs,Rs,Ps),this._renderer.xr.enabled=Ls,t.scissorTest=!1,xr(t,0,0,t.width,t.height)}_fromTexture(t,e){t.mapping===yi||t.mapping===Mi?this._setSize(t.image.length===0?16:t.image[0].width||t.image[0].image.width):this._setSize(t.image.width/4),Cs=this._renderer.getRenderTarget(),Rs=this._renderer.getActiveCubeFace(),Ps=this._renderer.getActiveMipmapLevel(),Ls=this._renderer.xr.enabled,this._renderer.xr.enabled=!1;const i=e||this._allocateTargets();return this._textureToCubeUV(t,i),this._applyPMREM(i),this._cleanup(i),i}_allocateTargets(){const t=3*Math.max(this._cubeSize,112),e=4*this._cubeSize,i={magFilter:Be,minFilter:Be,generateMipmaps:!1,type:qi,format:He,colorSpace:Rn,depthBuffer:!1},r=Co(t,e,i);if(this._pingPongRenderTarget===null||this._pingPongRenderTarget.width!==t||this._pingPongRenderTarget.height!==e){this._pingPongRenderTarget!==null&&this._dispose(),this._pingPongRenderTarget=Co(t,e,i);const{_lodMax:s}=this;({sizeLods:this._sizeLods,lodPlanes:this._lodPlanes,sigmas:this._sigmas}=ap(s)),this._blurMaterial=op(s,t,e)}return r}_compileMaterial(t){const e=new ae(this._lodPlanes[0],t);this._renderer.compile(e,bs)}_sceneToCubeUV(t,e,i,r){const a=new Ie(90,1,e,i),c=[1,-1,1,1,1,1],u=[1,1,1,-1,-1,-1],f=this._renderer,p=f.autoClear,m=f.toneMapping;f.getClearColor(wo),f.toneMapping=wn,f.autoClear=!1;const g=new sn({name:"PMREM.Background",side:Se,depthWrite:!1,depthTest:!1}),v=new ae(new Xn,g);let M=!1;const _=t.background;_?_.isColor&&(g.color.copy(_),t.background=null,M=!0):(g.color.copy(wo),M=!0);for(let d=0;d<6;d++){const C=d%3;C===0?(a.up.set(0,c[d],0),a.lookAt(u[d],0,0)):C===1?(a.up.set(0,0,c[d]),a.lookAt(0,u[d],0)):(a.up.set(0,c[d],0),a.lookAt(0,0,u[d]));const w=this._cubeSize;xr(r,C*w,d>2?w:0,w,w),f.setRenderTarget(r),M&&f.render(v,a),f.render(t,a)}v.geometry.dispose(),v.material.dispose(),f.toneMapping=m,f.autoClear=p,t.background=_}_textureToCubeUV(t,e){const i=this._renderer,r=t.mapping===yi||t.mapping===Mi;r?(this._cubemapMaterial===null&&(this._cubemapMaterial=Po()),this._cubemapMaterial.uniforms.flipEnvMap.value=t.isRenderTargetTexture===!1?-1:1):this._equirectMaterial===null&&(this._equirectMaterial=Ro());const s=r?this._cubemapMaterial:this._equirectMaterial,o=new ae(this._lodPlanes[0],s),a=s.uniforms;a.envMap.value=t;const c=this._cubeSize;xr(e,0,0,3*c,2*c),i.setRenderTarget(e),i.render(o,bs)}_applyPMREM(t){const e=this._renderer,i=e.autoClear;e.autoClear=!1;const r=this._lodPlanes.length;for(let s=1;s<r;s++){const o=Math.sqrt(this._sigmas[s]*this._sigmas[s]-this._sigmas[s-1]*this._sigmas[s-1]),a=Ao[(r-s-1)%Ao.length];this._blur(t,s-1,s,o,a)}e.autoClear=i}_blur(t,e,i,r,s){const o=this._pingPongRenderTarget;this._halfBlur(t,o,e,i,r,"latitudinal",s),this._halfBlur(o,t,i,i,r,"longitudinal",s)}_halfBlur(t,e,i,r,s,o,a){const c=this._renderer,u=this._blurMaterial;o!=="latitudinal"&&o!=="longitudinal"&&console.error("blur direction must be either latitudinal or longitudinal!");const f=3,p=new ae(this._lodPlanes[r],u),m=u.uniforms,g=this._sizeLods[i]-1,v=isFinite(s)?Math.PI/(2*g):2*Math.PI/(2*kn-1),M=s/v,_=isFinite(s)?1+Math.floor(f*M):kn;_>kn&&console.warn(`sigmaRadians, ${s}, is too large and will clip, as it requested ${_} samples when the maximum is set to ${kn}`);const d=[];let C=0;for(let I=0;I<kn;++I){const F=I/M,A=Math.exp(-F*F/2);d.push(A),I===0?C+=A:I<_&&(C+=2*A)}for(let I=0;I<d.length;I++)d[I]=d[I]/C;m.envMap.value=t.texture,m.samples.value=_,m.weights.value=d,m.latitudinal.value=o==="latitudinal",a&&(m.poleAxis.value=a);const{_lodMax:w}=this;m.dTheta.value=v,m.mipInt.value=w-i;const b=this._sizeLods[r],z=3*b*(r>w-_i?r-w+_i:0),P=4*(this._cubeSize-b);xr(e,z,P,3*b,2*b),c.setRenderTarget(e),c.render(p,bs)}}function ap(n){const t=[],e=[],i=[];let r=n;const s=n-_i+1+To.length;for(let o=0;o<s;o++){const a=Math.pow(2,r);e.push(a);let c=1/a;o>n-_i?c=To[o-n+_i-1]:o===0&&(c=0),i.push(c);const u=1/(a-2),f=-u,p=1+u,m=[f,f,p,f,p,p,f,f,p,p,f,p],g=6,v=6,M=3,_=2,d=1,C=new Float32Array(M*v*g),w=new Float32Array(_*v*g),b=new Float32Array(d*v*g);for(let P=0;P<g;P++){const I=P%3*2/3-1,F=P>2?0:-1,A=[I,F,0,I+2/3,F,0,I+2/3,F+1,0,I,F,0,I+2/3,F+1,0,I,F+1,0];C.set(A,M*v*P),w.set(m,_*v*P);const E=[P,P,P,P,P,P];b.set(E,d*v*P)}const z=new Ce;z.setAttribute("position",new ke(C,M)),z.setAttribute("uv",new ke(w,_)),z.setAttribute("faceIndex",new ke(b,d)),t.push(z),r>_i&&r--}return{lodPlanes:t,sizeLods:e,sigmas:i}}function Co(n,t,e){const i=new qn(n,t,e);return i.texture.mapping=Xr,i.texture.name="PMREM.cubeUv",i.scissorTest=!0,i}function xr(n,t,e,i,r){n.viewport.set(t,e,i,r),n.scissor.set(t,e,i,r)}function op(n,t,e){const i=new Float32Array(kn),r=new k(0,1,0);return new bn({name:"SphericalGaussianBlur",defines:{n:kn,CUBEUV_TEXEL_WIDTH:1/t,CUBEUV_TEXEL_HEIGHT:1/e,CUBEUV_MAX_MIP:`${n}.0`},uniforms:{envMap:{value:null},samples:{value:1},weights:{value:i},latitudinal:{value:!1},dTheta:{value:0},mipInt:{value:0},poleAxis:{value:r}},vertexShader:Oa(),fragmentShader:`

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
		`,blending:Tn,depthTest:!1,depthWrite:!1})}function Ro(){return new bn({name:"EquirectangularToCubeUV",uniforms:{envMap:{value:null}},vertexShader:Oa(),fragmentShader:`

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
		`,blending:Tn,depthTest:!1,depthWrite:!1})}function Po(){return new bn({name:"CubemapToCubeUV",uniforms:{envMap:{value:null},flipEnvMap:{value:-1}},vertexShader:Oa(),fragmentShader:`

			precision mediump float;
			precision mediump int;

			uniform float flipEnvMap;

			varying vec3 vOutputDirection;

			uniform samplerCube envMap;

			void main() {

				gl_FragColor = textureCube( envMap, vec3( flipEnvMap * vOutputDirection.x, vOutputDirection.yz ) );

			}
		`,blending:Tn,depthTest:!1,depthWrite:!1})}function Oa(){return`

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
	`}function cp(n){let t=new WeakMap,e=null;function i(a){if(a&&a.isTexture){const c=a.mapping,u=c===Vs||c===Ws,f=c===yi||c===Mi;if(u||f){let p=t.get(a);const m=p!==void 0?p.texture.pmremVersion:0;if(a.isRenderTargetTexture&&a.pmremVersion!==m)return e===null&&(e=new bo(n)),p=u?e.fromEquirectangular(a,p):e.fromCubemap(a,p),p.texture.pmremVersion=a.pmremVersion,t.set(a,p),p.texture;if(p!==void 0)return p.texture;{const g=a.image;return u&&g&&g.height>0||f&&g&&r(g)?(e===null&&(e=new bo(n)),p=u?e.fromEquirectangular(a):e.fromCubemap(a),p.texture.pmremVersion=a.pmremVersion,t.set(a,p),a.addEventListener("dispose",s),p.texture):null}}}return a}function r(a){let c=0;const u=6;for(let f=0;f<u;f++)a[f]!==void 0&&c++;return c===u}function s(a){const c=a.target;c.removeEventListener("dispose",s);const u=t.get(c);u!==void 0&&(t.delete(c),u.dispose())}function o(){t=new WeakMap,e!==null&&(e.dispose(),e=null)}return{get:i,dispose:o}}function lp(n){const t={};function e(i){if(t[i]!==void 0)return t[i];let r;switch(i){case"WEBGL_depth_texture":r=n.getExtension("WEBGL_depth_texture")||n.getExtension("MOZ_WEBGL_depth_texture")||n.getExtension("WEBKIT_WEBGL_depth_texture");break;case"EXT_texture_filter_anisotropic":r=n.getExtension("EXT_texture_filter_anisotropic")||n.getExtension("MOZ_EXT_texture_filter_anisotropic")||n.getExtension("WEBKIT_EXT_texture_filter_anisotropic");break;case"WEBGL_compressed_texture_s3tc":r=n.getExtension("WEBGL_compressed_texture_s3tc")||n.getExtension("MOZ_WEBGL_compressed_texture_s3tc")||n.getExtension("WEBKIT_WEBGL_compressed_texture_s3tc");break;case"WEBGL_compressed_texture_pvrtc":r=n.getExtension("WEBGL_compressed_texture_pvrtc")||n.getExtension("WEBKIT_WEBGL_compressed_texture_pvrtc");break;default:r=n.getExtension(i)}return t[i]=r,r}return{has:function(i){return e(i)!==null},init:function(){e("EXT_color_buffer_float"),e("WEBGL_clip_cull_distance"),e("OES_texture_float_linear"),e("EXT_color_buffer_half_float"),e("WEBGL_multisampled_render_to_texture"),e("WEBGL_render_shared_exponent")},get:function(i){const r=e(i);return r===null&&Hi("THREE.WebGLRenderer: "+i+" extension not supported."),r}}}function up(n,t,e,i){const r={},s=new WeakMap;function o(p){const m=p.target;m.index!==null&&t.remove(m.index);for(const v in m.attributes)t.remove(m.attributes[v]);for(const v in m.morphAttributes){const M=m.morphAttributes[v];for(let _=0,d=M.length;_<d;_++)t.remove(M[_])}m.removeEventListener("dispose",o),delete r[m.id];const g=s.get(m);g&&(t.remove(g),s.delete(m)),i.releaseStatesOfGeometry(m),m.isInstancedBufferGeometry===!0&&delete m._maxInstanceCount,e.memory.geometries--}function a(p,m){return r[m.id]===!0||(m.addEventListener("dispose",o),r[m.id]=!0,e.memory.geometries++),m}function c(p){const m=p.attributes;for(const v in m)t.update(m[v],n.ARRAY_BUFFER);const g=p.morphAttributes;for(const v in g){const M=g[v];for(let _=0,d=M.length;_<d;_++)t.update(M[_],n.ARRAY_BUFFER)}}function u(p){const m=[],g=p.index,v=p.attributes.position;let M=0;if(g!==null){const C=g.array;M=g.version;for(let w=0,b=C.length;w<b;w+=3){const z=C[w+0],P=C[w+1],I=C[w+2];m.push(z,P,P,I,I,z)}}else if(v!==void 0){const C=v.array;M=v.version;for(let w=0,b=C.length/3-1;w<b;w+=3){const z=w+0,P=w+1,I=w+2;m.push(z,P,P,I,I,z)}}else return;const _=new(Vc(m)?jc:Kc)(m,1);_.version=M;const d=s.get(p);d&&t.remove(d),s.set(p,_)}function f(p){const m=s.get(p);if(m){const g=p.index;g!==null&&m.version<g.version&&u(p)}else u(p);return s.get(p)}return{get:a,update:c,getWireframeAttribute:f}}function hp(n,t,e){let i;function r(m){i=m}let s,o;function a(m){s=m.type,o=m.bytesPerElement}function c(m,g){n.drawElements(i,g,s,m*o),e.update(g,i,1)}function u(m,g,v){v!==0&&(n.drawElementsInstanced(i,g,s,m*o,v),e.update(g,i,v))}function f(m,g,v){if(v===0)return;t.get("WEBGL_multi_draw").multiDrawElementsWEBGL(i,g,0,s,m,0,v);let _=0;for(let d=0;d<v;d++)_+=g[d];e.update(_,i,1)}function p(m,g,v,M){if(v===0)return;const _=t.get("WEBGL_multi_draw");if(_===null)for(let d=0;d<m.length;d++)u(m[d]/o,g[d],M[d]);else{_.multiDrawElementsInstancedWEBGL(i,g,0,s,m,0,M,0,v);let d=0;for(let C=0;C<v;C++)d+=g[C];for(let C=0;C<M.length;C++)e.update(d,i,M[C])}}this.setMode=r,this.setIndex=a,this.render=c,this.renderInstances=u,this.renderMultiDraw=f,this.renderMultiDrawInstances=p}function fp(n){const t={geometries:0,textures:0},e={frame:0,calls:0,triangles:0,points:0,lines:0};function i(s,o,a){switch(e.calls++,o){case n.TRIANGLES:e.triangles+=a*(s/3);break;case n.LINES:e.lines+=a*(s/2);break;case n.LINE_STRIP:e.lines+=a*(s-1);break;case n.LINE_LOOP:e.lines+=a*s;break;case n.POINTS:e.points+=a*s;break;default:console.error("THREE.WebGLInfo: Unknown draw mode:",o);break}}function r(){e.calls=0,e.triangles=0,e.points=0,e.lines=0}return{memory:t,render:e,programs:null,autoReset:!0,reset:r,update:i}}function dp(n,t,e){const i=new WeakMap,r=new se;function s(o,a,c){const u=o.morphTargetInfluences,f=a.morphAttributes.position||a.morphAttributes.normal||a.morphAttributes.color,p=f!==void 0?f.length:0;let m=i.get(a);if(m===void 0||m.count!==p){let A=function(){I.dispose(),i.delete(a),a.removeEventListener("dispose",A)};m!==void 0&&m.texture.dispose();const g=a.morphAttributes.position!==void 0,v=a.morphAttributes.normal!==void 0,M=a.morphAttributes.color!==void 0,_=a.morphAttributes.position||[],d=a.morphAttributes.normal||[],C=a.morphAttributes.color||[];let w=0;g===!0&&(w=1),v===!0&&(w=2),M===!0&&(w=3);let b=a.attributes.position.count*w,z=1;b>t.maxTextureSize&&(z=Math.ceil(b/t.maxTextureSize),b=t.maxTextureSize);const P=new Float32Array(b*z*4*p),I=new qc(P,b,z,p);I.type=nn,I.needsUpdate=!0;const F=w*4;for(let E=0;E<p;E++){const N=_[E],$=d[E],W=C[E],Y=b*z*4*E;for(let tt=0;tt<N.count;tt++){const K=tt*F;g===!0&&(r.fromBufferAttribute(N,tt),P[Y+K+0]=r.x,P[Y+K+1]=r.y,P[Y+K+2]=r.z,P[Y+K+3]=0),v===!0&&(r.fromBufferAttribute($,tt),P[Y+K+4]=r.x,P[Y+K+5]=r.y,P[Y+K+6]=r.z,P[Y+K+7]=0),M===!0&&(r.fromBufferAttribute(W,tt),P[Y+K+8]=r.x,P[Y+K+9]=r.y,P[Y+K+10]=r.z,P[Y+K+11]=W.itemSize===4?r.w:1)}}m={count:p,texture:I,size:new Bt(b,z)},i.set(a,m),a.addEventListener("dispose",A)}if(o.isInstancedMesh===!0&&o.morphTexture!==null)c.getUniforms().setValue(n,"morphTexture",o.morphTexture,e);else{let g=0;for(let M=0;M<u.length;M++)g+=u[M];const v=a.morphTargetsRelative?1:1-g;c.getUniforms().setValue(n,"morphTargetBaseInfluence",v),c.getUniforms().setValue(n,"morphTargetInfluences",u)}c.getUniforms().setValue(n,"morphTargetsTexture",m.texture,e),c.getUniforms().setValue(n,"morphTargetsTextureSize",m.size)}return{update:s}}function pp(n,t,e,i){let r=new WeakMap;function s(c){const u=i.render.frame,f=c.geometry,p=t.get(c,f);if(r.get(p)!==u&&(t.update(p),r.set(p,u)),c.isInstancedMesh&&(c.hasEventListener("dispose",a)===!1&&c.addEventListener("dispose",a),r.get(c)!==u&&(e.update(c.instanceMatrix,n.ARRAY_BUFFER),c.instanceColor!==null&&e.update(c.instanceColor,n.ARRAY_BUFFER),r.set(c,u))),c.isSkinnedMesh){const m=c.skeleton;r.get(m)!==u&&(m.update(),r.set(m,u))}return p}function o(){r=new WeakMap}function a(c){const u=c.target;u.removeEventListener("dispose",a),e.remove(u.instanceMatrix),u.instanceColor!==null&&e.remove(u.instanceColor)}return{update:s,dispose:o}}class nl extends Ee{constructor(t,e,i,r,s,o,a,c,u,f=xi){if(f!==xi&&f!==Ei)throw new Error("DepthTexture format must be either THREE.DepthFormat or THREE.DepthStencilFormat");i===void 0&&f===xi&&(i=Wn),i===void 0&&f===Ei&&(i=Si),super(null,r,s,o,a,c,f,i,u),this.isDepthTexture=!0,this.image={width:t,height:e},this.magFilter=a!==void 0?a:De,this.minFilter=c!==void 0?c:De,this.flipY=!1,this.generateMipmaps=!1,this.compareFunction=null}copy(t){return super.copy(t),this.compareFunction=t.compareFunction,this}toJSON(t){const e=super.toJSON(t);return this.compareFunction!==null&&(e.compareFunction=this.compareFunction),e}}const il=new Ee,Lo=new nl(1,1),rl=new qc,sl=new Qu,al=new Qc,Io=[],Do=[],Uo=new Float32Array(16),No=new Float32Array(9),Fo=new Float32Array(4);function bi(n,t,e){const i=n[0];if(i<=0||i>0)return n;const r=t*e;let s=Io[r];if(s===void 0&&(s=new Float32Array(r),Io[r]=s),t!==0){i.toArray(s,0);for(let o=1,a=0;o!==t;++o)a+=e,n[o].toArray(s,a)}return s}function oe(n,t){if(n.length!==t.length)return!1;for(let e=0,i=n.length;e<i;e++)if(n[e]!==t[e])return!1;return!0}function ce(n,t){for(let e=0,i=t.length;e<i;e++)n[e]=t[e]}function Kr(n,t){let e=Do[t];e===void 0&&(e=new Int32Array(t),Do[t]=e);for(let i=0;i!==t;++i)e[i]=n.allocateTextureUnit();return e}function mp(n,t){const e=this.cache;e[0]!==t&&(n.uniform1f(this.addr,t),e[0]=t)}function _p(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y)&&(n.uniform2f(this.addr,t.x,t.y),e[0]=t.x,e[1]=t.y);else{if(oe(e,t))return;n.uniform2fv(this.addr,t),ce(e,t)}}function gp(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z)&&(n.uniform3f(this.addr,t.x,t.y,t.z),e[0]=t.x,e[1]=t.y,e[2]=t.z);else if(t.r!==void 0)(e[0]!==t.r||e[1]!==t.g||e[2]!==t.b)&&(n.uniform3f(this.addr,t.r,t.g,t.b),e[0]=t.r,e[1]=t.g,e[2]=t.b);else{if(oe(e,t))return;n.uniform3fv(this.addr,t),ce(e,t)}}function xp(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z||e[3]!==t.w)&&(n.uniform4f(this.addr,t.x,t.y,t.z,t.w),e[0]=t.x,e[1]=t.y,e[2]=t.z,e[3]=t.w);else{if(oe(e,t))return;n.uniform4fv(this.addr,t),ce(e,t)}}function vp(n,t){const e=this.cache,i=t.elements;if(i===void 0){if(oe(e,t))return;n.uniformMatrix2fv(this.addr,!1,t),ce(e,t)}else{if(oe(e,i))return;Fo.set(i),n.uniformMatrix2fv(this.addr,!1,Fo),ce(e,i)}}function yp(n,t){const e=this.cache,i=t.elements;if(i===void 0){if(oe(e,t))return;n.uniformMatrix3fv(this.addr,!1,t),ce(e,t)}else{if(oe(e,i))return;No.set(i),n.uniformMatrix3fv(this.addr,!1,No),ce(e,i)}}function Mp(n,t){const e=this.cache,i=t.elements;if(i===void 0){if(oe(e,t))return;n.uniformMatrix4fv(this.addr,!1,t),ce(e,t)}else{if(oe(e,i))return;Uo.set(i),n.uniformMatrix4fv(this.addr,!1,Uo),ce(e,i)}}function Sp(n,t){const e=this.cache;e[0]!==t&&(n.uniform1i(this.addr,t),e[0]=t)}function Ep(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y)&&(n.uniform2i(this.addr,t.x,t.y),e[0]=t.x,e[1]=t.y);else{if(oe(e,t))return;n.uniform2iv(this.addr,t),ce(e,t)}}function Tp(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z)&&(n.uniform3i(this.addr,t.x,t.y,t.z),e[0]=t.x,e[1]=t.y,e[2]=t.z);else{if(oe(e,t))return;n.uniform3iv(this.addr,t),ce(e,t)}}function wp(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z||e[3]!==t.w)&&(n.uniform4i(this.addr,t.x,t.y,t.z,t.w),e[0]=t.x,e[1]=t.y,e[2]=t.z,e[3]=t.w);else{if(oe(e,t))return;n.uniform4iv(this.addr,t),ce(e,t)}}function Ap(n,t){const e=this.cache;e[0]!==t&&(n.uniform1ui(this.addr,t),e[0]=t)}function bp(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y)&&(n.uniform2ui(this.addr,t.x,t.y),e[0]=t.x,e[1]=t.y);else{if(oe(e,t))return;n.uniform2uiv(this.addr,t),ce(e,t)}}function Cp(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z)&&(n.uniform3ui(this.addr,t.x,t.y,t.z),e[0]=t.x,e[1]=t.y,e[2]=t.z);else{if(oe(e,t))return;n.uniform3uiv(this.addr,t),ce(e,t)}}function Rp(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z||e[3]!==t.w)&&(n.uniform4ui(this.addr,t.x,t.y,t.z,t.w),e[0]=t.x,e[1]=t.y,e[2]=t.z,e[3]=t.w);else{if(oe(e,t))return;n.uniform4uiv(this.addr,t),ce(e,t)}}function Pp(n,t,e){const i=this.cache,r=e.allocateTextureUnit();i[0]!==r&&(n.uniform1i(this.addr,r),i[0]=r);let s;this.type===n.SAMPLER_2D_SHADOW?(Lo.compareFunction=Gc,s=Lo):s=il,e.setTexture2D(t||s,r)}function Lp(n,t,e){const i=this.cache,r=e.allocateTextureUnit();i[0]!==r&&(n.uniform1i(this.addr,r),i[0]=r),e.setTexture3D(t||sl,r)}function Ip(n,t,e){const i=this.cache,r=e.allocateTextureUnit();i[0]!==r&&(n.uniform1i(this.addr,r),i[0]=r),e.setTextureCube(t||al,r)}function Dp(n,t,e){const i=this.cache,r=e.allocateTextureUnit();i[0]!==r&&(n.uniform1i(this.addr,r),i[0]=r),e.setTexture2DArray(t||rl,r)}function Up(n){switch(n){case 5126:return mp;case 35664:return _p;case 35665:return gp;case 35666:return xp;case 35674:return vp;case 35675:return yp;case 35676:return Mp;case 5124:case 35670:return Sp;case 35667:case 35671:return Ep;case 35668:case 35672:return Tp;case 35669:case 35673:return wp;case 5125:return Ap;case 36294:return bp;case 36295:return Cp;case 36296:return Rp;case 35678:case 36198:case 36298:case 36306:case 35682:return Pp;case 35679:case 36299:case 36307:return Lp;case 35680:case 36300:case 36308:case 36293:return Ip;case 36289:case 36303:case 36311:case 36292:return Dp}}function Np(n,t){n.uniform1fv(this.addr,t)}function Fp(n,t){const e=bi(t,this.size,2);n.uniform2fv(this.addr,e)}function Op(n,t){const e=bi(t,this.size,3);n.uniform3fv(this.addr,e)}function Bp(n,t){const e=bi(t,this.size,4);n.uniform4fv(this.addr,e)}function zp(n,t){const e=bi(t,this.size,4);n.uniformMatrix2fv(this.addr,!1,e)}function Hp(n,t){const e=bi(t,this.size,9);n.uniformMatrix3fv(this.addr,!1,e)}function kp(n,t){const e=bi(t,this.size,16);n.uniformMatrix4fv(this.addr,!1,e)}function Gp(n,t){n.uniform1iv(this.addr,t)}function Vp(n,t){n.uniform2iv(this.addr,t)}function Wp(n,t){n.uniform3iv(this.addr,t)}function qp(n,t){n.uniform4iv(this.addr,t)}function Xp(n,t){n.uniform1uiv(this.addr,t)}function $p(n,t){n.uniform2uiv(this.addr,t)}function Yp(n,t){n.uniform3uiv(this.addr,t)}function Kp(n,t){n.uniform4uiv(this.addr,t)}function jp(n,t,e){const i=this.cache,r=t.length,s=Kr(e,r);oe(i,s)||(n.uniform1iv(this.addr,s),ce(i,s));for(let o=0;o!==r;++o)e.setTexture2D(t[o]||il,s[o])}function Zp(n,t,e){const i=this.cache,r=t.length,s=Kr(e,r);oe(i,s)||(n.uniform1iv(this.addr,s),ce(i,s));for(let o=0;o!==r;++o)e.setTexture3D(t[o]||sl,s[o])}function Jp(n,t,e){const i=this.cache,r=t.length,s=Kr(e,r);oe(i,s)||(n.uniform1iv(this.addr,s),ce(i,s));for(let o=0;o!==r;++o)e.setTextureCube(t[o]||al,s[o])}function Qp(n,t,e){const i=this.cache,r=t.length,s=Kr(e,r);oe(i,s)||(n.uniform1iv(this.addr,s),ce(i,s));for(let o=0;o!==r;++o)e.setTexture2DArray(t[o]||rl,s[o])}function tm(n){switch(n){case 5126:return Np;case 35664:return Fp;case 35665:return Op;case 35666:return Bp;case 35674:return zp;case 35675:return Hp;case 35676:return kp;case 5124:case 35670:return Gp;case 35667:case 35671:return Vp;case 35668:case 35672:return Wp;case 35669:case 35673:return qp;case 5125:return Xp;case 36294:return $p;case 36295:return Yp;case 36296:return Kp;case 35678:case 36198:case 36298:case 36306:case 35682:return jp;case 35679:case 36299:case 36307:return Zp;case 35680:case 36300:case 36308:case 36293:return Jp;case 36289:case 36303:case 36311:case 36292:return Qp}}class em{constructor(t,e,i){this.id=t,this.addr=i,this.cache=[],this.type=e.type,this.setValue=Up(e.type)}}class nm{constructor(t,e,i){this.id=t,this.addr=i,this.cache=[],this.type=e.type,this.size=e.size,this.setValue=tm(e.type)}}class im{constructor(t){this.id=t,this.seq=[],this.map={}}setValue(t,e,i){const r=this.seq;for(let s=0,o=r.length;s!==o;++s){const a=r[s];a.setValue(t,e[a.id],i)}}}const Is=/(\w+)(\])?(\[|\.)?/g;function Oo(n,t){n.seq.push(t),n.map[t.id]=t}function rm(n,t,e){const i=n.name,r=i.length;for(Is.lastIndex=0;;){const s=Is.exec(i),o=Is.lastIndex;let a=s[1];const c=s[2]==="]",u=s[3];if(c&&(a=a|0),u===void 0||u==="["&&o+2===r){Oo(e,u===void 0?new em(a,n,t):new nm(a,n,t));break}else{let p=e.map[a];p===void 0&&(p=new im(a),Oo(e,p)),e=p}}}class Dr{constructor(t,e){this.seq=[],this.map={};const i=t.getProgramParameter(e,t.ACTIVE_UNIFORMS);for(let r=0;r<i;++r){const s=t.getActiveUniform(e,r),o=t.getUniformLocation(e,s.name);rm(s,o,this)}}setValue(t,e,i,r){const s=this.map[e];s!==void 0&&s.setValue(t,i,r)}setOptional(t,e,i){const r=e[i];r!==void 0&&this.setValue(t,i,r)}static upload(t,e,i,r){for(let s=0,o=e.length;s!==o;++s){const a=e[s],c=i[a.id];c.needsUpdate!==!1&&a.setValue(t,c.value,r)}}static seqWithValue(t,e){const i=[];for(let r=0,s=t.length;r!==s;++r){const o=t[r];o.id in e&&i.push(o)}return i}}function Bo(n,t,e){const i=n.createShader(t);return n.shaderSource(i,e),n.compileShader(i),i}const sm=37297;let am=0;function om(n,t){const e=n.split(`
`),i=[],r=Math.max(t-6,0),s=Math.min(t+6,e.length);for(let o=r;o<s;o++){const a=o+1;i.push(`${a===t?">":" "} ${a}: ${e[o]}`)}return i.join(`
`)}function cm(n){const t=$t.getPrimaries($t.workingColorSpace),e=$t.getPrimaries(n);let i;switch(t===e?i="":t===Br&&e===Or?i="LinearDisplayP3ToLinearSRGB":t===Or&&e===Br&&(i="LinearSRGBToLinearDisplayP3"),n){case Rn:case $r:return[i,"LinearTransferOETF"];case We:case Na:return[i,"sRGBTransferOETF"];default:return console.warn("THREE.WebGLProgram: Unsupported color space:",n),[i,"LinearTransferOETF"]}}function zo(n,t,e){const i=n.getShaderParameter(t,n.COMPILE_STATUS),r=n.getShaderInfoLog(t).trim();if(i&&r==="")return"";const s=/ERROR: 0:(\d+)/.exec(r);if(s){const o=parseInt(s[1]);return e.toUpperCase()+`

`+r+`

`+om(n.getShaderSource(t),o)}else return r}function lm(n,t){const e=cm(t);return`vec4 ${n}( vec4 value ) { return ${e[0]}( ${e[1]}( value ) ); }`}function um(n,t){let e;switch(t){case bu:e="Linear";break;case Cu:e="Reinhard";break;case Ru:e="OptimizedCineon";break;case Pu:e="ACESFilmic";break;case Iu:e="AgX";break;case Du:e="Neutral";break;case Lu:e="Custom";break;default:console.warn("THREE.WebGLProgram: Unsupported toneMapping:",t),e="Linear"}return"vec3 "+n+"( vec3 color ) { return "+e+"ToneMapping( color ); }"}const vr=new k;function hm(){$t.getLuminanceCoefficients(vr);const n=vr.x.toFixed(4),t=vr.y.toFixed(4),e=vr.z.toFixed(4);return["float luminance( const in vec3 rgb ) {",`	const vec3 weights = vec3( ${n}, ${t}, ${e} );`,"	return dot( weights, rgb );","}"].join(`
`)}function fm(n){return[n.extensionClipCullDistance?"#extension GL_ANGLE_clip_cull_distance : require":"",n.extensionMultiDraw?"#extension GL_ANGLE_multi_draw : require":""].filter(Bi).join(`
`)}function dm(n){const t=[];for(const e in n){const i=n[e];i!==!1&&t.push("#define "+e+" "+i)}return t.join(`
`)}function pm(n,t){const e={},i=n.getProgramParameter(t,n.ACTIVE_ATTRIBUTES);for(let r=0;r<i;r++){const s=n.getActiveAttrib(t,r),o=s.name;let a=1;s.type===n.FLOAT_MAT2&&(a=2),s.type===n.FLOAT_MAT3&&(a=3),s.type===n.FLOAT_MAT4&&(a=4),e[o]={type:s.type,location:n.getAttribLocation(t,o),locationSize:a}}return e}function Bi(n){return n!==""}function Ho(n,t){const e=t.numSpotLightShadows+t.numSpotLightMaps-t.numSpotLightShadowsWithMaps;return n.replace(/NUM_DIR_LIGHTS/g,t.numDirLights).replace(/NUM_SPOT_LIGHTS/g,t.numSpotLights).replace(/NUM_SPOT_LIGHT_MAPS/g,t.numSpotLightMaps).replace(/NUM_SPOT_LIGHT_COORDS/g,e).replace(/NUM_RECT_AREA_LIGHTS/g,t.numRectAreaLights).replace(/NUM_POINT_LIGHTS/g,t.numPointLights).replace(/NUM_HEMI_LIGHTS/g,t.numHemiLights).replace(/NUM_DIR_LIGHT_SHADOWS/g,t.numDirLightShadows).replace(/NUM_SPOT_LIGHT_SHADOWS_WITH_MAPS/g,t.numSpotLightShadowsWithMaps).replace(/NUM_SPOT_LIGHT_SHADOWS/g,t.numSpotLightShadows).replace(/NUM_POINT_LIGHT_SHADOWS/g,t.numPointLightShadows)}function ko(n,t){return n.replace(/NUM_CLIPPING_PLANES/g,t.numClippingPlanes).replace(/UNION_CLIPPING_PLANES/g,t.numClippingPlanes-t.numClipIntersection)}const mm=/^[ \t]*#include +<([\w\d./]+)>/gm;function ya(n){return n.replace(mm,gm)}const _m=new Map;function gm(n,t){let e=Ut[t];if(e===void 0){const i=_m.get(t);if(i!==void 0)e=Ut[i],console.warn('THREE.WebGLRenderer: Shader chunk "%s" has been deprecated. Use "%s" instead.',t,i);else throw new Error("Can not resolve #include <"+t+">")}return ya(e)}const xm=/#pragma unroll_loop_start\s+for\s*\(\s*int\s+i\s*=\s*(\d+)\s*;\s*i\s*<\s*(\d+)\s*;\s*i\s*\+\+\s*\)\s*{([\s\S]+?)}\s+#pragma unroll_loop_end/g;function Go(n){return n.replace(xm,vm)}function vm(n,t,e,i){let r="";for(let s=parseInt(t);s<parseInt(e);s++)r+=i.replace(/\[\s*i\s*\]/g,"[ "+s+" ]").replace(/UNROLLED_LOOP_INDEX/g,s);return r}function Vo(n){let t=`precision ${n.precision} float;
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
#define LOW_PRECISION`),t}function ym(n){let t="SHADOWMAP_TYPE_BASIC";return n.shadowMapType===Rc?t="SHADOWMAP_TYPE_PCF":n.shadowMapType===Ql?t="SHADOWMAP_TYPE_PCF_SOFT":n.shadowMapType===en&&(t="SHADOWMAP_TYPE_VSM"),t}function Mm(n){let t="ENVMAP_TYPE_CUBE";if(n.envMap)switch(n.envMapMode){case yi:case Mi:t="ENVMAP_TYPE_CUBE";break;case Xr:t="ENVMAP_TYPE_CUBE_UV";break}return t}function Sm(n){let t="ENVMAP_MODE_REFLECTION";if(n.envMap)switch(n.envMapMode){case Mi:t="ENVMAP_MODE_REFRACTION";break}return t}function Em(n){let t="ENVMAP_BLENDING_NONE";if(n.envMap)switch(n.combine){case Pc:t="ENVMAP_BLENDING_MULTIPLY";break;case wu:t="ENVMAP_BLENDING_MIX";break;case Au:t="ENVMAP_BLENDING_ADD";break}return t}function Tm(n){const t=n.envMapCubeUVHeight;if(t===null)return null;const e=Math.log2(t)-2,i=1/t;return{texelWidth:1/(3*Math.max(Math.pow(2,e),7*16)),texelHeight:i,maxMip:e}}function wm(n,t,e,i){const r=n.getContext(),s=e.defines;let o=e.vertexShader,a=e.fragmentShader;const c=ym(e),u=Mm(e),f=Sm(e),p=Em(e),m=Tm(e),g=fm(e),v=dm(s),M=r.createProgram();let _,d,C=e.glslVersion?"#version "+e.glslVersion+`
`:"";e.isRawShaderMaterial?(_=["#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,v].filter(Bi).join(`
`),_.length>0&&(_+=`
`),d=["#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,v].filter(Bi).join(`
`),d.length>0&&(d+=`
`)):(_=[Vo(e),"#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,v,e.extensionClipCullDistance?"#define USE_CLIP_DISTANCE":"",e.batching?"#define USE_BATCHING":"",e.batchingColor?"#define USE_BATCHING_COLOR":"",e.instancing?"#define USE_INSTANCING":"",e.instancingColor?"#define USE_INSTANCING_COLOR":"",e.instancingMorph?"#define USE_INSTANCING_MORPH":"",e.useFog&&e.fog?"#define USE_FOG":"",e.useFog&&e.fogExp2?"#define FOG_EXP2":"",e.map?"#define USE_MAP":"",e.envMap?"#define USE_ENVMAP":"",e.envMap?"#define "+f:"",e.lightMap?"#define USE_LIGHTMAP":"",e.aoMap?"#define USE_AOMAP":"",e.bumpMap?"#define USE_BUMPMAP":"",e.normalMap?"#define USE_NORMALMAP":"",e.normalMapObjectSpace?"#define USE_NORMALMAP_OBJECTSPACE":"",e.normalMapTangentSpace?"#define USE_NORMALMAP_TANGENTSPACE":"",e.displacementMap?"#define USE_DISPLACEMENTMAP":"",e.emissiveMap?"#define USE_EMISSIVEMAP":"",e.anisotropy?"#define USE_ANISOTROPY":"",e.anisotropyMap?"#define USE_ANISOTROPYMAP":"",e.clearcoatMap?"#define USE_CLEARCOATMAP":"",e.clearcoatRoughnessMap?"#define USE_CLEARCOAT_ROUGHNESSMAP":"",e.clearcoatNormalMap?"#define USE_CLEARCOAT_NORMALMAP":"",e.iridescenceMap?"#define USE_IRIDESCENCEMAP":"",e.iridescenceThicknessMap?"#define USE_IRIDESCENCE_THICKNESSMAP":"",e.specularMap?"#define USE_SPECULARMAP":"",e.specularColorMap?"#define USE_SPECULAR_COLORMAP":"",e.specularIntensityMap?"#define USE_SPECULAR_INTENSITYMAP":"",e.roughnessMap?"#define USE_ROUGHNESSMAP":"",e.metalnessMap?"#define USE_METALNESSMAP":"",e.alphaMap?"#define USE_ALPHAMAP":"",e.alphaHash?"#define USE_ALPHAHASH":"",e.transmission?"#define USE_TRANSMISSION":"",e.transmissionMap?"#define USE_TRANSMISSIONMAP":"",e.thicknessMap?"#define USE_THICKNESSMAP":"",e.sheenColorMap?"#define USE_SHEEN_COLORMAP":"",e.sheenRoughnessMap?"#define USE_SHEEN_ROUGHNESSMAP":"",e.mapUv?"#define MAP_UV "+e.mapUv:"",e.alphaMapUv?"#define ALPHAMAP_UV "+e.alphaMapUv:"",e.lightMapUv?"#define LIGHTMAP_UV "+e.lightMapUv:"",e.aoMapUv?"#define AOMAP_UV "+e.aoMapUv:"",e.emissiveMapUv?"#define EMISSIVEMAP_UV "+e.emissiveMapUv:"",e.bumpMapUv?"#define BUMPMAP_UV "+e.bumpMapUv:"",e.normalMapUv?"#define NORMALMAP_UV "+e.normalMapUv:"",e.displacementMapUv?"#define DISPLACEMENTMAP_UV "+e.displacementMapUv:"",e.metalnessMapUv?"#define METALNESSMAP_UV "+e.metalnessMapUv:"",e.roughnessMapUv?"#define ROUGHNESSMAP_UV "+e.roughnessMapUv:"",e.anisotropyMapUv?"#define ANISOTROPYMAP_UV "+e.anisotropyMapUv:"",e.clearcoatMapUv?"#define CLEARCOATMAP_UV "+e.clearcoatMapUv:"",e.clearcoatNormalMapUv?"#define CLEARCOAT_NORMALMAP_UV "+e.clearcoatNormalMapUv:"",e.clearcoatRoughnessMapUv?"#define CLEARCOAT_ROUGHNESSMAP_UV "+e.clearcoatRoughnessMapUv:"",e.iridescenceMapUv?"#define IRIDESCENCEMAP_UV "+e.iridescenceMapUv:"",e.iridescenceThicknessMapUv?"#define IRIDESCENCE_THICKNESSMAP_UV "+e.iridescenceThicknessMapUv:"",e.sheenColorMapUv?"#define SHEEN_COLORMAP_UV "+e.sheenColorMapUv:"",e.sheenRoughnessMapUv?"#define SHEEN_ROUGHNESSMAP_UV "+e.sheenRoughnessMapUv:"",e.specularMapUv?"#define SPECULARMAP_UV "+e.specularMapUv:"",e.specularColorMapUv?"#define SPECULAR_COLORMAP_UV "+e.specularColorMapUv:"",e.specularIntensityMapUv?"#define SPECULAR_INTENSITYMAP_UV "+e.specularIntensityMapUv:"",e.transmissionMapUv?"#define TRANSMISSIONMAP_UV "+e.transmissionMapUv:"",e.thicknessMapUv?"#define THICKNESSMAP_UV "+e.thicknessMapUv:"",e.vertexTangents&&e.flatShading===!1?"#define USE_TANGENT":"",e.vertexColors?"#define USE_COLOR":"",e.vertexAlphas?"#define USE_COLOR_ALPHA":"",e.vertexUv1s?"#define USE_UV1":"",e.vertexUv2s?"#define USE_UV2":"",e.vertexUv3s?"#define USE_UV3":"",e.pointsUvs?"#define USE_POINTS_UV":"",e.flatShading?"#define FLAT_SHADED":"",e.skinning?"#define USE_SKINNING":"",e.morphTargets?"#define USE_MORPHTARGETS":"",e.morphNormals&&e.flatShading===!1?"#define USE_MORPHNORMALS":"",e.morphColors?"#define USE_MORPHCOLORS":"",e.morphTargetsCount>0?"#define MORPHTARGETS_TEXTURE_STRIDE "+e.morphTextureStride:"",e.morphTargetsCount>0?"#define MORPHTARGETS_COUNT "+e.morphTargetsCount:"",e.doubleSided?"#define DOUBLE_SIDED":"",e.flipSided?"#define FLIP_SIDED":"",e.shadowMapEnabled?"#define USE_SHADOWMAP":"",e.shadowMapEnabled?"#define "+c:"",e.sizeAttenuation?"#define USE_SIZEATTENUATION":"",e.numLightProbes>0?"#define USE_LIGHT_PROBES":"",e.logarithmicDepthBuffer?"#define USE_LOGDEPTHBUF":"","uniform mat4 modelMatrix;","uniform mat4 modelViewMatrix;","uniform mat4 projectionMatrix;","uniform mat4 viewMatrix;","uniform mat3 normalMatrix;","uniform vec3 cameraPosition;","uniform bool isOrthographic;","#ifdef USE_INSTANCING","	attribute mat4 instanceMatrix;","#endif","#ifdef USE_INSTANCING_COLOR","	attribute vec3 instanceColor;","#endif","#ifdef USE_INSTANCING_MORPH","	uniform sampler2D morphTexture;","#endif","attribute vec3 position;","attribute vec3 normal;","attribute vec2 uv;","#ifdef USE_UV1","	attribute vec2 uv1;","#endif","#ifdef USE_UV2","	attribute vec2 uv2;","#endif","#ifdef USE_UV3","	attribute vec2 uv3;","#endif","#ifdef USE_TANGENT","	attribute vec4 tangent;","#endif","#if defined( USE_COLOR_ALPHA )","	attribute vec4 color;","#elif defined( USE_COLOR )","	attribute vec3 color;","#endif","#ifdef USE_SKINNING","	attribute vec4 skinIndex;","	attribute vec4 skinWeight;","#endif",`
`].filter(Bi).join(`
`),d=[Vo(e),"#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,v,e.useFog&&e.fog?"#define USE_FOG":"",e.useFog&&e.fogExp2?"#define FOG_EXP2":"",e.alphaToCoverage?"#define ALPHA_TO_COVERAGE":"",e.map?"#define USE_MAP":"",e.matcap?"#define USE_MATCAP":"",e.envMap?"#define USE_ENVMAP":"",e.envMap?"#define "+u:"",e.envMap?"#define "+f:"",e.envMap?"#define "+p:"",m?"#define CUBEUV_TEXEL_WIDTH "+m.texelWidth:"",m?"#define CUBEUV_TEXEL_HEIGHT "+m.texelHeight:"",m?"#define CUBEUV_MAX_MIP "+m.maxMip+".0":"",e.lightMap?"#define USE_LIGHTMAP":"",e.aoMap?"#define USE_AOMAP":"",e.bumpMap?"#define USE_BUMPMAP":"",e.normalMap?"#define USE_NORMALMAP":"",e.normalMapObjectSpace?"#define USE_NORMALMAP_OBJECTSPACE":"",e.normalMapTangentSpace?"#define USE_NORMALMAP_TANGENTSPACE":"",e.emissiveMap?"#define USE_EMISSIVEMAP":"",e.anisotropy?"#define USE_ANISOTROPY":"",e.anisotropyMap?"#define USE_ANISOTROPYMAP":"",e.clearcoat?"#define USE_CLEARCOAT":"",e.clearcoatMap?"#define USE_CLEARCOATMAP":"",e.clearcoatRoughnessMap?"#define USE_CLEARCOAT_ROUGHNESSMAP":"",e.clearcoatNormalMap?"#define USE_CLEARCOAT_NORMALMAP":"",e.dispersion?"#define USE_DISPERSION":"",e.iridescence?"#define USE_IRIDESCENCE":"",e.iridescenceMap?"#define USE_IRIDESCENCEMAP":"",e.iridescenceThicknessMap?"#define USE_IRIDESCENCE_THICKNESSMAP":"",e.specularMap?"#define USE_SPECULARMAP":"",e.specularColorMap?"#define USE_SPECULAR_COLORMAP":"",e.specularIntensityMap?"#define USE_SPECULAR_INTENSITYMAP":"",e.roughnessMap?"#define USE_ROUGHNESSMAP":"",e.metalnessMap?"#define USE_METALNESSMAP":"",e.alphaMap?"#define USE_ALPHAMAP":"",e.alphaTest?"#define USE_ALPHATEST":"",e.alphaHash?"#define USE_ALPHAHASH":"",e.sheen?"#define USE_SHEEN":"",e.sheenColorMap?"#define USE_SHEEN_COLORMAP":"",e.sheenRoughnessMap?"#define USE_SHEEN_ROUGHNESSMAP":"",e.transmission?"#define USE_TRANSMISSION":"",e.transmissionMap?"#define USE_TRANSMISSIONMAP":"",e.thicknessMap?"#define USE_THICKNESSMAP":"",e.vertexTangents&&e.flatShading===!1?"#define USE_TANGENT":"",e.vertexColors||e.instancingColor||e.batchingColor?"#define USE_COLOR":"",e.vertexAlphas?"#define USE_COLOR_ALPHA":"",e.vertexUv1s?"#define USE_UV1":"",e.vertexUv2s?"#define USE_UV2":"",e.vertexUv3s?"#define USE_UV3":"",e.pointsUvs?"#define USE_POINTS_UV":"",e.gradientMap?"#define USE_GRADIENTMAP":"",e.flatShading?"#define FLAT_SHADED":"",e.doubleSided?"#define DOUBLE_SIDED":"",e.flipSided?"#define FLIP_SIDED":"",e.shadowMapEnabled?"#define USE_SHADOWMAP":"",e.shadowMapEnabled?"#define "+c:"",e.premultipliedAlpha?"#define PREMULTIPLIED_ALPHA":"",e.numLightProbes>0?"#define USE_LIGHT_PROBES":"",e.decodeVideoTexture?"#define DECODE_VIDEO_TEXTURE":"",e.logarithmicDepthBuffer?"#define USE_LOGDEPTHBUF":"","uniform mat4 viewMatrix;","uniform vec3 cameraPosition;","uniform bool isOrthographic;",e.toneMapping!==wn?"#define TONE_MAPPING":"",e.toneMapping!==wn?Ut.tonemapping_pars_fragment:"",e.toneMapping!==wn?um("toneMapping",e.toneMapping):"",e.dithering?"#define DITHERING":"",e.opaque?"#define OPAQUE":"",Ut.colorspace_pars_fragment,lm("linearToOutputTexel",e.outputColorSpace),hm(),e.useDepthPacking?"#define DEPTH_PACKING "+e.depthPacking:"",`
`].filter(Bi).join(`
`)),o=ya(o),o=Ho(o,e),o=ko(o,e),a=ya(a),a=Ho(a,e),a=ko(a,e),o=Go(o),a=Go(a),e.isRawShaderMaterial!==!0&&(C=`#version 300 es
`,_=[g,"#define attribute in","#define varying out","#define texture2D texture"].join(`
`)+`
`+_,d=["#define varying in",e.glslVersion===io?"":"layout(location = 0) out highp vec4 pc_fragColor;",e.glslVersion===io?"":"#define gl_FragColor pc_fragColor","#define gl_FragDepthEXT gl_FragDepth","#define texture2D texture","#define textureCube texture","#define texture2DProj textureProj","#define texture2DLodEXT textureLod","#define texture2DProjLodEXT textureProjLod","#define textureCubeLodEXT textureLod","#define texture2DGradEXT textureGrad","#define texture2DProjGradEXT textureProjGrad","#define textureCubeGradEXT textureGrad"].join(`
`)+`
`+d);const w=C+_+o,b=C+d+a,z=Bo(r,r.VERTEX_SHADER,w),P=Bo(r,r.FRAGMENT_SHADER,b);r.attachShader(M,z),r.attachShader(M,P),e.index0AttributeName!==void 0?r.bindAttribLocation(M,0,e.index0AttributeName):e.morphTargets===!0&&r.bindAttribLocation(M,0,"position"),r.linkProgram(M);function I(N){if(n.debug.checkShaderErrors){const $=r.getProgramInfoLog(M).trim(),W=r.getShaderInfoLog(z).trim(),Y=r.getShaderInfoLog(P).trim();let tt=!0,K=!0;if(r.getProgramParameter(M,r.LINK_STATUS)===!1)if(tt=!1,typeof n.debug.onShaderError=="function")n.debug.onShaderError(r,M,z,P);else{const st=zo(r,z,"vertex"),j=zo(r,P,"fragment");console.error("THREE.WebGLProgram: Shader Error "+r.getError()+" - VALIDATE_STATUS "+r.getProgramParameter(M,r.VALIDATE_STATUS)+`

Material Name: `+N.name+`
Material Type: `+N.type+`

Program Info Log: `+$+`
`+st+`
`+j)}else $!==""?console.warn("THREE.WebGLProgram: Program Info Log:",$):(W===""||Y==="")&&(K=!1);K&&(N.diagnostics={runnable:tt,programLog:$,vertexShader:{log:W,prefix:_},fragmentShader:{log:Y,prefix:d}})}r.deleteShader(z),r.deleteShader(P),F=new Dr(r,M),A=pm(r,M)}let F;this.getUniforms=function(){return F===void 0&&I(this),F};let A;this.getAttributes=function(){return A===void 0&&I(this),A};let E=e.rendererExtensionParallelShaderCompile===!1;return this.isReady=function(){return E===!1&&(E=r.getProgramParameter(M,sm)),E},this.destroy=function(){i.releaseStatesOfProgram(this),r.deleteProgram(M),this.program=void 0},this.type=e.shaderType,this.name=e.shaderName,this.id=am++,this.cacheKey=t,this.usedTimes=1,this.program=M,this.vertexShader=z,this.fragmentShader=P,this}let Am=0;class bm{constructor(){this.shaderCache=new Map,this.materialCache=new Map}update(t){const e=t.vertexShader,i=t.fragmentShader,r=this._getShaderStage(e),s=this._getShaderStage(i),o=this._getShaderCacheForMaterial(t);return o.has(r)===!1&&(o.add(r),r.usedTimes++),o.has(s)===!1&&(o.add(s),s.usedTimes++),this}remove(t){const e=this.materialCache.get(t);for(const i of e)i.usedTimes--,i.usedTimes===0&&this.shaderCache.delete(i.code);return this.materialCache.delete(t),this}getVertexShaderID(t){return this._getShaderStage(t.vertexShader).id}getFragmentShaderID(t){return this._getShaderStage(t.fragmentShader).id}dispose(){this.shaderCache.clear(),this.materialCache.clear()}_getShaderCacheForMaterial(t){const e=this.materialCache;let i=e.get(t);return i===void 0&&(i=new Set,e.set(t,i)),i}_getShaderStage(t){const e=this.shaderCache;let i=e.get(t);return i===void 0&&(i=new Cm(t),e.set(t,i)),i}}class Cm{constructor(t){this.id=Am++,this.code=t,this.usedTimes=0}}function Rm(n,t,e,i,r,s,o){const a=new $c,c=new bm,u=new Set,f=[],p=r.logarithmicDepthBuffer,m=r.vertexTextures;let g=r.precision;const v={MeshDepthMaterial:"depth",MeshDistanceMaterial:"distanceRGBA",MeshNormalMaterial:"normal",MeshBasicMaterial:"basic",MeshLambertMaterial:"lambert",MeshPhongMaterial:"phong",MeshToonMaterial:"toon",MeshStandardMaterial:"physical",MeshPhysicalMaterial:"physical",MeshMatcapMaterial:"matcap",LineBasicMaterial:"basic",LineDashedMaterial:"dashed",PointsMaterial:"points",ShadowMaterial:"shadow",SpriteMaterial:"sprite"};function M(A){return u.add(A),A===0?"uv":`uv${A}`}function _(A,E,N,$,W){const Y=$.fog,tt=W.geometry,K=A.isMeshStandardMaterial?$.environment:null,st=(A.isMeshStandardMaterial?e:t).get(A.envMap||K),j=st&&st.mapping===Xr?st.image.height:null,dt=v[A.type];A.precision!==null&&(g=r.getMaxPrecision(A.precision),g!==A.precision&&console.warn("THREE.WebGLProgram.getParameters:",A.precision,"not supported, using",g,"instead."));const _t=tt.morphAttributes.position||tt.morphAttributes.normal||tt.morphAttributes.color,ut=_t!==void 0?_t.length:0;let wt=0;tt.morphAttributes.position!==void 0&&(wt=1),tt.morphAttributes.normal!==void 0&&(wt=2),tt.morphAttributes.color!==void 0&&(wt=3);let kt,Z,at,gt;if(dt){const Vt=qe[dt];kt=Vt.vertexShader,Z=Vt.fragmentShader}else kt=A.vertexShader,Z=A.fragmentShader,c.update(A),at=c.getVertexShaderID(A),gt=c.getFragmentShaderID(A);const pt=n.getRenderTarget(),At=W.isInstancedMesh===!0,Dt=W.isBatchedMesh===!0,Ft=!!A.map,Zt=!!A.matcap,D=!!st,Yt=!!A.aoMap,Ot=!!A.lightMap,Gt=!!A.bumpMap,Mt=!!A.normalMap,Jt=!!A.displacementMap,Lt=!!A.emissiveMap,It=!!A.metalnessMap,L=!!A.roughnessMap,S=A.anisotropy>0,V=A.clearcoat>0,it=A.dispersion>0,rt=A.iridescence>0,Q=A.sheen>0,Et=A.transmission>0,x=S&&!!A.anisotropyMap,l=V&&!!A.clearcoatMap,h=V&&!!A.clearcoatNormalMap,y=V&&!!A.clearcoatRoughnessMap,R=rt&&!!A.iridescenceMap,O=rt&&!!A.iridescenceThicknessMap,G=Q&&!!A.sheenColorMap,nt=Q&&!!A.sheenRoughnessMap,mt=!!A.specularMap,xt=!!A.specularColorMap,Rt=!!A.specularIntensityMap,U=Et&&!!A.transmissionMap,ot=Et&&!!A.thicknessMap,J=!!A.gradientMap,et=!!A.alphaMap,lt=A.alphaTest>0,bt=!!A.alphaHash,zt=!!A.extensions;let ne=wn;A.toneMapped&&(pt===null||pt.isXRRenderTarget===!0)&&(ne=n.toneMapping);const ue={shaderID:dt,shaderType:A.type,shaderName:A.name,vertexShader:kt,fragmentShader:Z,defines:A.defines,customVertexShaderID:at,customFragmentShaderID:gt,isRawShaderMaterial:A.isRawShaderMaterial===!0,glslVersion:A.glslVersion,precision:g,batching:Dt,batchingColor:Dt&&W._colorsTexture!==null,instancing:At,instancingColor:At&&W.instanceColor!==null,instancingMorph:At&&W.morphTexture!==null,supportsVertexTextures:m,outputColorSpace:pt===null?n.outputColorSpace:pt.isXRRenderTarget===!0?pt.texture.colorSpace:Rn,alphaToCoverage:!!A.alphaToCoverage,map:Ft,matcap:Zt,envMap:D,envMapMode:D&&st.mapping,envMapCubeUVHeight:j,aoMap:Yt,lightMap:Ot,bumpMap:Gt,normalMap:Mt,displacementMap:m&&Jt,emissiveMap:Lt,normalMapObjectSpace:Mt&&A.normalMapType===Ou,normalMapTangentSpace:Mt&&A.normalMapType===Ua,metalnessMap:It,roughnessMap:L,anisotropy:S,anisotropyMap:x,clearcoat:V,clearcoatMap:l,clearcoatNormalMap:h,clearcoatRoughnessMap:y,dispersion:it,iridescence:rt,iridescenceMap:R,iridescenceThicknessMap:O,sheen:Q,sheenColorMap:G,sheenRoughnessMap:nt,specularMap:mt,specularColorMap:xt,specularIntensityMap:Rt,transmission:Et,transmissionMap:U,thicknessMap:ot,gradientMap:J,opaque:A.transparent===!1&&A.blending===gi&&A.alphaToCoverage===!1,alphaMap:et,alphaTest:lt,alphaHash:bt,combine:A.combine,mapUv:Ft&&M(A.map.channel),aoMapUv:Yt&&M(A.aoMap.channel),lightMapUv:Ot&&M(A.lightMap.channel),bumpMapUv:Gt&&M(A.bumpMap.channel),normalMapUv:Mt&&M(A.normalMap.channel),displacementMapUv:Jt&&M(A.displacementMap.channel),emissiveMapUv:Lt&&M(A.emissiveMap.channel),metalnessMapUv:It&&M(A.metalnessMap.channel),roughnessMapUv:L&&M(A.roughnessMap.channel),anisotropyMapUv:x&&M(A.anisotropyMap.channel),clearcoatMapUv:l&&M(A.clearcoatMap.channel),clearcoatNormalMapUv:h&&M(A.clearcoatNormalMap.channel),clearcoatRoughnessMapUv:y&&M(A.clearcoatRoughnessMap.channel),iridescenceMapUv:R&&M(A.iridescenceMap.channel),iridescenceThicknessMapUv:O&&M(A.iridescenceThicknessMap.channel),sheenColorMapUv:G&&M(A.sheenColorMap.channel),sheenRoughnessMapUv:nt&&M(A.sheenRoughnessMap.channel),specularMapUv:mt&&M(A.specularMap.channel),specularColorMapUv:xt&&M(A.specularColorMap.channel),specularIntensityMapUv:Rt&&M(A.specularIntensityMap.channel),transmissionMapUv:U&&M(A.transmissionMap.channel),thicknessMapUv:ot&&M(A.thicknessMap.channel),alphaMapUv:et&&M(A.alphaMap.channel),vertexTangents:!!tt.attributes.tangent&&(Mt||S),vertexColors:A.vertexColors,vertexAlphas:A.vertexColors===!0&&!!tt.attributes.color&&tt.attributes.color.itemSize===4,pointsUvs:W.isPoints===!0&&!!tt.attributes.uv&&(Ft||et),fog:!!Y,useFog:A.fog===!0,fogExp2:!!Y&&Y.isFogExp2,flatShading:A.flatShading===!0,sizeAttenuation:A.sizeAttenuation===!0,logarithmicDepthBuffer:p,skinning:W.isSkinnedMesh===!0,morphTargets:tt.morphAttributes.position!==void 0,morphNormals:tt.morphAttributes.normal!==void 0,morphColors:tt.morphAttributes.color!==void 0,morphTargetsCount:ut,morphTextureStride:wt,numDirLights:E.directional.length,numPointLights:E.point.length,numSpotLights:E.spot.length,numSpotLightMaps:E.spotLightMap.length,numRectAreaLights:E.rectArea.length,numHemiLights:E.hemi.length,numDirLightShadows:E.directionalShadowMap.length,numPointLightShadows:E.pointShadowMap.length,numSpotLightShadows:E.spotShadowMap.length,numSpotLightShadowsWithMaps:E.numSpotLightShadowsWithMaps,numLightProbes:E.numLightProbes,numClippingPlanes:o.numPlanes,numClipIntersection:o.numIntersection,dithering:A.dithering,shadowMapEnabled:n.shadowMap.enabled&&N.length>0,shadowMapType:n.shadowMap.type,toneMapping:ne,decodeVideoTexture:Ft&&A.map.isVideoTexture===!0&&$t.getTransfer(A.map.colorSpace)===Kt,premultipliedAlpha:A.premultipliedAlpha,doubleSided:A.side===ge,flipSided:A.side===Se,useDepthPacking:A.depthPacking>=0,depthPacking:A.depthPacking||0,index0AttributeName:A.index0AttributeName,extensionClipCullDistance:zt&&A.extensions.clipCullDistance===!0&&i.has("WEBGL_clip_cull_distance"),extensionMultiDraw:(zt&&A.extensions.multiDraw===!0||Dt)&&i.has("WEBGL_multi_draw"),rendererExtensionParallelShaderCompile:i.has("KHR_parallel_shader_compile"),customProgramCacheKey:A.customProgramCacheKey()};return ue.vertexUv1s=u.has(1),ue.vertexUv2s=u.has(2),ue.vertexUv3s=u.has(3),u.clear(),ue}function d(A){const E=[];if(A.shaderID?E.push(A.shaderID):(E.push(A.customVertexShaderID),E.push(A.customFragmentShaderID)),A.defines!==void 0)for(const N in A.defines)E.push(N),E.push(A.defines[N]);return A.isRawShaderMaterial===!1&&(C(E,A),w(E,A),E.push(n.outputColorSpace)),E.push(A.customProgramCacheKey),E.join()}function C(A,E){A.push(E.precision),A.push(E.outputColorSpace),A.push(E.envMapMode),A.push(E.envMapCubeUVHeight),A.push(E.mapUv),A.push(E.alphaMapUv),A.push(E.lightMapUv),A.push(E.aoMapUv),A.push(E.bumpMapUv),A.push(E.normalMapUv),A.push(E.displacementMapUv),A.push(E.emissiveMapUv),A.push(E.metalnessMapUv),A.push(E.roughnessMapUv),A.push(E.anisotropyMapUv),A.push(E.clearcoatMapUv),A.push(E.clearcoatNormalMapUv),A.push(E.clearcoatRoughnessMapUv),A.push(E.iridescenceMapUv),A.push(E.iridescenceThicknessMapUv),A.push(E.sheenColorMapUv),A.push(E.sheenRoughnessMapUv),A.push(E.specularMapUv),A.push(E.specularColorMapUv),A.push(E.specularIntensityMapUv),A.push(E.transmissionMapUv),A.push(E.thicknessMapUv),A.push(E.combine),A.push(E.fogExp2),A.push(E.sizeAttenuation),A.push(E.morphTargetsCount),A.push(E.morphAttributeCount),A.push(E.numDirLights),A.push(E.numPointLights),A.push(E.numSpotLights),A.push(E.numSpotLightMaps),A.push(E.numHemiLights),A.push(E.numRectAreaLights),A.push(E.numDirLightShadows),A.push(E.numPointLightShadows),A.push(E.numSpotLightShadows),A.push(E.numSpotLightShadowsWithMaps),A.push(E.numLightProbes),A.push(E.shadowMapType),A.push(E.toneMapping),A.push(E.numClippingPlanes),A.push(E.numClipIntersection),A.push(E.depthPacking)}function w(A,E){a.disableAll(),E.supportsVertexTextures&&a.enable(0),E.instancing&&a.enable(1),E.instancingColor&&a.enable(2),E.instancingMorph&&a.enable(3),E.matcap&&a.enable(4),E.envMap&&a.enable(5),E.normalMapObjectSpace&&a.enable(6),E.normalMapTangentSpace&&a.enable(7),E.clearcoat&&a.enable(8),E.iridescence&&a.enable(9),E.alphaTest&&a.enable(10),E.vertexColors&&a.enable(11),E.vertexAlphas&&a.enable(12),E.vertexUv1s&&a.enable(13),E.vertexUv2s&&a.enable(14),E.vertexUv3s&&a.enable(15),E.vertexTangents&&a.enable(16),E.anisotropy&&a.enable(17),E.alphaHash&&a.enable(18),E.batching&&a.enable(19),E.dispersion&&a.enable(20),E.batchingColor&&a.enable(21),A.push(a.mask),a.disableAll(),E.fog&&a.enable(0),E.useFog&&a.enable(1),E.flatShading&&a.enable(2),E.logarithmicDepthBuffer&&a.enable(3),E.skinning&&a.enable(4),E.morphTargets&&a.enable(5),E.morphNormals&&a.enable(6),E.morphColors&&a.enable(7),E.premultipliedAlpha&&a.enable(8),E.shadowMapEnabled&&a.enable(9),E.doubleSided&&a.enable(10),E.flipSided&&a.enable(11),E.useDepthPacking&&a.enable(12),E.dithering&&a.enable(13),E.transmission&&a.enable(14),E.sheen&&a.enable(15),E.opaque&&a.enable(16),E.pointsUvs&&a.enable(17),E.decodeVideoTexture&&a.enable(18),E.alphaToCoverage&&a.enable(19),A.push(a.mask)}function b(A){const E=v[A.type];let N;if(E){const $=qe[E];N=hh.clone($.uniforms)}else N=A.uniforms;return N}function z(A,E){let N;for(let $=0,W=f.length;$<W;$++){const Y=f[$];if(Y.cacheKey===E){N=Y,++N.usedTimes;break}}return N===void 0&&(N=new wm(n,E,A,s),f.push(N)),N}function P(A){if(--A.usedTimes===0){const E=f.indexOf(A);f[E]=f[f.length-1],f.pop(),A.destroy()}}function I(A){c.remove(A)}function F(){c.dispose()}return{getParameters:_,getProgramCacheKey:d,getUniforms:b,acquireProgram:z,releaseProgram:P,releaseShaderCache:I,programs:f,dispose:F}}function Pm(){let n=new WeakMap;function t(s){let o=n.get(s);return o===void 0&&(o={},n.set(s,o)),o}function e(s){n.delete(s)}function i(s,o,a){n.get(s)[o]=a}function r(){n=new WeakMap}return{get:t,remove:e,update:i,dispose:r}}function Lm(n,t){return n.groupOrder!==t.groupOrder?n.groupOrder-t.groupOrder:n.renderOrder!==t.renderOrder?n.renderOrder-t.renderOrder:n.material.id!==t.material.id?n.material.id-t.material.id:n.z!==t.z?n.z-t.z:n.id-t.id}function Wo(n,t){return n.groupOrder!==t.groupOrder?n.groupOrder-t.groupOrder:n.renderOrder!==t.renderOrder?n.renderOrder-t.renderOrder:n.z!==t.z?t.z-n.z:n.id-t.id}function qo(){const n=[];let t=0;const e=[],i=[],r=[];function s(){t=0,e.length=0,i.length=0,r.length=0}function o(p,m,g,v,M,_){let d=n[t];return d===void 0?(d={id:p.id,object:p,geometry:m,material:g,groupOrder:v,renderOrder:p.renderOrder,z:M,group:_},n[t]=d):(d.id=p.id,d.object=p,d.geometry=m,d.material=g,d.groupOrder=v,d.renderOrder=p.renderOrder,d.z=M,d.group=_),t++,d}function a(p,m,g,v,M,_){const d=o(p,m,g,v,M,_);g.transmission>0?i.push(d):g.transparent===!0?r.push(d):e.push(d)}function c(p,m,g,v,M,_){const d=o(p,m,g,v,M,_);g.transmission>0?i.unshift(d):g.transparent===!0?r.unshift(d):e.unshift(d)}function u(p,m){e.length>1&&e.sort(p||Lm),i.length>1&&i.sort(m||Wo),r.length>1&&r.sort(m||Wo)}function f(){for(let p=t,m=n.length;p<m;p++){const g=n[p];if(g.id===null)break;g.id=null,g.object=null,g.geometry=null,g.material=null,g.group=null}}return{opaque:e,transmissive:i,transparent:r,init:s,push:a,unshift:c,finish:f,sort:u}}function Im(){let n=new WeakMap;function t(i,r){const s=n.get(i);let o;return s===void 0?(o=new qo,n.set(i,[o])):r>=s.length?(o=new qo,s.push(o)):o=s[r],o}function e(){n=new WeakMap}return{get:t,dispose:e}}function Dm(){const n={};return{get:function(t){if(n[t.id]!==void 0)return n[t.id];let e;switch(t.type){case"DirectionalLight":e={direction:new k,color:new Ht};break;case"SpotLight":e={position:new k,direction:new k,color:new Ht,distance:0,coneCos:0,penumbraCos:0,decay:0};break;case"PointLight":e={position:new k,color:new Ht,distance:0,decay:0};break;case"HemisphereLight":e={direction:new k,skyColor:new Ht,groundColor:new Ht};break;case"RectAreaLight":e={color:new Ht,position:new k,halfWidth:new k,halfHeight:new k};break}return n[t.id]=e,e}}}function Um(){const n={};return{get:function(t){if(n[t.id]!==void 0)return n[t.id];let e;switch(t.type){case"DirectionalLight":e={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new Bt};break;case"SpotLight":e={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new Bt};break;case"PointLight":e={shadowIntensity:1,shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new Bt,shadowCameraNear:1,shadowCameraFar:1e3};break}return n[t.id]=e,e}}}let Nm=0;function Fm(n,t){return(t.castShadow?2:0)-(n.castShadow?2:0)+(t.map?1:0)-(n.map?1:0)}function Om(n){const t=new Dm,e=Um(),i={version:0,hash:{directionalLength:-1,pointLength:-1,spotLength:-1,rectAreaLength:-1,hemiLength:-1,numDirectionalShadows:-1,numPointShadows:-1,numSpotShadows:-1,numSpotMaps:-1,numLightProbes:-1},ambient:[0,0,0],probe:[],directional:[],directionalShadow:[],directionalShadowMap:[],directionalShadowMatrix:[],spot:[],spotLightMap:[],spotShadow:[],spotShadowMap:[],spotLightMatrix:[],rectArea:[],rectAreaLTC1:null,rectAreaLTC2:null,point:[],pointShadow:[],pointShadowMap:[],pointShadowMatrix:[],hemi:[],numSpotLightShadowsWithMaps:0,numLightProbes:0};for(let u=0;u<9;u++)i.probe.push(new k);const r=new k,s=new Qt,o=new Qt;function a(u){let f=0,p=0,m=0;for(let A=0;A<9;A++)i.probe[A].set(0,0,0);let g=0,v=0,M=0,_=0,d=0,C=0,w=0,b=0,z=0,P=0,I=0;u.sort(Fm);for(let A=0,E=u.length;A<E;A++){const N=u[A],$=N.color,W=N.intensity,Y=N.distance,tt=N.shadow&&N.shadow.map?N.shadow.map.texture:null;if(N.isAmbientLight)f+=$.r*W,p+=$.g*W,m+=$.b*W;else if(N.isLightProbe){for(let K=0;K<9;K++)i.probe[K].addScaledVector(N.sh.coefficients[K],W);I++}else if(N.isDirectionalLight){const K=t.get(N);if(K.color.copy(N.color).multiplyScalar(N.intensity),N.castShadow){const st=N.shadow,j=e.get(N);j.shadowIntensity=st.intensity,j.shadowBias=st.bias,j.shadowNormalBias=st.normalBias,j.shadowRadius=st.radius,j.shadowMapSize=st.mapSize,i.directionalShadow[g]=j,i.directionalShadowMap[g]=tt,i.directionalShadowMatrix[g]=N.shadow.matrix,C++}i.directional[g]=K,g++}else if(N.isSpotLight){const K=t.get(N);K.position.setFromMatrixPosition(N.matrixWorld),K.color.copy($).multiplyScalar(W),K.distance=Y,K.coneCos=Math.cos(N.angle),K.penumbraCos=Math.cos(N.angle*(1-N.penumbra)),K.decay=N.decay,i.spot[M]=K;const st=N.shadow;if(N.map&&(i.spotLightMap[z]=N.map,z++,st.updateMatrices(N),N.castShadow&&P++),i.spotLightMatrix[M]=st.matrix,N.castShadow){const j=e.get(N);j.shadowIntensity=st.intensity,j.shadowBias=st.bias,j.shadowNormalBias=st.normalBias,j.shadowRadius=st.radius,j.shadowMapSize=st.mapSize,i.spotShadow[M]=j,i.spotShadowMap[M]=tt,b++}M++}else if(N.isRectAreaLight){const K=t.get(N);K.color.copy($).multiplyScalar(W),K.halfWidth.set(N.width*.5,0,0),K.halfHeight.set(0,N.height*.5,0),i.rectArea[_]=K,_++}else if(N.isPointLight){const K=t.get(N);if(K.color.copy(N.color).multiplyScalar(N.intensity),K.distance=N.distance,K.decay=N.decay,N.castShadow){const st=N.shadow,j=e.get(N);j.shadowIntensity=st.intensity,j.shadowBias=st.bias,j.shadowNormalBias=st.normalBias,j.shadowRadius=st.radius,j.shadowMapSize=st.mapSize,j.shadowCameraNear=st.camera.near,j.shadowCameraFar=st.camera.far,i.pointShadow[v]=j,i.pointShadowMap[v]=tt,i.pointShadowMatrix[v]=N.shadow.matrix,w++}i.point[v]=K,v++}else if(N.isHemisphereLight){const K=t.get(N);K.skyColor.copy(N.color).multiplyScalar(W),K.groundColor.copy(N.groundColor).multiplyScalar(W),i.hemi[d]=K,d++}}_>0&&(n.has("OES_texture_float_linear")===!0?(i.rectAreaLTC1=ht.LTC_FLOAT_1,i.rectAreaLTC2=ht.LTC_FLOAT_2):(i.rectAreaLTC1=ht.LTC_HALF_1,i.rectAreaLTC2=ht.LTC_HALF_2)),i.ambient[0]=f,i.ambient[1]=p,i.ambient[2]=m;const F=i.hash;(F.directionalLength!==g||F.pointLength!==v||F.spotLength!==M||F.rectAreaLength!==_||F.hemiLength!==d||F.numDirectionalShadows!==C||F.numPointShadows!==w||F.numSpotShadows!==b||F.numSpotMaps!==z||F.numLightProbes!==I)&&(i.directional.length=g,i.spot.length=M,i.rectArea.length=_,i.point.length=v,i.hemi.length=d,i.directionalShadow.length=C,i.directionalShadowMap.length=C,i.pointShadow.length=w,i.pointShadowMap.length=w,i.spotShadow.length=b,i.spotShadowMap.length=b,i.directionalShadowMatrix.length=C,i.pointShadowMatrix.length=w,i.spotLightMatrix.length=b+z-P,i.spotLightMap.length=z,i.numSpotLightShadowsWithMaps=P,i.numLightProbes=I,F.directionalLength=g,F.pointLength=v,F.spotLength=M,F.rectAreaLength=_,F.hemiLength=d,F.numDirectionalShadows=C,F.numPointShadows=w,F.numSpotShadows=b,F.numSpotMaps=z,F.numLightProbes=I,i.version=Nm++)}function c(u,f){let p=0,m=0,g=0,v=0,M=0;const _=f.matrixWorldInverse;for(let d=0,C=u.length;d<C;d++){const w=u[d];if(w.isDirectionalLight){const b=i.directional[p];b.direction.setFromMatrixPosition(w.matrixWorld),r.setFromMatrixPosition(w.target.matrixWorld),b.direction.sub(r),b.direction.transformDirection(_),p++}else if(w.isSpotLight){const b=i.spot[g];b.position.setFromMatrixPosition(w.matrixWorld),b.position.applyMatrix4(_),b.direction.setFromMatrixPosition(w.matrixWorld),r.setFromMatrixPosition(w.target.matrixWorld),b.direction.sub(r),b.direction.transformDirection(_),g++}else if(w.isRectAreaLight){const b=i.rectArea[v];b.position.setFromMatrixPosition(w.matrixWorld),b.position.applyMatrix4(_),o.identity(),s.copy(w.matrixWorld),s.premultiply(_),o.extractRotation(s),b.halfWidth.set(w.width*.5,0,0),b.halfHeight.set(0,w.height*.5,0),b.halfWidth.applyMatrix4(o),b.halfHeight.applyMatrix4(o),v++}else if(w.isPointLight){const b=i.point[m];b.position.setFromMatrixPosition(w.matrixWorld),b.position.applyMatrix4(_),m++}else if(w.isHemisphereLight){const b=i.hemi[M];b.direction.setFromMatrixPosition(w.matrixWorld),b.direction.transformDirection(_),M++}}}return{setup:a,setupView:c,state:i}}function Xo(n){const t=new Om(n),e=[],i=[];function r(f){u.camera=f,e.length=0,i.length=0}function s(f){e.push(f)}function o(f){i.push(f)}function a(){t.setup(e)}function c(f){t.setupView(e,f)}const u={lightsArray:e,shadowsArray:i,camera:null,lights:t,transmissionRenderTarget:{}};return{init:r,state:u,setupLights:a,setupLightsView:c,pushLight:s,pushShadow:o}}function Bm(n){let t=new WeakMap;function e(r,s=0){const o=t.get(r);let a;return o===void 0?(a=new Xo(n),t.set(r,[a])):s>=o.length?(a=new Xo(n),o.push(a)):a=o[s],a}function i(){t=new WeakMap}return{get:e,dispose:i}}class zm extends Kn{constructor(t){super(),this.isMeshDepthMaterial=!0,this.type="MeshDepthMaterial",this.depthPacking=Nu,this.map=null,this.alphaMap=null,this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.wireframe=!1,this.wireframeLinewidth=1,this.setValues(t)}copy(t){return super.copy(t),this.depthPacking=t.depthPacking,this.map=t.map,this.alphaMap=t.alphaMap,this.displacementMap=t.displacementMap,this.displacementScale=t.displacementScale,this.displacementBias=t.displacementBias,this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this}}class Hm extends Kn{constructor(t){super(),this.isMeshDistanceMaterial=!0,this.type="MeshDistanceMaterial",this.map=null,this.alphaMap=null,this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.setValues(t)}copy(t){return super.copy(t),this.map=t.map,this.alphaMap=t.alphaMap,this.displacementMap=t.displacementMap,this.displacementScale=t.displacementScale,this.displacementBias=t.displacementBias,this}}const km=`void main() {
	gl_Position = vec4( position, 1.0 );
}`,Gm=`uniform sampler2D shadow_pass;
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
}`;function Vm(n,t,e){let i=new Fa;const r=new Bt,s=new Bt,o=new se,a=new zm({depthPacking:Fu}),c=new Hm,u={},f=e.maxTextureSize,p={[An]:Se,[Se]:An,[ge]:ge},m=new bn({defines:{VSM_SAMPLES:8},uniforms:{shadow_pass:{value:null},resolution:{value:new Bt},radius:{value:4}},vertexShader:km,fragmentShader:Gm}),g=m.clone();g.defines.HORIZONTAL_PASS=1;const v=new Ce;v.setAttribute("position",new ke(new Float32Array([-1,-1,.5,3,-1,.5,-1,3,.5]),3));const M=new ae(v,m),_=this;this.enabled=!1,this.autoUpdate=!0,this.needsUpdate=!1,this.type=Rc;let d=this.type;this.render=function(P,I,F){if(_.enabled===!1||_.autoUpdate===!1&&_.needsUpdate===!1||P.length===0)return;const A=n.getRenderTarget(),E=n.getActiveCubeFace(),N=n.getActiveMipmapLevel(),$=n.state;$.setBlending(Tn),$.buffers.color.setClear(1,1,1,1),$.buffers.depth.setTest(!0),$.setScissorTest(!1);const W=d!==en&&this.type===en,Y=d===en&&this.type!==en;for(let tt=0,K=P.length;tt<K;tt++){const st=P[tt],j=st.shadow;if(j===void 0){console.warn("THREE.WebGLShadowMap:",st,"has no shadow.");continue}if(j.autoUpdate===!1&&j.needsUpdate===!1)continue;r.copy(j.mapSize);const dt=j.getFrameExtents();if(r.multiply(dt),s.copy(j.mapSize),(r.x>f||r.y>f)&&(r.x>f&&(s.x=Math.floor(f/dt.x),r.x=s.x*dt.x,j.mapSize.x=s.x),r.y>f&&(s.y=Math.floor(f/dt.y),r.y=s.y*dt.y,j.mapSize.y=s.y)),j.map===null||W===!0||Y===!0){const ut=this.type!==en?{minFilter:De,magFilter:De}:{};j.map!==null&&j.map.dispose(),j.map=new qn(r.x,r.y,ut),j.map.texture.name=st.name+".shadowMap",j.camera.updateProjectionMatrix()}n.setRenderTarget(j.map),n.clear();const _t=j.getViewportCount();for(let ut=0;ut<_t;ut++){const wt=j.getViewport(ut);o.set(s.x*wt.x,s.y*wt.y,s.x*wt.z,s.y*wt.w),$.viewport(o),j.updateMatrices(st,ut),i=j.getFrustum(),b(I,F,j.camera,st,this.type)}j.isPointLightShadow!==!0&&this.type===en&&C(j,F),j.needsUpdate=!1}d=this.type,_.needsUpdate=!1,n.setRenderTarget(A,E,N)};function C(P,I){const F=t.update(M);m.defines.VSM_SAMPLES!==P.blurSamples&&(m.defines.VSM_SAMPLES=P.blurSamples,g.defines.VSM_SAMPLES=P.blurSamples,m.needsUpdate=!0,g.needsUpdate=!0),P.mapPass===null&&(P.mapPass=new qn(r.x,r.y)),m.uniforms.shadow_pass.value=P.map.texture,m.uniforms.resolution.value=P.mapSize,m.uniforms.radius.value=P.radius,n.setRenderTarget(P.mapPass),n.clear(),n.renderBufferDirect(I,null,F,m,M,null),g.uniforms.shadow_pass.value=P.mapPass.texture,g.uniforms.resolution.value=P.mapSize,g.uniforms.radius.value=P.radius,n.setRenderTarget(P.map),n.clear(),n.renderBufferDirect(I,null,F,g,M,null)}function w(P,I,F,A){let E=null;const N=F.isPointLight===!0?P.customDistanceMaterial:P.customDepthMaterial;if(N!==void 0)E=N;else if(E=F.isPointLight===!0?c:a,n.localClippingEnabled&&I.clipShadows===!0&&Array.isArray(I.clippingPlanes)&&I.clippingPlanes.length!==0||I.displacementMap&&I.displacementScale!==0||I.alphaMap&&I.alphaTest>0||I.map&&I.alphaTest>0){const $=E.uuid,W=I.uuid;let Y=u[$];Y===void 0&&(Y={},u[$]=Y);let tt=Y[W];tt===void 0&&(tt=E.clone(),Y[W]=tt,I.addEventListener("dispose",z)),E=tt}if(E.visible=I.visible,E.wireframe=I.wireframe,A===en?E.side=I.shadowSide!==null?I.shadowSide:I.side:E.side=I.shadowSide!==null?I.shadowSide:p[I.side],E.alphaMap=I.alphaMap,E.alphaTest=I.alphaTest,E.map=I.map,E.clipShadows=I.clipShadows,E.clippingPlanes=I.clippingPlanes,E.clipIntersection=I.clipIntersection,E.displacementMap=I.displacementMap,E.displacementScale=I.displacementScale,E.displacementBias=I.displacementBias,E.wireframeLinewidth=I.wireframeLinewidth,E.linewidth=I.linewidth,F.isPointLight===!0&&E.isMeshDistanceMaterial===!0){const $=n.properties.get(E);$.light=F}return E}function b(P,I,F,A,E){if(P.visible===!1)return;if(P.layers.test(I.layers)&&(P.isMesh||P.isLine||P.isPoints)&&(P.castShadow||P.receiveShadow&&E===en)&&(!P.frustumCulled||i.intersectsObject(P))){P.modelViewMatrix.multiplyMatrices(F.matrixWorldInverse,P.matrixWorld);const W=t.update(P),Y=P.material;if(Array.isArray(Y)){const tt=W.groups;for(let K=0,st=tt.length;K<st;K++){const j=tt[K],dt=Y[j.materialIndex];if(dt&&dt.visible){const _t=w(P,dt,A,E);P.onBeforeShadow(n,P,I,F,W,_t,j),n.renderBufferDirect(F,null,W,_t,P,j),P.onAfterShadow(n,P,I,F,W,_t,j)}}}else if(Y.visible){const tt=w(P,Y,A,E);P.onBeforeShadow(n,P,I,F,W,tt,null),n.renderBufferDirect(F,null,W,tt,P,null),P.onAfterShadow(n,P,I,F,W,tt,null)}}const $=P.children;for(let W=0,Y=$.length;W<Y;W++)b($[W],I,F,A,E)}function z(P){P.target.removeEventListener("dispose",z);for(const F in u){const A=u[F],E=P.target.uuid;E in A&&(A[E].dispose(),delete A[E])}}}function Wm(n){function t(){let U=!1;const ot=new se;let J=null;const et=new se(0,0,0,0);return{setMask:function(lt){J!==lt&&!U&&(n.colorMask(lt,lt,lt,lt),J=lt)},setLocked:function(lt){U=lt},setClear:function(lt,bt,zt,ne,ue){ue===!0&&(lt*=ne,bt*=ne,zt*=ne),ot.set(lt,bt,zt,ne),et.equals(ot)===!1&&(n.clearColor(lt,bt,zt,ne),et.copy(ot))},reset:function(){U=!1,J=null,et.set(-1,0,0,0)}}}function e(){let U=!1,ot=null,J=null,et=null;return{setTest:function(lt){lt?gt(n.DEPTH_TEST):pt(n.DEPTH_TEST)},setMask:function(lt){ot!==lt&&!U&&(n.depthMask(lt),ot=lt)},setFunc:function(lt){if(J!==lt){switch(lt){case xu:n.depthFunc(n.NEVER);break;case vu:n.depthFunc(n.ALWAYS);break;case yu:n.depthFunc(n.LESS);break;case Nr:n.depthFunc(n.LEQUAL);break;case Mu:n.depthFunc(n.EQUAL);break;case Su:n.depthFunc(n.GEQUAL);break;case Eu:n.depthFunc(n.GREATER);break;case Tu:n.depthFunc(n.NOTEQUAL);break;default:n.depthFunc(n.LEQUAL)}J=lt}},setLocked:function(lt){U=lt},setClear:function(lt){et!==lt&&(n.clearDepth(lt),et=lt)},reset:function(){U=!1,ot=null,J=null,et=null}}}function i(){let U=!1,ot=null,J=null,et=null,lt=null,bt=null,zt=null,ne=null,ue=null;return{setTest:function(Vt){U||(Vt?gt(n.STENCIL_TEST):pt(n.STENCIL_TEST))},setMask:function(Vt){ot!==Vt&&!U&&(n.stencilMask(Vt),ot=Vt)},setFunc:function(Vt,Ke,Ve){(J!==Vt||et!==Ke||lt!==Ve)&&(n.stencilFunc(Vt,Ke,Ve),J=Vt,et=Ke,lt=Ve)},setOp:function(Vt,Ke,Ve){(bt!==Vt||zt!==Ke||ne!==Ve)&&(n.stencilOp(Vt,Ke,Ve),bt=Vt,zt=Ke,ne=Ve)},setLocked:function(Vt){U=Vt},setClear:function(Vt){ue!==Vt&&(n.clearStencil(Vt),ue=Vt)},reset:function(){U=!1,ot=null,J=null,et=null,lt=null,bt=null,zt=null,ne=null,ue=null}}}const r=new t,s=new e,o=new i,a=new WeakMap,c=new WeakMap;let u={},f={},p=new WeakMap,m=[],g=null,v=!1,M=null,_=null,d=null,C=null,w=null,b=null,z=null,P=new Ht(0,0,0),I=0,F=!1,A=null,E=null,N=null,$=null,W=null;const Y=n.getParameter(n.MAX_COMBINED_TEXTURE_IMAGE_UNITS);let tt=!1,K=0;const st=n.getParameter(n.VERSION);st.indexOf("WebGL")!==-1?(K=parseFloat(/^WebGL (\d)/.exec(st)[1]),tt=K>=1):st.indexOf("OpenGL ES")!==-1&&(K=parseFloat(/^OpenGL ES (\d)/.exec(st)[1]),tt=K>=2);let j=null,dt={};const _t=n.getParameter(n.SCISSOR_BOX),ut=n.getParameter(n.VIEWPORT),wt=new se().fromArray(_t),kt=new se().fromArray(ut);function Z(U,ot,J,et){const lt=new Uint8Array(4),bt=n.createTexture();n.bindTexture(U,bt),n.texParameteri(U,n.TEXTURE_MIN_FILTER,n.NEAREST),n.texParameteri(U,n.TEXTURE_MAG_FILTER,n.NEAREST);for(let zt=0;zt<J;zt++)U===n.TEXTURE_3D||U===n.TEXTURE_2D_ARRAY?n.texImage3D(ot,0,n.RGBA,1,1,et,0,n.RGBA,n.UNSIGNED_BYTE,lt):n.texImage2D(ot+zt,0,n.RGBA,1,1,0,n.RGBA,n.UNSIGNED_BYTE,lt);return bt}const at={};at[n.TEXTURE_2D]=Z(n.TEXTURE_2D,n.TEXTURE_2D,1),at[n.TEXTURE_CUBE_MAP]=Z(n.TEXTURE_CUBE_MAP,n.TEXTURE_CUBE_MAP_POSITIVE_X,6),at[n.TEXTURE_2D_ARRAY]=Z(n.TEXTURE_2D_ARRAY,n.TEXTURE_2D_ARRAY,1,1),at[n.TEXTURE_3D]=Z(n.TEXTURE_3D,n.TEXTURE_3D,1,1),r.setClear(0,0,0,1),s.setClear(1),o.setClear(0),gt(n.DEPTH_TEST),s.setFunc(Nr),Gt(!1),Mt(Za),gt(n.CULL_FACE),Yt(Tn);function gt(U){u[U]!==!0&&(n.enable(U),u[U]=!0)}function pt(U){u[U]!==!1&&(n.disable(U),u[U]=!1)}function At(U,ot){return f[U]!==ot?(n.bindFramebuffer(U,ot),f[U]=ot,U===n.DRAW_FRAMEBUFFER&&(f[n.FRAMEBUFFER]=ot),U===n.FRAMEBUFFER&&(f[n.DRAW_FRAMEBUFFER]=ot),!0):!1}function Dt(U,ot){let J=m,et=!1;if(U){J=p.get(ot),J===void 0&&(J=[],p.set(ot,J));const lt=U.textures;if(J.length!==lt.length||J[0]!==n.COLOR_ATTACHMENT0){for(let bt=0,zt=lt.length;bt<zt;bt++)J[bt]=n.COLOR_ATTACHMENT0+bt;J.length=lt.length,et=!0}}else J[0]!==n.BACK&&(J[0]=n.BACK,et=!0);et&&n.drawBuffers(J)}function Ft(U){return g!==U?(n.useProgram(U),g=U,!0):!1}const Zt={[Hn]:n.FUNC_ADD,[eu]:n.FUNC_SUBTRACT,[nu]:n.FUNC_REVERSE_SUBTRACT};Zt[iu]=n.MIN,Zt[ru]=n.MAX;const D={[su]:n.ZERO,[au]:n.ONE,[ou]:n.SRC_COLOR,[ks]:n.SRC_ALPHA,[du]:n.SRC_ALPHA_SATURATE,[hu]:n.DST_COLOR,[lu]:n.DST_ALPHA,[cu]:n.ONE_MINUS_SRC_COLOR,[Gs]:n.ONE_MINUS_SRC_ALPHA,[fu]:n.ONE_MINUS_DST_COLOR,[uu]:n.ONE_MINUS_DST_ALPHA,[pu]:n.CONSTANT_COLOR,[mu]:n.ONE_MINUS_CONSTANT_COLOR,[_u]:n.CONSTANT_ALPHA,[gu]:n.ONE_MINUS_CONSTANT_ALPHA};function Yt(U,ot,J,et,lt,bt,zt,ne,ue,Vt){if(U===Tn){v===!0&&(pt(n.BLEND),v=!1);return}if(v===!1&&(gt(n.BLEND),v=!0),U!==tu){if(U!==M||Vt!==F){if((_!==Hn||w!==Hn)&&(n.blendEquation(n.FUNC_ADD),_=Hn,w=Hn),Vt)switch(U){case gi:n.blendFuncSeparate(n.ONE,n.ONE_MINUS_SRC_ALPHA,n.ONE,n.ONE_MINUS_SRC_ALPHA);break;case Ja:n.blendFunc(n.ONE,n.ONE);break;case Qa:n.blendFuncSeparate(n.ZERO,n.ONE_MINUS_SRC_COLOR,n.ZERO,n.ONE);break;case to:n.blendFuncSeparate(n.ZERO,n.SRC_COLOR,n.ZERO,n.SRC_ALPHA);break;default:console.error("THREE.WebGLState: Invalid blending: ",U);break}else switch(U){case gi:n.blendFuncSeparate(n.SRC_ALPHA,n.ONE_MINUS_SRC_ALPHA,n.ONE,n.ONE_MINUS_SRC_ALPHA);break;case Ja:n.blendFunc(n.SRC_ALPHA,n.ONE);break;case Qa:n.blendFuncSeparate(n.ZERO,n.ONE_MINUS_SRC_COLOR,n.ZERO,n.ONE);break;case to:n.blendFunc(n.ZERO,n.SRC_COLOR);break;default:console.error("THREE.WebGLState: Invalid blending: ",U);break}d=null,C=null,b=null,z=null,P.set(0,0,0),I=0,M=U,F=Vt}return}lt=lt||ot,bt=bt||J,zt=zt||et,(ot!==_||lt!==w)&&(n.blendEquationSeparate(Zt[ot],Zt[lt]),_=ot,w=lt),(J!==d||et!==C||bt!==b||zt!==z)&&(n.blendFuncSeparate(D[J],D[et],D[bt],D[zt]),d=J,C=et,b=bt,z=zt),(ne.equals(P)===!1||ue!==I)&&(n.blendColor(ne.r,ne.g,ne.b,ue),P.copy(ne),I=ue),M=U,F=!1}function Ot(U,ot){U.side===ge?pt(n.CULL_FACE):gt(n.CULL_FACE);let J=U.side===Se;ot&&(J=!J),Gt(J),U.blending===gi&&U.transparent===!1?Yt(Tn):Yt(U.blending,U.blendEquation,U.blendSrc,U.blendDst,U.blendEquationAlpha,U.blendSrcAlpha,U.blendDstAlpha,U.blendColor,U.blendAlpha,U.premultipliedAlpha),s.setFunc(U.depthFunc),s.setTest(U.depthTest),s.setMask(U.depthWrite),r.setMask(U.colorWrite);const et=U.stencilWrite;o.setTest(et),et&&(o.setMask(U.stencilWriteMask),o.setFunc(U.stencilFunc,U.stencilRef,U.stencilFuncMask),o.setOp(U.stencilFail,U.stencilZFail,U.stencilZPass)),Lt(U.polygonOffset,U.polygonOffsetFactor,U.polygonOffsetUnits),U.alphaToCoverage===!0?gt(n.SAMPLE_ALPHA_TO_COVERAGE):pt(n.SAMPLE_ALPHA_TO_COVERAGE)}function Gt(U){A!==U&&(U?n.frontFace(n.CW):n.frontFace(n.CCW),A=U)}function Mt(U){U!==Zl?(gt(n.CULL_FACE),U!==E&&(U===Za?n.cullFace(n.BACK):U===Jl?n.cullFace(n.FRONT):n.cullFace(n.FRONT_AND_BACK))):pt(n.CULL_FACE),E=U}function Jt(U){U!==N&&(tt&&n.lineWidth(U),N=U)}function Lt(U,ot,J){U?(gt(n.POLYGON_OFFSET_FILL),($!==ot||W!==J)&&(n.polygonOffset(ot,J),$=ot,W=J)):pt(n.POLYGON_OFFSET_FILL)}function It(U){U?gt(n.SCISSOR_TEST):pt(n.SCISSOR_TEST)}function L(U){U===void 0&&(U=n.TEXTURE0+Y-1),j!==U&&(n.activeTexture(U),j=U)}function S(U,ot,J){J===void 0&&(j===null?J=n.TEXTURE0+Y-1:J=j);let et=dt[J];et===void 0&&(et={type:void 0,texture:void 0},dt[J]=et),(et.type!==U||et.texture!==ot)&&(j!==J&&(n.activeTexture(J),j=J),n.bindTexture(U,ot||at[U]),et.type=U,et.texture=ot)}function V(){const U=dt[j];U!==void 0&&U.type!==void 0&&(n.bindTexture(U.type,null),U.type=void 0,U.texture=void 0)}function it(){try{n.compressedTexImage2D.apply(n,arguments)}catch(U){console.error("THREE.WebGLState:",U)}}function rt(){try{n.compressedTexImage3D.apply(n,arguments)}catch(U){console.error("THREE.WebGLState:",U)}}function Q(){try{n.texSubImage2D.apply(n,arguments)}catch(U){console.error("THREE.WebGLState:",U)}}function Et(){try{n.texSubImage3D.apply(n,arguments)}catch(U){console.error("THREE.WebGLState:",U)}}function x(){try{n.compressedTexSubImage2D.apply(n,arguments)}catch(U){console.error("THREE.WebGLState:",U)}}function l(){try{n.compressedTexSubImage3D.apply(n,arguments)}catch(U){console.error("THREE.WebGLState:",U)}}function h(){try{n.texStorage2D.apply(n,arguments)}catch(U){console.error("THREE.WebGLState:",U)}}function y(){try{n.texStorage3D.apply(n,arguments)}catch(U){console.error("THREE.WebGLState:",U)}}function R(){try{n.texImage2D.apply(n,arguments)}catch(U){console.error("THREE.WebGLState:",U)}}function O(){try{n.texImage3D.apply(n,arguments)}catch(U){console.error("THREE.WebGLState:",U)}}function G(U){wt.equals(U)===!1&&(n.scissor(U.x,U.y,U.z,U.w),wt.copy(U))}function nt(U){kt.equals(U)===!1&&(n.viewport(U.x,U.y,U.z,U.w),kt.copy(U))}function mt(U,ot){let J=c.get(ot);J===void 0&&(J=new WeakMap,c.set(ot,J));let et=J.get(U);et===void 0&&(et=n.getUniformBlockIndex(ot,U.name),J.set(U,et))}function xt(U,ot){const et=c.get(ot).get(U);a.get(ot)!==et&&(n.uniformBlockBinding(ot,et,U.__bindingPointIndex),a.set(ot,et))}function Rt(){n.disable(n.BLEND),n.disable(n.CULL_FACE),n.disable(n.DEPTH_TEST),n.disable(n.POLYGON_OFFSET_FILL),n.disable(n.SCISSOR_TEST),n.disable(n.STENCIL_TEST),n.disable(n.SAMPLE_ALPHA_TO_COVERAGE),n.blendEquation(n.FUNC_ADD),n.blendFunc(n.ONE,n.ZERO),n.blendFuncSeparate(n.ONE,n.ZERO,n.ONE,n.ZERO),n.blendColor(0,0,0,0),n.colorMask(!0,!0,!0,!0),n.clearColor(0,0,0,0),n.depthMask(!0),n.depthFunc(n.LESS),n.clearDepth(1),n.stencilMask(4294967295),n.stencilFunc(n.ALWAYS,0,4294967295),n.stencilOp(n.KEEP,n.KEEP,n.KEEP),n.clearStencil(0),n.cullFace(n.BACK),n.frontFace(n.CCW),n.polygonOffset(0,0),n.activeTexture(n.TEXTURE0),n.bindFramebuffer(n.FRAMEBUFFER,null),n.bindFramebuffer(n.DRAW_FRAMEBUFFER,null),n.bindFramebuffer(n.READ_FRAMEBUFFER,null),n.useProgram(null),n.lineWidth(1),n.scissor(0,0,n.canvas.width,n.canvas.height),n.viewport(0,0,n.canvas.width,n.canvas.height),u={},j=null,dt={},f={},p=new WeakMap,m=[],g=null,v=!1,M=null,_=null,d=null,C=null,w=null,b=null,z=null,P=new Ht(0,0,0),I=0,F=!1,A=null,E=null,N=null,$=null,W=null,wt.set(0,0,n.canvas.width,n.canvas.height),kt.set(0,0,n.canvas.width,n.canvas.height),r.reset(),s.reset(),o.reset()}return{buffers:{color:r,depth:s,stencil:o},enable:gt,disable:pt,bindFramebuffer:At,drawBuffers:Dt,useProgram:Ft,setBlending:Yt,setMaterial:Ot,setFlipSided:Gt,setCullFace:Mt,setLineWidth:Jt,setPolygonOffset:Lt,setScissorTest:It,activeTexture:L,bindTexture:S,unbindTexture:V,compressedTexImage2D:it,compressedTexImage3D:rt,texImage2D:R,texImage3D:O,updateUBOMapping:mt,uniformBlockBinding:xt,texStorage2D:h,texStorage3D:y,texSubImage2D:Q,texSubImage3D:Et,compressedTexSubImage2D:x,compressedTexSubImage3D:l,scissor:G,viewport:nt,reset:Rt}}function $o(n,t,e,i){const r=qm(i);switch(e){case Nc:return n*t;case Oc:return n*t;case Bc:return n*t*2;case zc:return n*t/r.components*r.byteLength;case La:return n*t/r.components*r.byteLength;case Hc:return n*t*2/r.components*r.byteLength;case Ia:return n*t*2/r.components*r.byteLength;case Fc:return n*t*3/r.components*r.byteLength;case He:return n*t*4/r.components*r.byteLength;case Da:return n*t*4/r.components*r.byteLength;case Cr:case Rr:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*8;case Pr:case Lr:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*16;case Ys:case js:return Math.max(n,16)*Math.max(t,8)/4;case $s:case Ks:return Math.max(n,8)*Math.max(t,8)/2;case Zs:case Js:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*8;case Qs:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*16;case ta:return Math.floor((n+3)/4)*Math.floor((t+3)/4)*16;case ea:return Math.floor((n+4)/5)*Math.floor((t+3)/4)*16;case na:return Math.floor((n+4)/5)*Math.floor((t+4)/5)*16;case ia:return Math.floor((n+5)/6)*Math.floor((t+4)/5)*16;case ra:return Math.floor((n+5)/6)*Math.floor((t+5)/6)*16;case sa:return Math.floor((n+7)/8)*Math.floor((t+4)/5)*16;case aa:return Math.floor((n+7)/8)*Math.floor((t+5)/6)*16;case oa:return Math.floor((n+7)/8)*Math.floor((t+7)/8)*16;case ca:return Math.floor((n+9)/10)*Math.floor((t+4)/5)*16;case la:return Math.floor((n+9)/10)*Math.floor((t+5)/6)*16;case ua:return Math.floor((n+9)/10)*Math.floor((t+7)/8)*16;case ha:return Math.floor((n+9)/10)*Math.floor((t+9)/10)*16;case fa:return Math.floor((n+11)/12)*Math.floor((t+9)/10)*16;case da:return Math.floor((n+11)/12)*Math.floor((t+11)/12)*16;case Ir:case pa:case ma:return Math.ceil(n/4)*Math.ceil(t/4)*16;case kc:case _a:return Math.ceil(n/4)*Math.ceil(t/4)*8;case ga:case xa:return Math.ceil(n/4)*Math.ceil(t/4)*16}throw new Error(`Unable to determine texture byte length for ${e} format.`)}function qm(n){switch(n){case ln:case Ic:return{byteLength:1,components:1};case Gi:case Dc:case qi:return{byteLength:2,components:1};case Ra:case Pa:return{byteLength:2,components:4};case Wn:case Ca:case nn:return{byteLength:4,components:1};case Uc:return{byteLength:4,components:3}}throw new Error(`Unknown texture type ${n}.`)}function Xm(n,t,e,i,r,s,o){const a=t.has("WEBGL_multisampled_render_to_texture")?t.get("WEBGL_multisampled_render_to_texture"):null,c=typeof navigator>"u"?!1:/OculusBrowser/g.test(navigator.userAgent),u=new Bt,f=new WeakMap;let p;const m=new WeakMap;let g=!1;try{g=typeof OffscreenCanvas<"u"&&new OffscreenCanvas(1,1).getContext("2d")!==null}catch{}function v(L,S){return g?new OffscreenCanvas(L,S):Hr("canvas")}function M(L,S,V){let it=1;const rt=It(L);if((rt.width>V||rt.height>V)&&(it=V/Math.max(rt.width,rt.height)),it<1)if(typeof HTMLImageElement<"u"&&L instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&L instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&L instanceof ImageBitmap||typeof VideoFrame<"u"&&L instanceof VideoFrame){const Q=Math.floor(it*rt.width),Et=Math.floor(it*rt.height);p===void 0&&(p=v(Q,Et));const x=S?v(Q,Et):p;return x.width=Q,x.height=Et,x.getContext("2d").drawImage(L,0,0,Q,Et),console.warn("THREE.WebGLRenderer: Texture has been resized from ("+rt.width+"x"+rt.height+") to ("+Q+"x"+Et+")."),x}else return"data"in L&&console.warn("THREE.WebGLRenderer: Image in DataTexture is too big ("+rt.width+"x"+rt.height+")."),L;return L}function _(L){return L.generateMipmaps&&L.minFilter!==De&&L.minFilter!==Be}function d(L){n.generateMipmap(L)}function C(L,S,V,it,rt=!1){if(L!==null){if(n[L]!==void 0)return n[L];console.warn("THREE.WebGLRenderer: Attempt to use non-existing WebGL internal format '"+L+"'")}let Q=S;if(S===n.RED&&(V===n.FLOAT&&(Q=n.R32F),V===n.HALF_FLOAT&&(Q=n.R16F),V===n.UNSIGNED_BYTE&&(Q=n.R8)),S===n.RED_INTEGER&&(V===n.UNSIGNED_BYTE&&(Q=n.R8UI),V===n.UNSIGNED_SHORT&&(Q=n.R16UI),V===n.UNSIGNED_INT&&(Q=n.R32UI),V===n.BYTE&&(Q=n.R8I),V===n.SHORT&&(Q=n.R16I),V===n.INT&&(Q=n.R32I)),S===n.RG&&(V===n.FLOAT&&(Q=n.RG32F),V===n.HALF_FLOAT&&(Q=n.RG16F),V===n.UNSIGNED_BYTE&&(Q=n.RG8)),S===n.RG_INTEGER&&(V===n.UNSIGNED_BYTE&&(Q=n.RG8UI),V===n.UNSIGNED_SHORT&&(Q=n.RG16UI),V===n.UNSIGNED_INT&&(Q=n.RG32UI),V===n.BYTE&&(Q=n.RG8I),V===n.SHORT&&(Q=n.RG16I),V===n.INT&&(Q=n.RG32I)),S===n.RGB&&V===n.UNSIGNED_INT_5_9_9_9_REV&&(Q=n.RGB9_E5),S===n.RGBA){const Et=rt?Fr:$t.getTransfer(it);V===n.FLOAT&&(Q=n.RGBA32F),V===n.HALF_FLOAT&&(Q=n.RGBA16F),V===n.UNSIGNED_BYTE&&(Q=Et===Kt?n.SRGB8_ALPHA8:n.RGBA8),V===n.UNSIGNED_SHORT_4_4_4_4&&(Q=n.RGBA4),V===n.UNSIGNED_SHORT_5_5_5_1&&(Q=n.RGB5_A1)}return(Q===n.R16F||Q===n.R32F||Q===n.RG16F||Q===n.RG32F||Q===n.RGBA16F||Q===n.RGBA32F)&&t.get("EXT_color_buffer_float"),Q}function w(L,S){let V;return L?S===null||S===Wn||S===Si?V=n.DEPTH24_STENCIL8:S===nn?V=n.DEPTH32F_STENCIL8:S===Gi&&(V=n.DEPTH24_STENCIL8,console.warn("DepthTexture: 16 bit depth attachment is not supported with stencil. Using 24-bit attachment.")):S===null||S===Wn||S===Si?V=n.DEPTH_COMPONENT24:S===nn?V=n.DEPTH_COMPONENT32F:S===Gi&&(V=n.DEPTH_COMPONENT16),V}function b(L,S){return _(L)===!0||L.isFramebufferTexture&&L.minFilter!==De&&L.minFilter!==Be?Math.log2(Math.max(S.width,S.height))+1:L.mipmaps!==void 0&&L.mipmaps.length>0?L.mipmaps.length:L.isCompressedTexture&&Array.isArray(L.image)?S.mipmaps.length:1}function z(L){const S=L.target;S.removeEventListener("dispose",z),I(S),S.isVideoTexture&&f.delete(S)}function P(L){const S=L.target;S.removeEventListener("dispose",P),A(S)}function I(L){const S=i.get(L);if(S.__webglInit===void 0)return;const V=L.source,it=m.get(V);if(it){const rt=it[S.__cacheKey];rt.usedTimes--,rt.usedTimes===0&&F(L),Object.keys(it).length===0&&m.delete(V)}i.remove(L)}function F(L){const S=i.get(L);n.deleteTexture(S.__webglTexture);const V=L.source,it=m.get(V);delete it[S.__cacheKey],o.memory.textures--}function A(L){const S=i.get(L);if(L.depthTexture&&L.depthTexture.dispose(),L.isWebGLCubeRenderTarget)for(let it=0;it<6;it++){if(Array.isArray(S.__webglFramebuffer[it]))for(let rt=0;rt<S.__webglFramebuffer[it].length;rt++)n.deleteFramebuffer(S.__webglFramebuffer[it][rt]);else n.deleteFramebuffer(S.__webglFramebuffer[it]);S.__webglDepthbuffer&&n.deleteRenderbuffer(S.__webglDepthbuffer[it])}else{if(Array.isArray(S.__webglFramebuffer))for(let it=0;it<S.__webglFramebuffer.length;it++)n.deleteFramebuffer(S.__webglFramebuffer[it]);else n.deleteFramebuffer(S.__webglFramebuffer);if(S.__webglDepthbuffer&&n.deleteRenderbuffer(S.__webglDepthbuffer),S.__webglMultisampledFramebuffer&&n.deleteFramebuffer(S.__webglMultisampledFramebuffer),S.__webglColorRenderbuffer)for(let it=0;it<S.__webglColorRenderbuffer.length;it++)S.__webglColorRenderbuffer[it]&&n.deleteRenderbuffer(S.__webglColorRenderbuffer[it]);S.__webglDepthRenderbuffer&&n.deleteRenderbuffer(S.__webglDepthRenderbuffer)}const V=L.textures;for(let it=0,rt=V.length;it<rt;it++){const Q=i.get(V[it]);Q.__webglTexture&&(n.deleteTexture(Q.__webglTexture),o.memory.textures--),i.remove(V[it])}i.remove(L)}let E=0;function N(){E=0}function $(){const L=E;return L>=r.maxTextures&&console.warn("THREE.WebGLTextures: Trying to use "+L+" texture units while this GPU supports only "+r.maxTextures),E+=1,L}function W(L){const S=[];return S.push(L.wrapS),S.push(L.wrapT),S.push(L.wrapR||0),S.push(L.magFilter),S.push(L.minFilter),S.push(L.anisotropy),S.push(L.internalFormat),S.push(L.format),S.push(L.type),S.push(L.generateMipmaps),S.push(L.premultiplyAlpha),S.push(L.flipY),S.push(L.unpackAlignment),S.push(L.colorSpace),S.join()}function Y(L,S){const V=i.get(L);if(L.isVideoTexture&&Jt(L),L.isRenderTargetTexture===!1&&L.version>0&&V.__version!==L.version){const it=L.image;if(it===null)console.warn("THREE.WebGLRenderer: Texture marked for update but no image data found.");else if(it.complete===!1)console.warn("THREE.WebGLRenderer: Texture marked for update but image is incomplete");else{kt(V,L,S);return}}e.bindTexture(n.TEXTURE_2D,V.__webglTexture,n.TEXTURE0+S)}function tt(L,S){const V=i.get(L);if(L.version>0&&V.__version!==L.version){kt(V,L,S);return}e.bindTexture(n.TEXTURE_2D_ARRAY,V.__webglTexture,n.TEXTURE0+S)}function K(L,S){const V=i.get(L);if(L.version>0&&V.__version!==L.version){kt(V,L,S);return}e.bindTexture(n.TEXTURE_3D,V.__webglTexture,n.TEXTURE0+S)}function st(L,S){const V=i.get(L);if(L.version>0&&V.__version!==L.version){Z(V,L,S);return}e.bindTexture(n.TEXTURE_CUBE_MAP,V.__webglTexture,n.TEXTURE0+S)}const j={[qs]:n.REPEAT,[Gn]:n.CLAMP_TO_EDGE,[Xs]:n.MIRRORED_REPEAT},dt={[De]:n.NEAREST,[Uu]:n.NEAREST_MIPMAP_NEAREST,[Ji]:n.NEAREST_MIPMAP_LINEAR,[Be]:n.LINEAR,[as]:n.LINEAR_MIPMAP_NEAREST,[Vn]:n.LINEAR_MIPMAP_LINEAR},_t={[Bu]:n.NEVER,[Wu]:n.ALWAYS,[zu]:n.LESS,[Gc]:n.LEQUAL,[Hu]:n.EQUAL,[Vu]:n.GEQUAL,[ku]:n.GREATER,[Gu]:n.NOTEQUAL};function ut(L,S){if(S.type===nn&&t.has("OES_texture_float_linear")===!1&&(S.magFilter===Be||S.magFilter===as||S.magFilter===Ji||S.magFilter===Vn||S.minFilter===Be||S.minFilter===as||S.minFilter===Ji||S.minFilter===Vn)&&console.warn("THREE.WebGLRenderer: Unable to use linear filtering with floating point textures. OES_texture_float_linear not supported on this device."),n.texParameteri(L,n.TEXTURE_WRAP_S,j[S.wrapS]),n.texParameteri(L,n.TEXTURE_WRAP_T,j[S.wrapT]),(L===n.TEXTURE_3D||L===n.TEXTURE_2D_ARRAY)&&n.texParameteri(L,n.TEXTURE_WRAP_R,j[S.wrapR]),n.texParameteri(L,n.TEXTURE_MAG_FILTER,dt[S.magFilter]),n.texParameteri(L,n.TEXTURE_MIN_FILTER,dt[S.minFilter]),S.compareFunction&&(n.texParameteri(L,n.TEXTURE_COMPARE_MODE,n.COMPARE_REF_TO_TEXTURE),n.texParameteri(L,n.TEXTURE_COMPARE_FUNC,_t[S.compareFunction])),t.has("EXT_texture_filter_anisotropic")===!0){if(S.magFilter===De||S.minFilter!==Ji&&S.minFilter!==Vn||S.type===nn&&t.has("OES_texture_float_linear")===!1)return;if(S.anisotropy>1||i.get(S).__currentAnisotropy){const V=t.get("EXT_texture_filter_anisotropic");n.texParameterf(L,V.TEXTURE_MAX_ANISOTROPY_EXT,Math.min(S.anisotropy,r.getMaxAnisotropy())),i.get(S).__currentAnisotropy=S.anisotropy}}}function wt(L,S){let V=!1;L.__webglInit===void 0&&(L.__webglInit=!0,S.addEventListener("dispose",z));const it=S.source;let rt=m.get(it);rt===void 0&&(rt={},m.set(it,rt));const Q=W(S);if(Q!==L.__cacheKey){rt[Q]===void 0&&(rt[Q]={texture:n.createTexture(),usedTimes:0},o.memory.textures++,V=!0),rt[Q].usedTimes++;const Et=rt[L.__cacheKey];Et!==void 0&&(rt[L.__cacheKey].usedTimes--,Et.usedTimes===0&&F(S)),L.__cacheKey=Q,L.__webglTexture=rt[Q].texture}return V}function kt(L,S,V){let it=n.TEXTURE_2D;(S.isDataArrayTexture||S.isCompressedArrayTexture)&&(it=n.TEXTURE_2D_ARRAY),S.isData3DTexture&&(it=n.TEXTURE_3D);const rt=wt(L,S),Q=S.source;e.bindTexture(it,L.__webglTexture,n.TEXTURE0+V);const Et=i.get(Q);if(Q.version!==Et.__version||rt===!0){e.activeTexture(n.TEXTURE0+V);const x=$t.getPrimaries($t.workingColorSpace),l=S.colorSpace===Sn?null:$t.getPrimaries(S.colorSpace),h=S.colorSpace===Sn||x===l?n.NONE:n.BROWSER_DEFAULT_WEBGL;n.pixelStorei(n.UNPACK_FLIP_Y_WEBGL,S.flipY),n.pixelStorei(n.UNPACK_PREMULTIPLY_ALPHA_WEBGL,S.premultiplyAlpha),n.pixelStorei(n.UNPACK_ALIGNMENT,S.unpackAlignment),n.pixelStorei(n.UNPACK_COLORSPACE_CONVERSION_WEBGL,h);let y=M(S.image,!1,r.maxTextureSize);y=Lt(S,y);const R=s.convert(S.format,S.colorSpace),O=s.convert(S.type);let G=C(S.internalFormat,R,O,S.colorSpace,S.isVideoTexture);ut(it,S);let nt;const mt=S.mipmaps,xt=S.isVideoTexture!==!0,Rt=Et.__version===void 0||rt===!0,U=Q.dataReady,ot=b(S,y);if(S.isDepthTexture)G=w(S.format===Ei,S.type),Rt&&(xt?e.texStorage2D(n.TEXTURE_2D,1,G,y.width,y.height):e.texImage2D(n.TEXTURE_2D,0,G,y.width,y.height,0,R,O,null));else if(S.isDataTexture)if(mt.length>0){xt&&Rt&&e.texStorage2D(n.TEXTURE_2D,ot,G,mt[0].width,mt[0].height);for(let J=0,et=mt.length;J<et;J++)nt=mt[J],xt?U&&e.texSubImage2D(n.TEXTURE_2D,J,0,0,nt.width,nt.height,R,O,nt.data):e.texImage2D(n.TEXTURE_2D,J,G,nt.width,nt.height,0,R,O,nt.data);S.generateMipmaps=!1}else xt?(Rt&&e.texStorage2D(n.TEXTURE_2D,ot,G,y.width,y.height),U&&e.texSubImage2D(n.TEXTURE_2D,0,0,0,y.width,y.height,R,O,y.data)):e.texImage2D(n.TEXTURE_2D,0,G,y.width,y.height,0,R,O,y.data);else if(S.isCompressedTexture)if(S.isCompressedArrayTexture){xt&&Rt&&e.texStorage3D(n.TEXTURE_2D_ARRAY,ot,G,mt[0].width,mt[0].height,y.depth);for(let J=0,et=mt.length;J<et;J++)if(nt=mt[J],S.format!==He)if(R!==null)if(xt){if(U)if(S.layerUpdates.size>0){const lt=$o(nt.width,nt.height,S.format,S.type);for(const bt of S.layerUpdates){const zt=nt.data.subarray(bt*lt/nt.data.BYTES_PER_ELEMENT,(bt+1)*lt/nt.data.BYTES_PER_ELEMENT);e.compressedTexSubImage3D(n.TEXTURE_2D_ARRAY,J,0,0,bt,nt.width,nt.height,1,R,zt,0,0)}S.clearLayerUpdates()}else e.compressedTexSubImage3D(n.TEXTURE_2D_ARRAY,J,0,0,0,nt.width,nt.height,y.depth,R,nt.data,0,0)}else e.compressedTexImage3D(n.TEXTURE_2D_ARRAY,J,G,nt.width,nt.height,y.depth,0,nt.data,0,0);else console.warn("THREE.WebGLRenderer: Attempt to load unsupported compressed texture format in .uploadTexture()");else xt?U&&e.texSubImage3D(n.TEXTURE_2D_ARRAY,J,0,0,0,nt.width,nt.height,y.depth,R,O,nt.data):e.texImage3D(n.TEXTURE_2D_ARRAY,J,G,nt.width,nt.height,y.depth,0,R,O,nt.data)}else{xt&&Rt&&e.texStorage2D(n.TEXTURE_2D,ot,G,mt[0].width,mt[0].height);for(let J=0,et=mt.length;J<et;J++)nt=mt[J],S.format!==He?R!==null?xt?U&&e.compressedTexSubImage2D(n.TEXTURE_2D,J,0,0,nt.width,nt.height,R,nt.data):e.compressedTexImage2D(n.TEXTURE_2D,J,G,nt.width,nt.height,0,nt.data):console.warn("THREE.WebGLRenderer: Attempt to load unsupported compressed texture format in .uploadTexture()"):xt?U&&e.texSubImage2D(n.TEXTURE_2D,J,0,0,nt.width,nt.height,R,O,nt.data):e.texImage2D(n.TEXTURE_2D,J,G,nt.width,nt.height,0,R,O,nt.data)}else if(S.isDataArrayTexture)if(xt){if(Rt&&e.texStorage3D(n.TEXTURE_2D_ARRAY,ot,G,y.width,y.height,y.depth),U)if(S.layerUpdates.size>0){const J=$o(y.width,y.height,S.format,S.type);for(const et of S.layerUpdates){const lt=y.data.subarray(et*J/y.data.BYTES_PER_ELEMENT,(et+1)*J/y.data.BYTES_PER_ELEMENT);e.texSubImage3D(n.TEXTURE_2D_ARRAY,0,0,0,et,y.width,y.height,1,R,O,lt)}S.clearLayerUpdates()}else e.texSubImage3D(n.TEXTURE_2D_ARRAY,0,0,0,0,y.width,y.height,y.depth,R,O,y.data)}else e.texImage3D(n.TEXTURE_2D_ARRAY,0,G,y.width,y.height,y.depth,0,R,O,y.data);else if(S.isData3DTexture)xt?(Rt&&e.texStorage3D(n.TEXTURE_3D,ot,G,y.width,y.height,y.depth),U&&e.texSubImage3D(n.TEXTURE_3D,0,0,0,0,y.width,y.height,y.depth,R,O,y.data)):e.texImage3D(n.TEXTURE_3D,0,G,y.width,y.height,y.depth,0,R,O,y.data);else if(S.isFramebufferTexture){if(Rt)if(xt)e.texStorage2D(n.TEXTURE_2D,ot,G,y.width,y.height);else{let J=y.width,et=y.height;for(let lt=0;lt<ot;lt++)e.texImage2D(n.TEXTURE_2D,lt,G,J,et,0,R,O,null),J>>=1,et>>=1}}else if(mt.length>0){if(xt&&Rt){const J=It(mt[0]);e.texStorage2D(n.TEXTURE_2D,ot,G,J.width,J.height)}for(let J=0,et=mt.length;J<et;J++)nt=mt[J],xt?U&&e.texSubImage2D(n.TEXTURE_2D,J,0,0,R,O,nt):e.texImage2D(n.TEXTURE_2D,J,G,R,O,nt);S.generateMipmaps=!1}else if(xt){if(Rt){const J=It(y);e.texStorage2D(n.TEXTURE_2D,ot,G,J.width,J.height)}U&&e.texSubImage2D(n.TEXTURE_2D,0,0,0,R,O,y)}else e.texImage2D(n.TEXTURE_2D,0,G,R,O,y);_(S)&&d(it),Et.__version=Q.version,S.onUpdate&&S.onUpdate(S)}L.__version=S.version}function Z(L,S,V){if(S.image.length!==6)return;const it=wt(L,S),rt=S.source;e.bindTexture(n.TEXTURE_CUBE_MAP,L.__webglTexture,n.TEXTURE0+V);const Q=i.get(rt);if(rt.version!==Q.__version||it===!0){e.activeTexture(n.TEXTURE0+V);const Et=$t.getPrimaries($t.workingColorSpace),x=S.colorSpace===Sn?null:$t.getPrimaries(S.colorSpace),l=S.colorSpace===Sn||Et===x?n.NONE:n.BROWSER_DEFAULT_WEBGL;n.pixelStorei(n.UNPACK_FLIP_Y_WEBGL,S.flipY),n.pixelStorei(n.UNPACK_PREMULTIPLY_ALPHA_WEBGL,S.premultiplyAlpha),n.pixelStorei(n.UNPACK_ALIGNMENT,S.unpackAlignment),n.pixelStorei(n.UNPACK_COLORSPACE_CONVERSION_WEBGL,l);const h=S.isCompressedTexture||S.image[0].isCompressedTexture,y=S.image[0]&&S.image[0].isDataTexture,R=[];for(let et=0;et<6;et++)!h&&!y?R[et]=M(S.image[et],!0,r.maxCubemapSize):R[et]=y?S.image[et].image:S.image[et],R[et]=Lt(S,R[et]);const O=R[0],G=s.convert(S.format,S.colorSpace),nt=s.convert(S.type),mt=C(S.internalFormat,G,nt,S.colorSpace),xt=S.isVideoTexture!==!0,Rt=Q.__version===void 0||it===!0,U=rt.dataReady;let ot=b(S,O);ut(n.TEXTURE_CUBE_MAP,S);let J;if(h){xt&&Rt&&e.texStorage2D(n.TEXTURE_CUBE_MAP,ot,mt,O.width,O.height);for(let et=0;et<6;et++){J=R[et].mipmaps;for(let lt=0;lt<J.length;lt++){const bt=J[lt];S.format!==He?G!==null?xt?U&&e.compressedTexSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+et,lt,0,0,bt.width,bt.height,G,bt.data):e.compressedTexImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+et,lt,mt,bt.width,bt.height,0,bt.data):console.warn("THREE.WebGLRenderer: Attempt to load unsupported compressed texture format in .setTextureCube()"):xt?U&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+et,lt,0,0,bt.width,bt.height,G,nt,bt.data):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+et,lt,mt,bt.width,bt.height,0,G,nt,bt.data)}}}else{if(J=S.mipmaps,xt&&Rt){J.length>0&&ot++;const et=It(R[0]);e.texStorage2D(n.TEXTURE_CUBE_MAP,ot,mt,et.width,et.height)}for(let et=0;et<6;et++)if(y){xt?U&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+et,0,0,0,R[et].width,R[et].height,G,nt,R[et].data):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+et,0,mt,R[et].width,R[et].height,0,G,nt,R[et].data);for(let lt=0;lt<J.length;lt++){const zt=J[lt].image[et].image;xt?U&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+et,lt+1,0,0,zt.width,zt.height,G,nt,zt.data):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+et,lt+1,mt,zt.width,zt.height,0,G,nt,zt.data)}}else{xt?U&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+et,0,0,0,G,nt,R[et]):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+et,0,mt,G,nt,R[et]);for(let lt=0;lt<J.length;lt++){const bt=J[lt];xt?U&&e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+et,lt+1,0,0,G,nt,bt.image[et]):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+et,lt+1,mt,G,nt,bt.image[et])}}}_(S)&&d(n.TEXTURE_CUBE_MAP),Q.__version=rt.version,S.onUpdate&&S.onUpdate(S)}L.__version=S.version}function at(L,S,V,it,rt,Q){const Et=s.convert(V.format,V.colorSpace),x=s.convert(V.type),l=C(V.internalFormat,Et,x,V.colorSpace);if(!i.get(S).__hasExternalTextures){const y=Math.max(1,S.width>>Q),R=Math.max(1,S.height>>Q);rt===n.TEXTURE_3D||rt===n.TEXTURE_2D_ARRAY?e.texImage3D(rt,Q,l,y,R,S.depth,0,Et,x,null):e.texImage2D(rt,Q,l,y,R,0,Et,x,null)}e.bindFramebuffer(n.FRAMEBUFFER,L),Mt(S)?a.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,it,rt,i.get(V).__webglTexture,0,Gt(S)):(rt===n.TEXTURE_2D||rt>=n.TEXTURE_CUBE_MAP_POSITIVE_X&&rt<=n.TEXTURE_CUBE_MAP_NEGATIVE_Z)&&n.framebufferTexture2D(n.FRAMEBUFFER,it,rt,i.get(V).__webglTexture,Q),e.bindFramebuffer(n.FRAMEBUFFER,null)}function gt(L,S,V){if(n.bindRenderbuffer(n.RENDERBUFFER,L),S.depthBuffer){const it=S.depthTexture,rt=it&&it.isDepthTexture?it.type:null,Q=w(S.stencilBuffer,rt),Et=S.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT,x=Gt(S);Mt(S)?a.renderbufferStorageMultisampleEXT(n.RENDERBUFFER,x,Q,S.width,S.height):V?n.renderbufferStorageMultisample(n.RENDERBUFFER,x,Q,S.width,S.height):n.renderbufferStorage(n.RENDERBUFFER,Q,S.width,S.height),n.framebufferRenderbuffer(n.FRAMEBUFFER,Et,n.RENDERBUFFER,L)}else{const it=S.textures;for(let rt=0;rt<it.length;rt++){const Q=it[rt],Et=s.convert(Q.format,Q.colorSpace),x=s.convert(Q.type),l=C(Q.internalFormat,Et,x,Q.colorSpace),h=Gt(S);V&&Mt(S)===!1?n.renderbufferStorageMultisample(n.RENDERBUFFER,h,l,S.width,S.height):Mt(S)?a.renderbufferStorageMultisampleEXT(n.RENDERBUFFER,h,l,S.width,S.height):n.renderbufferStorage(n.RENDERBUFFER,l,S.width,S.height)}}n.bindRenderbuffer(n.RENDERBUFFER,null)}function pt(L,S){if(S&&S.isWebGLCubeRenderTarget)throw new Error("Depth Texture with cube render targets is not supported");if(e.bindFramebuffer(n.FRAMEBUFFER,L),!(S.depthTexture&&S.depthTexture.isDepthTexture))throw new Error("renderTarget.depthTexture must be an instance of THREE.DepthTexture");(!i.get(S.depthTexture).__webglTexture||S.depthTexture.image.width!==S.width||S.depthTexture.image.height!==S.height)&&(S.depthTexture.image.width=S.width,S.depthTexture.image.height=S.height,S.depthTexture.needsUpdate=!0),Y(S.depthTexture,0);const it=i.get(S.depthTexture).__webglTexture,rt=Gt(S);if(S.depthTexture.format===xi)Mt(S)?a.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,n.DEPTH_ATTACHMENT,n.TEXTURE_2D,it,0,rt):n.framebufferTexture2D(n.FRAMEBUFFER,n.DEPTH_ATTACHMENT,n.TEXTURE_2D,it,0);else if(S.depthTexture.format===Ei)Mt(S)?a.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,n.DEPTH_STENCIL_ATTACHMENT,n.TEXTURE_2D,it,0,rt):n.framebufferTexture2D(n.FRAMEBUFFER,n.DEPTH_STENCIL_ATTACHMENT,n.TEXTURE_2D,it,0);else throw new Error("Unknown depthTexture format")}function At(L){const S=i.get(L),V=L.isWebGLCubeRenderTarget===!0;if(L.depthTexture&&!S.__autoAllocateDepthBuffer){if(V)throw new Error("target.depthTexture not supported in Cube render targets");pt(S.__webglFramebuffer,L)}else if(V){S.__webglDepthbuffer=[];for(let it=0;it<6;it++)e.bindFramebuffer(n.FRAMEBUFFER,S.__webglFramebuffer[it]),S.__webglDepthbuffer[it]=n.createRenderbuffer(),gt(S.__webglDepthbuffer[it],L,!1)}else e.bindFramebuffer(n.FRAMEBUFFER,S.__webglFramebuffer),S.__webglDepthbuffer=n.createRenderbuffer(),gt(S.__webglDepthbuffer,L,!1);e.bindFramebuffer(n.FRAMEBUFFER,null)}function Dt(L,S,V){const it=i.get(L);S!==void 0&&at(it.__webglFramebuffer,L,L.texture,n.COLOR_ATTACHMENT0,n.TEXTURE_2D,0),V!==void 0&&At(L)}function Ft(L){const S=L.texture,V=i.get(L),it=i.get(S);L.addEventListener("dispose",P);const rt=L.textures,Q=L.isWebGLCubeRenderTarget===!0,Et=rt.length>1;if(Et||(it.__webglTexture===void 0&&(it.__webglTexture=n.createTexture()),it.__version=S.version,o.memory.textures++),Q){V.__webglFramebuffer=[];for(let x=0;x<6;x++)if(S.mipmaps&&S.mipmaps.length>0){V.__webglFramebuffer[x]=[];for(let l=0;l<S.mipmaps.length;l++)V.__webglFramebuffer[x][l]=n.createFramebuffer()}else V.__webglFramebuffer[x]=n.createFramebuffer()}else{if(S.mipmaps&&S.mipmaps.length>0){V.__webglFramebuffer=[];for(let x=0;x<S.mipmaps.length;x++)V.__webglFramebuffer[x]=n.createFramebuffer()}else V.__webglFramebuffer=n.createFramebuffer();if(Et)for(let x=0,l=rt.length;x<l;x++){const h=i.get(rt[x]);h.__webglTexture===void 0&&(h.__webglTexture=n.createTexture(),o.memory.textures++)}if(L.samples>0&&Mt(L)===!1){V.__webglMultisampledFramebuffer=n.createFramebuffer(),V.__webglColorRenderbuffer=[],e.bindFramebuffer(n.FRAMEBUFFER,V.__webglMultisampledFramebuffer);for(let x=0;x<rt.length;x++){const l=rt[x];V.__webglColorRenderbuffer[x]=n.createRenderbuffer(),n.bindRenderbuffer(n.RENDERBUFFER,V.__webglColorRenderbuffer[x]);const h=s.convert(l.format,l.colorSpace),y=s.convert(l.type),R=C(l.internalFormat,h,y,l.colorSpace,L.isXRRenderTarget===!0),O=Gt(L);n.renderbufferStorageMultisample(n.RENDERBUFFER,O,R,L.width,L.height),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+x,n.RENDERBUFFER,V.__webglColorRenderbuffer[x])}n.bindRenderbuffer(n.RENDERBUFFER,null),L.depthBuffer&&(V.__webglDepthRenderbuffer=n.createRenderbuffer(),gt(V.__webglDepthRenderbuffer,L,!0)),e.bindFramebuffer(n.FRAMEBUFFER,null)}}if(Q){e.bindTexture(n.TEXTURE_CUBE_MAP,it.__webglTexture),ut(n.TEXTURE_CUBE_MAP,S);for(let x=0;x<6;x++)if(S.mipmaps&&S.mipmaps.length>0)for(let l=0;l<S.mipmaps.length;l++)at(V.__webglFramebuffer[x][l],L,S,n.COLOR_ATTACHMENT0,n.TEXTURE_CUBE_MAP_POSITIVE_X+x,l);else at(V.__webglFramebuffer[x],L,S,n.COLOR_ATTACHMENT0,n.TEXTURE_CUBE_MAP_POSITIVE_X+x,0);_(S)&&d(n.TEXTURE_CUBE_MAP),e.unbindTexture()}else if(Et){for(let x=0,l=rt.length;x<l;x++){const h=rt[x],y=i.get(h);e.bindTexture(n.TEXTURE_2D,y.__webglTexture),ut(n.TEXTURE_2D,h),at(V.__webglFramebuffer,L,h,n.COLOR_ATTACHMENT0+x,n.TEXTURE_2D,0),_(h)&&d(n.TEXTURE_2D)}e.unbindTexture()}else{let x=n.TEXTURE_2D;if((L.isWebGL3DRenderTarget||L.isWebGLArrayRenderTarget)&&(x=L.isWebGL3DRenderTarget?n.TEXTURE_3D:n.TEXTURE_2D_ARRAY),e.bindTexture(x,it.__webglTexture),ut(x,S),S.mipmaps&&S.mipmaps.length>0)for(let l=0;l<S.mipmaps.length;l++)at(V.__webglFramebuffer[l],L,S,n.COLOR_ATTACHMENT0,x,l);else at(V.__webglFramebuffer,L,S,n.COLOR_ATTACHMENT0,x,0);_(S)&&d(x),e.unbindTexture()}L.depthBuffer&&At(L)}function Zt(L){const S=L.textures;for(let V=0,it=S.length;V<it;V++){const rt=S[V];if(_(rt)){const Q=L.isWebGLCubeRenderTarget?n.TEXTURE_CUBE_MAP:n.TEXTURE_2D,Et=i.get(rt).__webglTexture;e.bindTexture(Q,Et),d(Q),e.unbindTexture()}}}const D=[],Yt=[];function Ot(L){if(L.samples>0){if(Mt(L)===!1){const S=L.textures,V=L.width,it=L.height;let rt=n.COLOR_BUFFER_BIT;const Q=L.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT,Et=i.get(L),x=S.length>1;if(x)for(let l=0;l<S.length;l++)e.bindFramebuffer(n.FRAMEBUFFER,Et.__webglMultisampledFramebuffer),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+l,n.RENDERBUFFER,null),e.bindFramebuffer(n.FRAMEBUFFER,Et.__webglFramebuffer),n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0+l,n.TEXTURE_2D,null,0);e.bindFramebuffer(n.READ_FRAMEBUFFER,Et.__webglMultisampledFramebuffer),e.bindFramebuffer(n.DRAW_FRAMEBUFFER,Et.__webglFramebuffer);for(let l=0;l<S.length;l++){if(L.resolveDepthBuffer&&(L.depthBuffer&&(rt|=n.DEPTH_BUFFER_BIT),L.stencilBuffer&&L.resolveStencilBuffer&&(rt|=n.STENCIL_BUFFER_BIT)),x){n.framebufferRenderbuffer(n.READ_FRAMEBUFFER,n.COLOR_ATTACHMENT0,n.RENDERBUFFER,Et.__webglColorRenderbuffer[l]);const h=i.get(S[l]).__webglTexture;n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0,n.TEXTURE_2D,h,0)}n.blitFramebuffer(0,0,V,it,0,0,V,it,rt,n.NEAREST),c===!0&&(D.length=0,Yt.length=0,D.push(n.COLOR_ATTACHMENT0+l),L.depthBuffer&&L.resolveDepthBuffer===!1&&(D.push(Q),Yt.push(Q),n.invalidateFramebuffer(n.DRAW_FRAMEBUFFER,Yt)),n.invalidateFramebuffer(n.READ_FRAMEBUFFER,D))}if(e.bindFramebuffer(n.READ_FRAMEBUFFER,null),e.bindFramebuffer(n.DRAW_FRAMEBUFFER,null),x)for(let l=0;l<S.length;l++){e.bindFramebuffer(n.FRAMEBUFFER,Et.__webglMultisampledFramebuffer),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+l,n.RENDERBUFFER,Et.__webglColorRenderbuffer[l]);const h=i.get(S[l]).__webglTexture;e.bindFramebuffer(n.FRAMEBUFFER,Et.__webglFramebuffer),n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0+l,n.TEXTURE_2D,h,0)}e.bindFramebuffer(n.DRAW_FRAMEBUFFER,Et.__webglMultisampledFramebuffer)}else if(L.depthBuffer&&L.resolveDepthBuffer===!1&&c){const S=L.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT;n.invalidateFramebuffer(n.DRAW_FRAMEBUFFER,[S])}}}function Gt(L){return Math.min(r.maxSamples,L.samples)}function Mt(L){const S=i.get(L);return L.samples>0&&t.has("WEBGL_multisampled_render_to_texture")===!0&&S.__useRenderToTexture!==!1}function Jt(L){const S=o.render.frame;f.get(L)!==S&&(f.set(L,S),L.update())}function Lt(L,S){const V=L.colorSpace,it=L.format,rt=L.type;return L.isCompressedTexture===!0||L.isVideoTexture===!0||V!==Rn&&V!==Sn&&($t.getTransfer(V)===Kt?(it!==He||rt!==ln)&&console.warn("THREE.WebGLTextures: sRGB encoded textures have to use RGBAFormat and UnsignedByteType."):console.error("THREE.WebGLTextures: Unsupported texture color space:",V)),S}function It(L){return typeof HTMLImageElement<"u"&&L instanceof HTMLImageElement?(u.width=L.naturalWidth||L.width,u.height=L.naturalHeight||L.height):typeof VideoFrame<"u"&&L instanceof VideoFrame?(u.width=L.displayWidth,u.height=L.displayHeight):(u.width=L.width,u.height=L.height),u}this.allocateTextureUnit=$,this.resetTextureUnits=N,this.setTexture2D=Y,this.setTexture2DArray=tt,this.setTexture3D=K,this.setTextureCube=st,this.rebindTextures=Dt,this.setupRenderTarget=Ft,this.updateRenderTargetMipmap=Zt,this.updateMultisampleRenderTarget=Ot,this.setupDepthRenderbuffer=At,this.setupFrameBufferTexture=at,this.useMultisampledRTT=Mt}function $m(n,t){function e(i,r=Sn){let s;const o=$t.getTransfer(r);if(i===ln)return n.UNSIGNED_BYTE;if(i===Ra)return n.UNSIGNED_SHORT_4_4_4_4;if(i===Pa)return n.UNSIGNED_SHORT_5_5_5_1;if(i===Uc)return n.UNSIGNED_INT_5_9_9_9_REV;if(i===Ic)return n.BYTE;if(i===Dc)return n.SHORT;if(i===Gi)return n.UNSIGNED_SHORT;if(i===Ca)return n.INT;if(i===Wn)return n.UNSIGNED_INT;if(i===nn)return n.FLOAT;if(i===qi)return n.HALF_FLOAT;if(i===Nc)return n.ALPHA;if(i===Fc)return n.RGB;if(i===He)return n.RGBA;if(i===Oc)return n.LUMINANCE;if(i===Bc)return n.LUMINANCE_ALPHA;if(i===xi)return n.DEPTH_COMPONENT;if(i===Ei)return n.DEPTH_STENCIL;if(i===zc)return n.RED;if(i===La)return n.RED_INTEGER;if(i===Hc)return n.RG;if(i===Ia)return n.RG_INTEGER;if(i===Da)return n.RGBA_INTEGER;if(i===Cr||i===Rr||i===Pr||i===Lr)if(o===Kt)if(s=t.get("WEBGL_compressed_texture_s3tc_srgb"),s!==null){if(i===Cr)return s.COMPRESSED_SRGB_S3TC_DXT1_EXT;if(i===Rr)return s.COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT;if(i===Pr)return s.COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT;if(i===Lr)return s.COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT}else return null;else if(s=t.get("WEBGL_compressed_texture_s3tc"),s!==null){if(i===Cr)return s.COMPRESSED_RGB_S3TC_DXT1_EXT;if(i===Rr)return s.COMPRESSED_RGBA_S3TC_DXT1_EXT;if(i===Pr)return s.COMPRESSED_RGBA_S3TC_DXT3_EXT;if(i===Lr)return s.COMPRESSED_RGBA_S3TC_DXT5_EXT}else return null;if(i===$s||i===Ys||i===Ks||i===js)if(s=t.get("WEBGL_compressed_texture_pvrtc"),s!==null){if(i===$s)return s.COMPRESSED_RGB_PVRTC_4BPPV1_IMG;if(i===Ys)return s.COMPRESSED_RGB_PVRTC_2BPPV1_IMG;if(i===Ks)return s.COMPRESSED_RGBA_PVRTC_4BPPV1_IMG;if(i===js)return s.COMPRESSED_RGBA_PVRTC_2BPPV1_IMG}else return null;if(i===Zs||i===Js||i===Qs)if(s=t.get("WEBGL_compressed_texture_etc"),s!==null){if(i===Zs||i===Js)return o===Kt?s.COMPRESSED_SRGB8_ETC2:s.COMPRESSED_RGB8_ETC2;if(i===Qs)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ETC2_EAC:s.COMPRESSED_RGBA8_ETC2_EAC}else return null;if(i===ta||i===ea||i===na||i===ia||i===ra||i===sa||i===aa||i===oa||i===ca||i===la||i===ua||i===ha||i===fa||i===da)if(s=t.get("WEBGL_compressed_texture_astc"),s!==null){if(i===ta)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR:s.COMPRESSED_RGBA_ASTC_4x4_KHR;if(i===ea)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR:s.COMPRESSED_RGBA_ASTC_5x4_KHR;if(i===na)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR:s.COMPRESSED_RGBA_ASTC_5x5_KHR;if(i===ia)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR:s.COMPRESSED_RGBA_ASTC_6x5_KHR;if(i===ra)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR:s.COMPRESSED_RGBA_ASTC_6x6_KHR;if(i===sa)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR:s.COMPRESSED_RGBA_ASTC_8x5_KHR;if(i===aa)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR:s.COMPRESSED_RGBA_ASTC_8x6_KHR;if(i===oa)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR:s.COMPRESSED_RGBA_ASTC_8x8_KHR;if(i===ca)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR:s.COMPRESSED_RGBA_ASTC_10x5_KHR;if(i===la)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR:s.COMPRESSED_RGBA_ASTC_10x6_KHR;if(i===ua)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR:s.COMPRESSED_RGBA_ASTC_10x8_KHR;if(i===ha)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR:s.COMPRESSED_RGBA_ASTC_10x10_KHR;if(i===fa)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR:s.COMPRESSED_RGBA_ASTC_12x10_KHR;if(i===da)return o===Kt?s.COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR:s.COMPRESSED_RGBA_ASTC_12x12_KHR}else return null;if(i===Ir||i===pa||i===ma)if(s=t.get("EXT_texture_compression_bptc"),s!==null){if(i===Ir)return o===Kt?s.COMPRESSED_SRGB_ALPHA_BPTC_UNORM_EXT:s.COMPRESSED_RGBA_BPTC_UNORM_EXT;if(i===pa)return s.COMPRESSED_RGB_BPTC_SIGNED_FLOAT_EXT;if(i===ma)return s.COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_EXT}else return null;if(i===kc||i===_a||i===ga||i===xa)if(s=t.get("EXT_texture_compression_rgtc"),s!==null){if(i===Ir)return s.COMPRESSED_RED_RGTC1_EXT;if(i===_a)return s.COMPRESSED_SIGNED_RED_RGTC1_EXT;if(i===ga)return s.COMPRESSED_RED_GREEN_RGTC2_EXT;if(i===xa)return s.COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT}else return null;return i===Si?n.UNSIGNED_INT_24_8:n[i]!==void 0?n[i]:null}return{convert:e}}class Ym extends Ie{constructor(t=[]){super(),this.isArrayCamera=!0,this.cameras=t}}class En extends fe{constructor(){super(),this.isGroup=!0,this.type="Group"}}const Km={type:"move"};class Ds{constructor(){this._targetRay=null,this._grip=null,this._hand=null}getHandSpace(){return this._hand===null&&(this._hand=new En,this._hand.matrixAutoUpdate=!1,this._hand.visible=!1,this._hand.joints={},this._hand.inputState={pinching:!1}),this._hand}getTargetRaySpace(){return this._targetRay===null&&(this._targetRay=new En,this._targetRay.matrixAutoUpdate=!1,this._targetRay.visible=!1,this._targetRay.hasLinearVelocity=!1,this._targetRay.linearVelocity=new k,this._targetRay.hasAngularVelocity=!1,this._targetRay.angularVelocity=new k),this._targetRay}getGripSpace(){return this._grip===null&&(this._grip=new En,this._grip.matrixAutoUpdate=!1,this._grip.visible=!1,this._grip.hasLinearVelocity=!1,this._grip.linearVelocity=new k,this._grip.hasAngularVelocity=!1,this._grip.angularVelocity=new k),this._grip}dispatchEvent(t){return this._targetRay!==null&&this._targetRay.dispatchEvent(t),this._grip!==null&&this._grip.dispatchEvent(t),this._hand!==null&&this._hand.dispatchEvent(t),this}connect(t){if(t&&t.hand){const e=this._hand;if(e)for(const i of t.hand.values())this._getHandJoint(e,i)}return this.dispatchEvent({type:"connected",data:t}),this}disconnect(t){return this.dispatchEvent({type:"disconnected",data:t}),this._targetRay!==null&&(this._targetRay.visible=!1),this._grip!==null&&(this._grip.visible=!1),this._hand!==null&&(this._hand.visible=!1),this}update(t,e,i){let r=null,s=null,o=null;const a=this._targetRay,c=this._grip,u=this._hand;if(t&&e.session.visibilityState!=="visible-blurred"){if(u&&t.hand){o=!0;for(const M of t.hand.values()){const _=e.getJointPose(M,i),d=this._getHandJoint(u,M);_!==null&&(d.matrix.fromArray(_.transform.matrix),d.matrix.decompose(d.position,d.rotation,d.scale),d.matrixWorldNeedsUpdate=!0,d.jointRadius=_.radius),d.visible=_!==null}const f=u.joints["index-finger-tip"],p=u.joints["thumb-tip"],m=f.position.distanceTo(p.position),g=.02,v=.005;u.inputState.pinching&&m>g+v?(u.inputState.pinching=!1,this.dispatchEvent({type:"pinchend",handedness:t.handedness,target:this})):!u.inputState.pinching&&m<=g-v&&(u.inputState.pinching=!0,this.dispatchEvent({type:"pinchstart",handedness:t.handedness,target:this}))}else c!==null&&t.gripSpace&&(s=e.getPose(t.gripSpace,i),s!==null&&(c.matrix.fromArray(s.transform.matrix),c.matrix.decompose(c.position,c.rotation,c.scale),c.matrixWorldNeedsUpdate=!0,s.linearVelocity?(c.hasLinearVelocity=!0,c.linearVelocity.copy(s.linearVelocity)):c.hasLinearVelocity=!1,s.angularVelocity?(c.hasAngularVelocity=!0,c.angularVelocity.copy(s.angularVelocity)):c.hasAngularVelocity=!1));a!==null&&(r=e.getPose(t.targetRaySpace,i),r===null&&s!==null&&(r=s),r!==null&&(a.matrix.fromArray(r.transform.matrix),a.matrix.decompose(a.position,a.rotation,a.scale),a.matrixWorldNeedsUpdate=!0,r.linearVelocity?(a.hasLinearVelocity=!0,a.linearVelocity.copy(r.linearVelocity)):a.hasLinearVelocity=!1,r.angularVelocity?(a.hasAngularVelocity=!0,a.angularVelocity.copy(r.angularVelocity)):a.hasAngularVelocity=!1,this.dispatchEvent(Km)))}return a!==null&&(a.visible=r!==null),c!==null&&(c.visible=s!==null),u!==null&&(u.visible=o!==null),this}_getHandJoint(t,e){if(t.joints[e.jointName]===void 0){const i=new En;i.matrixAutoUpdate=!1,i.visible=!1,t.joints[e.jointName]=i,t.add(i)}return t.joints[e.jointName]}}const jm=`
void main() {

	gl_Position = vec4( position, 1.0 );

}`,Zm=`
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

}`;class Jm{constructor(){this.texture=null,this.mesh=null,this.depthNear=0,this.depthFar=0}init(t,e,i){if(this.texture===null){const r=new Ee,s=t.properties.get(r);s.__webglTexture=e.texture,(e.depthNear!=i.depthNear||e.depthFar!=i.depthFar)&&(this.depthNear=e.depthNear,this.depthFar=e.depthFar),this.texture=r}}getMesh(t){if(this.texture!==null&&this.mesh===null){const e=t.cameras[0].viewport,i=new bn({vertexShader:jm,fragmentShader:Zm,uniforms:{depthColor:{value:this.texture},depthWidth:{value:e.z},depthHeight:{value:e.w}}});this.mesh=new ae(new Ki(20,20),i)}return this.mesh}reset(){this.texture=null,this.mesh=null}getDepthTexture(){return this.texture}}class Qm extends Ai{constructor(t,e){super();const i=this;let r=null,s=1,o=null,a="local-floor",c=1,u=null,f=null,p=null,m=null,g=null,v=null;const M=new Jm,_=e.getContextAttributes();let d=null,C=null;const w=[],b=[],z=new Bt;let P=null;const I=new Ie;I.layers.enable(1),I.viewport=new se;const F=new Ie;F.layers.enable(2),F.viewport=new se;const A=[I,F],E=new Ym;E.layers.enable(1),E.layers.enable(2);let N=null,$=null;this.cameraAutoUpdate=!0,this.enabled=!1,this.isPresenting=!1,this.getController=function(Z){let at=w[Z];return at===void 0&&(at=new Ds,w[Z]=at),at.getTargetRaySpace()},this.getControllerGrip=function(Z){let at=w[Z];return at===void 0&&(at=new Ds,w[Z]=at),at.getGripSpace()},this.getHand=function(Z){let at=w[Z];return at===void 0&&(at=new Ds,w[Z]=at),at.getHandSpace()};function W(Z){const at=b.indexOf(Z.inputSource);if(at===-1)return;const gt=w[at];gt!==void 0&&(gt.update(Z.inputSource,Z.frame,u||o),gt.dispatchEvent({type:Z.type,data:Z.inputSource}))}function Y(){r.removeEventListener("select",W),r.removeEventListener("selectstart",W),r.removeEventListener("selectend",W),r.removeEventListener("squeeze",W),r.removeEventListener("squeezestart",W),r.removeEventListener("squeezeend",W),r.removeEventListener("end",Y),r.removeEventListener("inputsourceschange",tt);for(let Z=0;Z<w.length;Z++){const at=b[Z];at!==null&&(b[Z]=null,w[Z].disconnect(at))}N=null,$=null,M.reset(),t.setRenderTarget(d),g=null,m=null,p=null,r=null,C=null,kt.stop(),i.isPresenting=!1,t.setPixelRatio(P),t.setSize(z.width,z.height,!1),i.dispatchEvent({type:"sessionend"})}this.setFramebufferScaleFactor=function(Z){s=Z,i.isPresenting===!0&&console.warn("THREE.WebXRManager: Cannot change framebuffer scale while presenting.")},this.setReferenceSpaceType=function(Z){a=Z,i.isPresenting===!0&&console.warn("THREE.WebXRManager: Cannot change reference space type while presenting.")},this.getReferenceSpace=function(){return u||o},this.setReferenceSpace=function(Z){u=Z},this.getBaseLayer=function(){return m!==null?m:g},this.getBinding=function(){return p},this.getFrame=function(){return v},this.getSession=function(){return r},this.setSession=async function(Z){if(r=Z,r!==null){if(d=t.getRenderTarget(),r.addEventListener("select",W),r.addEventListener("selectstart",W),r.addEventListener("selectend",W),r.addEventListener("squeeze",W),r.addEventListener("squeezestart",W),r.addEventListener("squeezeend",W),r.addEventListener("end",Y),r.addEventListener("inputsourceschange",tt),_.xrCompatible!==!0&&await e.makeXRCompatible(),P=t.getPixelRatio(),t.getSize(z),r.renderState.layers===void 0){const at={antialias:_.antialias,alpha:!0,depth:_.depth,stencil:_.stencil,framebufferScaleFactor:s};g=new XRWebGLLayer(r,e,at),r.updateRenderState({baseLayer:g}),t.setPixelRatio(1),t.setSize(g.framebufferWidth,g.framebufferHeight,!1),C=new qn(g.framebufferWidth,g.framebufferHeight,{format:He,type:ln,colorSpace:t.outputColorSpace,stencilBuffer:_.stencil})}else{let at=null,gt=null,pt=null;_.depth&&(pt=_.stencil?e.DEPTH24_STENCIL8:e.DEPTH_COMPONENT24,at=_.stencil?Ei:xi,gt=_.stencil?Si:Wn);const At={colorFormat:e.RGBA8,depthFormat:pt,scaleFactor:s};p=new XRWebGLBinding(r,e),m=p.createProjectionLayer(At),r.updateRenderState({layers:[m]}),t.setPixelRatio(1),t.setSize(m.textureWidth,m.textureHeight,!1),C=new qn(m.textureWidth,m.textureHeight,{format:He,type:ln,depthTexture:new nl(m.textureWidth,m.textureHeight,gt,void 0,void 0,void 0,void 0,void 0,void 0,at),stencilBuffer:_.stencil,colorSpace:t.outputColorSpace,samples:_.antialias?4:0,resolveDepthBuffer:m.ignoreDepthValues===!1})}C.isXRRenderTarget=!0,this.setFoveation(c),u=null,o=await r.requestReferenceSpace(a),kt.setContext(r),kt.start(),i.isPresenting=!0,i.dispatchEvent({type:"sessionstart"})}},this.getEnvironmentBlendMode=function(){if(r!==null)return r.environmentBlendMode},this.getDepthTexture=function(){return M.getDepthTexture()};function tt(Z){for(let at=0;at<Z.removed.length;at++){const gt=Z.removed[at],pt=b.indexOf(gt);pt>=0&&(b[pt]=null,w[pt].disconnect(gt))}for(let at=0;at<Z.added.length;at++){const gt=Z.added[at];let pt=b.indexOf(gt);if(pt===-1){for(let Dt=0;Dt<w.length;Dt++)if(Dt>=b.length){b.push(gt),pt=Dt;break}else if(b[Dt]===null){b[Dt]=gt,pt=Dt;break}if(pt===-1)break}const At=w[pt];At&&At.connect(gt)}}const K=new k,st=new k;function j(Z,at,gt){K.setFromMatrixPosition(at.matrixWorld),st.setFromMatrixPosition(gt.matrixWorld);const pt=K.distanceTo(st),At=at.projectionMatrix.elements,Dt=gt.projectionMatrix.elements,Ft=At[14]/(At[10]-1),Zt=At[14]/(At[10]+1),D=(At[9]+1)/At[5],Yt=(At[9]-1)/At[5],Ot=(At[8]-1)/At[0],Gt=(Dt[8]+1)/Dt[0],Mt=Ft*Ot,Jt=Ft*Gt,Lt=pt/(-Ot+Gt),It=Lt*-Ot;at.matrixWorld.decompose(Z.position,Z.quaternion,Z.scale),Z.translateX(It),Z.translateZ(Lt),Z.matrixWorld.compose(Z.position,Z.quaternion,Z.scale),Z.matrixWorldInverse.copy(Z.matrixWorld).invert();const L=Ft+Lt,S=Zt+Lt,V=Mt-It,it=Jt+(pt-It),rt=D*Zt/S*L,Q=Yt*Zt/S*L;Z.projectionMatrix.makePerspective(V,it,rt,Q,L,S),Z.projectionMatrixInverse.copy(Z.projectionMatrix).invert()}function dt(Z,at){at===null?Z.matrixWorld.copy(Z.matrix):Z.matrixWorld.multiplyMatrices(at.matrixWorld,Z.matrix),Z.matrixWorldInverse.copy(Z.matrixWorld).invert()}this.updateCamera=function(Z){if(r===null)return;M.texture!==null&&(Z.near=M.depthNear,Z.far=M.depthFar),E.near=F.near=I.near=Z.near,E.far=F.far=I.far=Z.far,(N!==E.near||$!==E.far)&&(r.updateRenderState({depthNear:E.near,depthFar:E.far}),N=E.near,$=E.far,I.near=N,I.far=$,F.near=N,F.far=$,I.updateProjectionMatrix(),F.updateProjectionMatrix(),Z.updateProjectionMatrix());const at=Z.parent,gt=E.cameras;dt(E,at);for(let pt=0;pt<gt.length;pt++)dt(gt[pt],at);gt.length===2?j(E,I,F):E.projectionMatrix.copy(I.projectionMatrix),_t(Z,E,at)};function _t(Z,at,gt){gt===null?Z.matrix.copy(at.matrixWorld):(Z.matrix.copy(gt.matrixWorld),Z.matrix.invert(),Z.matrix.multiply(at.matrixWorld)),Z.matrix.decompose(Z.position,Z.quaternion,Z.scale),Z.updateMatrixWorld(!0),Z.projectionMatrix.copy(at.projectionMatrix),Z.projectionMatrixInverse.copy(at.projectionMatrixInverse),Z.isPerspectiveCamera&&(Z.fov=va*2*Math.atan(1/Z.projectionMatrix.elements[5]),Z.zoom=1)}this.getCamera=function(){return E},this.getFoveation=function(){if(!(m===null&&g===null))return c},this.setFoveation=function(Z){c=Z,m!==null&&(m.fixedFoveation=Z),g!==null&&g.fixedFoveation!==void 0&&(g.fixedFoveation=Z)},this.hasDepthSensing=function(){return M.texture!==null},this.getDepthSensingMesh=function(){return M.getMesh(E)};let ut=null;function wt(Z,at){if(f=at.getViewerPose(u||o),v=at,f!==null){const gt=f.views;g!==null&&(t.setRenderTargetFramebuffer(C,g.framebuffer),t.setRenderTarget(C));let pt=!1;gt.length!==E.cameras.length&&(E.cameras.length=0,pt=!0);for(let Dt=0;Dt<gt.length;Dt++){const Ft=gt[Dt];let Zt=null;if(g!==null)Zt=g.getViewport(Ft);else{const Yt=p.getViewSubImage(m,Ft);Zt=Yt.viewport,Dt===0&&(t.setRenderTargetTextures(C,Yt.colorTexture,m.ignoreDepthValues?void 0:Yt.depthStencilTexture),t.setRenderTarget(C))}let D=A[Dt];D===void 0&&(D=new Ie,D.layers.enable(Dt),D.viewport=new se,A[Dt]=D),D.matrix.fromArray(Ft.transform.matrix),D.matrix.decompose(D.position,D.quaternion,D.scale),D.projectionMatrix.fromArray(Ft.projectionMatrix),D.projectionMatrixInverse.copy(D.projectionMatrix).invert(),D.viewport.set(Zt.x,Zt.y,Zt.width,Zt.height),Dt===0&&(E.matrix.copy(D.matrix),E.matrix.decompose(E.position,E.quaternion,E.scale)),pt===!0&&E.cameras.push(D)}const At=r.enabledFeatures;if(At&&At.includes("depth-sensing")){const Dt=p.getDepthInformation(gt[0]);Dt&&Dt.isValid&&Dt.texture&&M.init(t,Dt,r.renderState)}}for(let gt=0;gt<w.length;gt++){const pt=b[gt],At=w[gt];pt!==null&&At!==void 0&&At.update(pt,at,u||o)}ut&&ut(Z,at),at.detectedPlanes&&i.dispatchEvent({type:"planesdetected",data:at}),v=null}const kt=new tl;kt.setAnimationLoop(wt),this.setAnimationLoop=function(Z){ut=Z},this.dispose=function(){}}}const Fn=new Ge,t0=new Qt;function e0(n,t){function e(_,d){_.matrixAutoUpdate===!0&&_.updateMatrix(),d.value.copy(_.matrix)}function i(_,d){d.color.getRGB(_.fogColor.value,Zc(n)),d.isFog?(_.fogNear.value=d.near,_.fogFar.value=d.far):d.isFogExp2&&(_.fogDensity.value=d.density)}function r(_,d,C,w,b){d.isMeshBasicMaterial||d.isMeshLambertMaterial?s(_,d):d.isMeshToonMaterial?(s(_,d),p(_,d)):d.isMeshPhongMaterial?(s(_,d),f(_,d)):d.isMeshStandardMaterial?(s(_,d),m(_,d),d.isMeshPhysicalMaterial&&g(_,d,b)):d.isMeshMatcapMaterial?(s(_,d),v(_,d)):d.isMeshDepthMaterial?s(_,d):d.isMeshDistanceMaterial?(s(_,d),M(_,d)):d.isMeshNormalMaterial?s(_,d):d.isLineBasicMaterial?(o(_,d),d.isLineDashedMaterial&&a(_,d)):d.isPointsMaterial?c(_,d,C,w):d.isSpriteMaterial?u(_,d):d.isShadowMaterial?(_.color.value.copy(d.color),_.opacity.value=d.opacity):d.isShaderMaterial&&(d.uniformsNeedUpdate=!1)}function s(_,d){_.opacity.value=d.opacity,d.color&&_.diffuse.value.copy(d.color),d.emissive&&_.emissive.value.copy(d.emissive).multiplyScalar(d.emissiveIntensity),d.map&&(_.map.value=d.map,e(d.map,_.mapTransform)),d.alphaMap&&(_.alphaMap.value=d.alphaMap,e(d.alphaMap,_.alphaMapTransform)),d.bumpMap&&(_.bumpMap.value=d.bumpMap,e(d.bumpMap,_.bumpMapTransform),_.bumpScale.value=d.bumpScale,d.side===Se&&(_.bumpScale.value*=-1)),d.normalMap&&(_.normalMap.value=d.normalMap,e(d.normalMap,_.normalMapTransform),_.normalScale.value.copy(d.normalScale),d.side===Se&&_.normalScale.value.negate()),d.displacementMap&&(_.displacementMap.value=d.displacementMap,e(d.displacementMap,_.displacementMapTransform),_.displacementScale.value=d.displacementScale,_.displacementBias.value=d.displacementBias),d.emissiveMap&&(_.emissiveMap.value=d.emissiveMap,e(d.emissiveMap,_.emissiveMapTransform)),d.specularMap&&(_.specularMap.value=d.specularMap,e(d.specularMap,_.specularMapTransform)),d.alphaTest>0&&(_.alphaTest.value=d.alphaTest);const C=t.get(d),w=C.envMap,b=C.envMapRotation;w&&(_.envMap.value=w,Fn.copy(b),Fn.x*=-1,Fn.y*=-1,Fn.z*=-1,w.isCubeTexture&&w.isRenderTargetTexture===!1&&(Fn.y*=-1,Fn.z*=-1),_.envMapRotation.value.setFromMatrix4(t0.makeRotationFromEuler(Fn)),_.flipEnvMap.value=w.isCubeTexture&&w.isRenderTargetTexture===!1?-1:1,_.reflectivity.value=d.reflectivity,_.ior.value=d.ior,_.refractionRatio.value=d.refractionRatio),d.lightMap&&(_.lightMap.value=d.lightMap,_.lightMapIntensity.value=d.lightMapIntensity,e(d.lightMap,_.lightMapTransform)),d.aoMap&&(_.aoMap.value=d.aoMap,_.aoMapIntensity.value=d.aoMapIntensity,e(d.aoMap,_.aoMapTransform))}function o(_,d){_.diffuse.value.copy(d.color),_.opacity.value=d.opacity,d.map&&(_.map.value=d.map,e(d.map,_.mapTransform))}function a(_,d){_.dashSize.value=d.dashSize,_.totalSize.value=d.dashSize+d.gapSize,_.scale.value=d.scale}function c(_,d,C,w){_.diffuse.value.copy(d.color),_.opacity.value=d.opacity,_.size.value=d.size*C,_.scale.value=w*.5,d.map&&(_.map.value=d.map,e(d.map,_.uvTransform)),d.alphaMap&&(_.alphaMap.value=d.alphaMap,e(d.alphaMap,_.alphaMapTransform)),d.alphaTest>0&&(_.alphaTest.value=d.alphaTest)}function u(_,d){_.diffuse.value.copy(d.color),_.opacity.value=d.opacity,_.rotation.value=d.rotation,d.map&&(_.map.value=d.map,e(d.map,_.mapTransform)),d.alphaMap&&(_.alphaMap.value=d.alphaMap,e(d.alphaMap,_.alphaMapTransform)),d.alphaTest>0&&(_.alphaTest.value=d.alphaTest)}function f(_,d){_.specular.value.copy(d.specular),_.shininess.value=Math.max(d.shininess,1e-4)}function p(_,d){d.gradientMap&&(_.gradientMap.value=d.gradientMap)}function m(_,d){_.metalness.value=d.metalness,d.metalnessMap&&(_.metalnessMap.value=d.metalnessMap,e(d.metalnessMap,_.metalnessMapTransform)),_.roughness.value=d.roughness,d.roughnessMap&&(_.roughnessMap.value=d.roughnessMap,e(d.roughnessMap,_.roughnessMapTransform)),d.envMap&&(_.envMapIntensity.value=d.envMapIntensity)}function g(_,d,C){_.ior.value=d.ior,d.sheen>0&&(_.sheenColor.value.copy(d.sheenColor).multiplyScalar(d.sheen),_.sheenRoughness.value=d.sheenRoughness,d.sheenColorMap&&(_.sheenColorMap.value=d.sheenColorMap,e(d.sheenColorMap,_.sheenColorMapTransform)),d.sheenRoughnessMap&&(_.sheenRoughnessMap.value=d.sheenRoughnessMap,e(d.sheenRoughnessMap,_.sheenRoughnessMapTransform))),d.clearcoat>0&&(_.clearcoat.value=d.clearcoat,_.clearcoatRoughness.value=d.clearcoatRoughness,d.clearcoatMap&&(_.clearcoatMap.value=d.clearcoatMap,e(d.clearcoatMap,_.clearcoatMapTransform)),d.clearcoatRoughnessMap&&(_.clearcoatRoughnessMap.value=d.clearcoatRoughnessMap,e(d.clearcoatRoughnessMap,_.clearcoatRoughnessMapTransform)),d.clearcoatNormalMap&&(_.clearcoatNormalMap.value=d.clearcoatNormalMap,e(d.clearcoatNormalMap,_.clearcoatNormalMapTransform),_.clearcoatNormalScale.value.copy(d.clearcoatNormalScale),d.side===Se&&_.clearcoatNormalScale.value.negate())),d.dispersion>0&&(_.dispersion.value=d.dispersion),d.iridescence>0&&(_.iridescence.value=d.iridescence,_.iridescenceIOR.value=d.iridescenceIOR,_.iridescenceThicknessMinimum.value=d.iridescenceThicknessRange[0],_.iridescenceThicknessMaximum.value=d.iridescenceThicknessRange[1],d.iridescenceMap&&(_.iridescenceMap.value=d.iridescenceMap,e(d.iridescenceMap,_.iridescenceMapTransform)),d.iridescenceThicknessMap&&(_.iridescenceThicknessMap.value=d.iridescenceThicknessMap,e(d.iridescenceThicknessMap,_.iridescenceThicknessMapTransform))),d.transmission>0&&(_.transmission.value=d.transmission,_.transmissionSamplerMap.value=C.texture,_.transmissionSamplerSize.value.set(C.width,C.height),d.transmissionMap&&(_.transmissionMap.value=d.transmissionMap,e(d.transmissionMap,_.transmissionMapTransform)),_.thickness.value=d.thickness,d.thicknessMap&&(_.thicknessMap.value=d.thicknessMap,e(d.thicknessMap,_.thicknessMapTransform)),_.attenuationDistance.value=d.attenuationDistance,_.attenuationColor.value.copy(d.attenuationColor)),d.anisotropy>0&&(_.anisotropyVector.value.set(d.anisotropy*Math.cos(d.anisotropyRotation),d.anisotropy*Math.sin(d.anisotropyRotation)),d.anisotropyMap&&(_.anisotropyMap.value=d.anisotropyMap,e(d.anisotropyMap,_.anisotropyMapTransform))),_.specularIntensity.value=d.specularIntensity,_.specularColor.value.copy(d.specularColor),d.specularColorMap&&(_.specularColorMap.value=d.specularColorMap,e(d.specularColorMap,_.specularColorMapTransform)),d.specularIntensityMap&&(_.specularIntensityMap.value=d.specularIntensityMap,e(d.specularIntensityMap,_.specularIntensityMapTransform))}function v(_,d){d.matcap&&(_.matcap.value=d.matcap)}function M(_,d){const C=t.get(d).light;_.referencePosition.value.setFromMatrixPosition(C.matrixWorld),_.nearDistance.value=C.shadow.camera.near,_.farDistance.value=C.shadow.camera.far}return{refreshFogUniforms:i,refreshMaterialUniforms:r}}function n0(n,t,e,i){let r={},s={},o=[];const a=n.getParameter(n.MAX_UNIFORM_BUFFER_BINDINGS);function c(C,w){const b=w.program;i.uniformBlockBinding(C,b)}function u(C,w){let b=r[C.id];b===void 0&&(v(C),b=f(C),r[C.id]=b,C.addEventListener("dispose",_));const z=w.program;i.updateUBOMapping(C,z);const P=t.render.frame;s[C.id]!==P&&(m(C),s[C.id]=P)}function f(C){const w=p();C.__bindingPointIndex=w;const b=n.createBuffer(),z=C.__size,P=C.usage;return n.bindBuffer(n.UNIFORM_BUFFER,b),n.bufferData(n.UNIFORM_BUFFER,z,P),n.bindBuffer(n.UNIFORM_BUFFER,null),n.bindBufferBase(n.UNIFORM_BUFFER,w,b),b}function p(){for(let C=0;C<a;C++)if(o.indexOf(C)===-1)return o.push(C),C;return console.error("THREE.WebGLRenderer: Maximum number of simultaneously usable uniforms groups reached."),0}function m(C){const w=r[C.id],b=C.uniforms,z=C.__cache;n.bindBuffer(n.UNIFORM_BUFFER,w);for(let P=0,I=b.length;P<I;P++){const F=Array.isArray(b[P])?b[P]:[b[P]];for(let A=0,E=F.length;A<E;A++){const N=F[A];if(g(N,P,A,z)===!0){const $=N.__offset,W=Array.isArray(N.value)?N.value:[N.value];let Y=0;for(let tt=0;tt<W.length;tt++){const K=W[tt],st=M(K);typeof K=="number"||typeof K=="boolean"?(N.__data[0]=K,n.bufferSubData(n.UNIFORM_BUFFER,$+Y,N.__data)):K.isMatrix3?(N.__data[0]=K.elements[0],N.__data[1]=K.elements[1],N.__data[2]=K.elements[2],N.__data[3]=0,N.__data[4]=K.elements[3],N.__data[5]=K.elements[4],N.__data[6]=K.elements[5],N.__data[7]=0,N.__data[8]=K.elements[6],N.__data[9]=K.elements[7],N.__data[10]=K.elements[8],N.__data[11]=0):(K.toArray(N.__data,Y),Y+=st.storage/Float32Array.BYTES_PER_ELEMENT)}n.bufferSubData(n.UNIFORM_BUFFER,$,N.__data)}}}n.bindBuffer(n.UNIFORM_BUFFER,null)}function g(C,w,b,z){const P=C.value,I=w+"_"+b;if(z[I]===void 0)return typeof P=="number"||typeof P=="boolean"?z[I]=P:z[I]=P.clone(),!0;{const F=z[I];if(typeof P=="number"||typeof P=="boolean"){if(F!==P)return z[I]=P,!0}else if(F.equals(P)===!1)return F.copy(P),!0}return!1}function v(C){const w=C.uniforms;let b=0;const z=16;for(let I=0,F=w.length;I<F;I++){const A=Array.isArray(w[I])?w[I]:[w[I]];for(let E=0,N=A.length;E<N;E++){const $=A[E],W=Array.isArray($.value)?$.value:[$.value];for(let Y=0,tt=W.length;Y<tt;Y++){const K=W[Y],st=M(K),j=b%z,dt=j%st.boundary,_t=j+dt;b+=dt,_t!==0&&z-_t<st.storage&&(b+=z-_t),$.__data=new Float32Array(st.storage/Float32Array.BYTES_PER_ELEMENT),$.__offset=b,b+=st.storage}}}const P=b%z;return P>0&&(b+=z-P),C.__size=b,C.__cache={},this}function M(C){const w={boundary:0,storage:0};return typeof C=="number"||typeof C=="boolean"?(w.boundary=4,w.storage=4):C.isVector2?(w.boundary=8,w.storage=8):C.isVector3||C.isColor?(w.boundary=16,w.storage=12):C.isVector4?(w.boundary=16,w.storage=16):C.isMatrix3?(w.boundary=48,w.storage=48):C.isMatrix4?(w.boundary=64,w.storage=64):C.isTexture?console.warn("THREE.WebGLRenderer: Texture samplers can not be part of an uniforms group."):console.warn("THREE.WebGLRenderer: Unsupported uniform value type.",C),w}function _(C){const w=C.target;w.removeEventListener("dispose",_);const b=o.indexOf(w.__bindingPointIndex);o.splice(b,1),n.deleteBuffer(r[w.id]),delete r[w.id],delete s[w.id]}function d(){for(const C in r)n.deleteBuffer(r[C]);o=[],r={},s={}}return{bind:c,update:u,dispose:d}}class i0{constructor(t={}){const{canvas:e=Xu(),context:i=null,depth:r=!0,stencil:s=!1,alpha:o=!1,antialias:a=!1,premultipliedAlpha:c=!0,preserveDrawingBuffer:u=!1,powerPreference:f="default",failIfMajorPerformanceCaveat:p=!1}=t;this.isWebGLRenderer=!0;let m;if(i!==null){if(typeof WebGLRenderingContext<"u"&&i instanceof WebGLRenderingContext)throw new Error("THREE.WebGLRenderer: WebGL 1 is not supported since r163.");m=i.getContextAttributes().alpha}else m=o;const g=new Uint32Array(4),v=new Int32Array(4);let M=null,_=null;const d=[],C=[];this.domElement=e,this.debug={checkShaderErrors:!0,onShaderError:null},this.autoClear=!0,this.autoClearColor=!0,this.autoClearDepth=!0,this.autoClearStencil=!0,this.sortObjects=!0,this.clippingPlanes=[],this.localClippingEnabled=!1,this._outputColorSpace=We,this.toneMapping=wn,this.toneMappingExposure=1;const w=this;let b=!1,z=0,P=0,I=null,F=-1,A=null;const E=new se,N=new se;let $=null;const W=new Ht(0);let Y=0,tt=e.width,K=e.height,st=1,j=null,dt=null;const _t=new se(0,0,tt,K),ut=new se(0,0,tt,K);let wt=!1;const kt=new Fa;let Z=!1,at=!1;const gt=new Qt,pt=new k,At=new se,Dt={background:null,fog:null,environment:null,overrideMaterial:null,isScene:!0};let Ft=!1;function Zt(){return I===null?st:1}let D=i;function Yt(T,B){return e.getContext(T,B)}try{const T={alpha:!0,depth:r,stencil:s,antialias:a,premultipliedAlpha:c,preserveDrawingBuffer:u,powerPreference:f,failIfMajorPerformanceCaveat:p};if("setAttribute"in e&&e.setAttribute("data-engine",`three.js r${ba}`),e.addEventListener("webglcontextlost",J,!1),e.addEventListener("webglcontextrestored",et,!1),e.addEventListener("webglcontextcreationerror",lt,!1),D===null){const B="webgl2";if(D=Yt(B,T),D===null)throw Yt(B)?new Error("Error creating WebGL context with your selected attributes."):new Error("Error creating WebGL context.")}}catch(T){throw console.error("THREE.WebGLRenderer: "+T.message),T}let Ot,Gt,Mt,Jt,Lt,It,L,S,V,it,rt,Q,Et,x,l,h,y,R,O,G,nt,mt,xt,Rt;function U(){Ot=new lp(D),Ot.init(),mt=new $m(D,Ot),Gt=new ip(D,Ot,t,mt),Mt=new Wm(D),Jt=new fp(D),Lt=new Pm,It=new Xm(D,Ot,Mt,Lt,Gt,mt,Jt),L=new sp(w),S=new cp(w),V=new xh(D),xt=new ep(D,V),it=new up(D,V,Jt,xt),rt=new pp(D,it,V,Jt),O=new dp(D,Gt,It),h=new rp(Lt),Q=new Rm(w,L,S,Ot,Gt,xt,h),Et=new e0(w,Lt),x=new Im,l=new Bm(Ot),R=new tp(w,L,S,Mt,rt,m,c),y=new Vm(w,rt,Gt),Rt=new n0(D,Jt,Gt,Mt),G=new np(D,Ot,Jt),nt=new hp(D,Ot,Jt),Jt.programs=Q.programs,w.capabilities=Gt,w.extensions=Ot,w.properties=Lt,w.renderLists=x,w.shadowMap=y,w.state=Mt,w.info=Jt}U();const ot=new Qm(w,D);this.xr=ot,this.getContext=function(){return D},this.getContextAttributes=function(){return D.getContextAttributes()},this.forceContextLoss=function(){const T=Ot.get("WEBGL_lose_context");T&&T.loseContext()},this.forceContextRestore=function(){const T=Ot.get("WEBGL_lose_context");T&&T.restoreContext()},this.getPixelRatio=function(){return st},this.setPixelRatio=function(T){T!==void 0&&(st=T,this.setSize(tt,K,!1))},this.getSize=function(T){return T.set(tt,K)},this.setSize=function(T,B,q=!0){if(ot.isPresenting){console.warn("THREE.WebGLRenderer: Can't change size while VR device is presenting.");return}tt=T,K=B,e.width=Math.floor(T*st),e.height=Math.floor(B*st),q===!0&&(e.style.width=T+"px",e.style.height=B+"px"),this.setViewport(0,0,T,B)},this.getDrawingBufferSize=function(T){return T.set(tt*st,K*st).floor()},this.setDrawingBufferSize=function(T,B,q){tt=T,K=B,st=q,e.width=Math.floor(T*q),e.height=Math.floor(B*q),this.setViewport(0,0,T,B)},this.getCurrentViewport=function(T){return T.copy(E)},this.getViewport=function(T){return T.copy(_t)},this.setViewport=function(T,B,q,X){T.isVector4?_t.set(T.x,T.y,T.z,T.w):_t.set(T,B,q,X),Mt.viewport(E.copy(_t).multiplyScalar(st).round())},this.getScissor=function(T){return T.copy(ut)},this.setScissor=function(T,B,q,X){T.isVector4?ut.set(T.x,T.y,T.z,T.w):ut.set(T,B,q,X),Mt.scissor(N.copy(ut).multiplyScalar(st).round())},this.getScissorTest=function(){return wt},this.setScissorTest=function(T){Mt.setScissorTest(wt=T)},this.setOpaqueSort=function(T){j=T},this.setTransparentSort=function(T){dt=T},this.getClearColor=function(T){return T.copy(R.getClearColor())},this.setClearColor=function(){R.setClearColor.apply(R,arguments)},this.getClearAlpha=function(){return R.getClearAlpha()},this.setClearAlpha=function(){R.setClearAlpha.apply(R,arguments)},this.clear=function(T=!0,B=!0,q=!0){let X=0;if(T){let H=!1;if(I!==null){const ct=I.texture.format;H=ct===Da||ct===Ia||ct===La}if(H){const ct=I.texture.type,ft=ct===ln||ct===Wn||ct===Gi||ct===Si||ct===Ra||ct===Pa,vt=R.getClearColor(),yt=R.getClearAlpha(),Ct=vt.r,Pt=vt.g,Tt=vt.b;ft?(g[0]=Ct,g[1]=Pt,g[2]=Tt,g[3]=yt,D.clearBufferuiv(D.COLOR,0,g)):(v[0]=Ct,v[1]=Pt,v[2]=Tt,v[3]=yt,D.clearBufferiv(D.COLOR,0,v))}else X|=D.COLOR_BUFFER_BIT}B&&(X|=D.DEPTH_BUFFER_BIT),q&&(X|=D.STENCIL_BUFFER_BIT,this.state.buffers.stencil.setMask(4294967295)),D.clear(X)},this.clearColor=function(){this.clear(!0,!1,!1)},this.clearDepth=function(){this.clear(!1,!0,!1)},this.clearStencil=function(){this.clear(!1,!1,!0)},this.dispose=function(){e.removeEventListener("webglcontextlost",J,!1),e.removeEventListener("webglcontextrestored",et,!1),e.removeEventListener("webglcontextcreationerror",lt,!1),x.dispose(),l.dispose(),Lt.dispose(),L.dispose(),S.dispose(),rt.dispose(),xt.dispose(),Rt.dispose(),Q.dispose(),ot.dispose(),ot.removeEventListener("sessionstart",Ve),ot.removeEventListener("sessionend",Wa),Pn.stop()};function J(T){T.preventDefault(),console.log("THREE.WebGLRenderer: Context Lost."),b=!0}function et(){console.log("THREE.WebGLRenderer: Context Restored."),b=!1;const T=Jt.autoReset,B=y.enabled,q=y.autoUpdate,X=y.needsUpdate,H=y.type;U(),Jt.autoReset=T,y.enabled=B,y.autoUpdate=q,y.needsUpdate=X,y.type=H}function lt(T){console.error("THREE.WebGLRenderer: A WebGL context could not be created. Reason: ",T.statusMessage)}function bt(T){const B=T.target;B.removeEventListener("dispose",bt),zt(B)}function zt(T){ne(T),Lt.remove(T)}function ne(T){const B=Lt.get(T).programs;B!==void 0&&(B.forEach(function(q){Q.releaseProgram(q)}),T.isShaderMaterial&&Q.releaseShaderCache(T))}this.renderBufferDirect=function(T,B,q,X,H,ct){B===null&&(B=Dt);const ft=H.isMesh&&H.matrixWorld.determinant()<0,vt=Bl(T,B,q,X,H);Mt.setMaterial(X,ft);let yt=q.index,Ct=1;if(X.wireframe===!0){if(yt=it.getWireframeAttribute(q),yt===void 0)return;Ct=2}const Pt=q.drawRange,Tt=q.attributes.position;let Wt=Pt.start*Ct,te=(Pt.start+Pt.count)*Ct;ct!==null&&(Wt=Math.max(Wt,ct.start*Ct),te=Math.min(te,(ct.start+ct.count)*Ct)),yt!==null?(Wt=Math.max(Wt,0),te=Math.min(te,yt.count)):Tt!=null&&(Wt=Math.max(Wt,0),te=Math.min(te,Tt.count));const ee=te-Wt;if(ee<0||ee===1/0)return;xt.setup(H,X,vt,q,yt);let Te,qt=G;if(yt!==null&&(Te=V.get(yt),qt=nt,qt.setIndex(Te)),H.isMesh)X.wireframe===!0?(Mt.setLineWidth(X.wireframeLinewidth*Zt()),qt.setMode(D.LINES)):qt.setMode(D.TRIANGLES);else if(H.isLine){let St=X.linewidth;St===void 0&&(St=1),Mt.setLineWidth(St*Zt()),H.isLineSegments?qt.setMode(D.LINES):H.isLineLoop?qt.setMode(D.LINE_LOOP):qt.setMode(D.LINE_STRIP)}else H.isPoints?qt.setMode(D.POINTS):H.isSprite&&qt.setMode(D.TRIANGLES);if(H.isBatchedMesh)if(H._multiDrawInstances!==null)qt.renderMultiDrawInstances(H._multiDrawStarts,H._multiDrawCounts,H._multiDrawCount,H._multiDrawInstances);else if(Ot.get("WEBGL_multi_draw"))qt.renderMultiDraw(H._multiDrawStarts,H._multiDrawCounts,H._multiDrawCount);else{const St=H._multiDrawStarts,he=H._multiDrawCounts,Xt=H._multiDrawCount,Ue=yt?V.get(yt).bytesPerElement:1,Zn=Lt.get(X).currentProgram.getUniforms();for(let we=0;we<Xt;we++)Zn.setValue(D,"_gl_DrawID",we),qt.render(St[we]/Ue,he[we])}else if(H.isInstancedMesh)qt.renderInstances(Wt,ee,H.count);else if(q.isInstancedBufferGeometry){const St=q._maxInstanceCount!==void 0?q._maxInstanceCount:1/0,he=Math.min(q.instanceCount,St);qt.renderInstances(Wt,ee,he)}else qt.render(Wt,ee)};function ue(T,B,q){T.transparent===!0&&T.side===ge&&T.forceSinglePass===!1?(T.side=Se,T.needsUpdate=!0,Zi(T,B,q),T.side=An,T.needsUpdate=!0,Zi(T,B,q),T.side=ge):Zi(T,B,q)}this.compile=function(T,B,q=null){q===null&&(q=T),_=l.get(q),_.init(B),C.push(_),q.traverseVisible(function(H){H.isLight&&H.layers.test(B.layers)&&(_.pushLight(H),H.castShadow&&_.pushShadow(H))}),T!==q&&T.traverseVisible(function(H){H.isLight&&H.layers.test(B.layers)&&(_.pushLight(H),H.castShadow&&_.pushShadow(H))}),_.setupLights();const X=new Set;return T.traverse(function(H){const ct=H.material;if(ct)if(Array.isArray(ct))for(let ft=0;ft<ct.length;ft++){const vt=ct[ft];ue(vt,q,H),X.add(vt)}else ue(ct,q,H),X.add(ct)}),C.pop(),_=null,X},this.compileAsync=function(T,B,q=null){const X=this.compile(T,B,q);return new Promise(H=>{function ct(){if(X.forEach(function(ft){Lt.get(ft).currentProgram.isReady()&&X.delete(ft)}),X.size===0){H(T);return}setTimeout(ct,10)}Ot.get("KHR_parallel_shader_compile")!==null?ct():setTimeout(ct,10)})};let Vt=null;function Ke(T){Vt&&Vt(T)}function Ve(){Pn.stop()}function Wa(){Pn.start()}const Pn=new tl;Pn.setAnimationLoop(Ke),typeof self<"u"&&Pn.setContext(self),this.setAnimationLoop=function(T){Vt=T,ot.setAnimationLoop(T),T===null?Pn.stop():Pn.start()},ot.addEventListener("sessionstart",Ve),ot.addEventListener("sessionend",Wa),this.render=function(T,B){if(B!==void 0&&B.isCamera!==!0){console.error("THREE.WebGLRenderer.render: camera is not an instance of THREE.Camera.");return}if(b===!0)return;if(T.matrixWorldAutoUpdate===!0&&T.updateMatrixWorld(),B.parent===null&&B.matrixWorldAutoUpdate===!0&&B.updateMatrixWorld(),ot.enabled===!0&&ot.isPresenting===!0&&(ot.cameraAutoUpdate===!0&&ot.updateCamera(B),B=ot.getCamera()),T.isScene===!0&&T.onBeforeRender(w,T,B,I),_=l.get(T,C.length),_.init(B),C.push(_),gt.multiplyMatrices(B.projectionMatrix,B.matrixWorldInverse),kt.setFromProjectionMatrix(gt),at=this.localClippingEnabled,Z=h.init(this.clippingPlanes,at),M=x.get(T,d.length),M.init(),d.push(M),ot.enabled===!0&&ot.isPresenting===!0){const ct=w.xr.getDepthSensingMesh();ct!==null&&es(ct,B,-1/0,w.sortObjects)}es(T,B,0,w.sortObjects),M.finish(),w.sortObjects===!0&&M.sort(j,dt),Ft=ot.enabled===!1||ot.isPresenting===!1||ot.hasDepthSensing()===!1,Ft&&R.addToRenderList(M,T),this.info.render.frame++,Z===!0&&h.beginShadows();const q=_.state.shadowsArray;y.render(q,T,B),Z===!0&&h.endShadows(),this.info.autoReset===!0&&this.info.reset();const X=M.opaque,H=M.transmissive;if(_.setupLights(),B.isArrayCamera){const ct=B.cameras;if(H.length>0)for(let ft=0,vt=ct.length;ft<vt;ft++){const yt=ct[ft];Xa(X,H,T,yt)}Ft&&R.render(T);for(let ft=0,vt=ct.length;ft<vt;ft++){const yt=ct[ft];qa(M,T,yt,yt.viewport)}}else H.length>0&&Xa(X,H,T,B),Ft&&R.render(T),qa(M,T,B);I!==null&&(It.updateMultisampleRenderTarget(I),It.updateRenderTargetMipmap(I)),T.isScene===!0&&T.onAfterRender(w,T,B),xt.resetDefaultState(),F=-1,A=null,C.pop(),C.length>0?(_=C[C.length-1],Z===!0&&h.setGlobalState(w.clippingPlanes,_.state.camera)):_=null,d.pop(),d.length>0?M=d[d.length-1]:M=null};function es(T,B,q,X){if(T.visible===!1)return;if(T.layers.test(B.layers)){if(T.isGroup)q=T.renderOrder;else if(T.isLOD)T.autoUpdate===!0&&T.update(B);else if(T.isLight)_.pushLight(T),T.castShadow&&_.pushShadow(T);else if(T.isSprite){if(!T.frustumCulled||kt.intersectsSprite(T)){X&&At.setFromMatrixPosition(T.matrixWorld).applyMatrix4(gt);const ft=rt.update(T),vt=T.material;vt.visible&&M.push(T,ft,vt,q,At.z,null)}}else if((T.isMesh||T.isLine||T.isPoints)&&(!T.frustumCulled||kt.intersectsObject(T))){const ft=rt.update(T),vt=T.material;if(X&&(T.boundingSphere!==void 0?(T.boundingSphere===null&&T.computeBoundingSphere(),At.copy(T.boundingSphere.center)):(ft.boundingSphere===null&&ft.computeBoundingSphere(),At.copy(ft.boundingSphere.center)),At.applyMatrix4(T.matrixWorld).applyMatrix4(gt)),Array.isArray(vt)){const yt=ft.groups;for(let Ct=0,Pt=yt.length;Ct<Pt;Ct++){const Tt=yt[Ct],Wt=vt[Tt.materialIndex];Wt&&Wt.visible&&M.push(T,ft,Wt,q,At.z,Tt)}}else vt.visible&&M.push(T,ft,vt,q,At.z,null)}}const ct=T.children;for(let ft=0,vt=ct.length;ft<vt;ft++)es(ct[ft],B,q,X)}function qa(T,B,q,X){const H=T.opaque,ct=T.transmissive,ft=T.transparent;_.setupLightsView(q),Z===!0&&h.setGlobalState(w.clippingPlanes,q),X&&Mt.viewport(E.copy(X)),H.length>0&&ji(H,B,q),ct.length>0&&ji(ct,B,q),ft.length>0&&ji(ft,B,q),Mt.buffers.depth.setTest(!0),Mt.buffers.depth.setMask(!0),Mt.buffers.color.setMask(!0),Mt.setPolygonOffset(!1)}function Xa(T,B,q,X){if((q.isScene===!0?q.overrideMaterial:null)!==null)return;_.state.transmissionRenderTarget[X.id]===void 0&&(_.state.transmissionRenderTarget[X.id]=new qn(1,1,{generateMipmaps:!0,type:Ot.has("EXT_color_buffer_half_float")||Ot.has("EXT_color_buffer_float")?qi:ln,minFilter:Vn,samples:4,stencilBuffer:s,resolveDepthBuffer:!1,resolveStencilBuffer:!1,colorSpace:$t.workingColorSpace}));const ct=_.state.transmissionRenderTarget[X.id],ft=X.viewport||E;ct.setSize(ft.z,ft.w);const vt=w.getRenderTarget();w.setRenderTarget(ct),w.getClearColor(W),Y=w.getClearAlpha(),Y<1&&w.setClearColor(16777215,.5),w.clear(),Ft&&R.render(q);const yt=w.toneMapping;w.toneMapping=wn;const Ct=X.viewport;if(X.viewport!==void 0&&(X.viewport=void 0),_.setupLightsView(X),Z===!0&&h.setGlobalState(w.clippingPlanes,X),ji(T,q,X),It.updateMultisampleRenderTarget(ct),It.updateRenderTargetMipmap(ct),Ot.has("WEBGL_multisampled_render_to_texture")===!1){let Pt=!1;for(let Tt=0,Wt=B.length;Tt<Wt;Tt++){const te=B[Tt],ee=te.object,Te=te.geometry,qt=te.material,St=te.group;if(qt.side===ge&&ee.layers.test(X.layers)){const he=qt.side;qt.side=Se,qt.needsUpdate=!0,$a(ee,q,X,Te,qt,St),qt.side=he,qt.needsUpdate=!0,Pt=!0}}Pt===!0&&(It.updateMultisampleRenderTarget(ct),It.updateRenderTargetMipmap(ct))}w.setRenderTarget(vt),w.setClearColor(W,Y),Ct!==void 0&&(X.viewport=Ct),w.toneMapping=yt}function ji(T,B,q){const X=B.isScene===!0?B.overrideMaterial:null;for(let H=0,ct=T.length;H<ct;H++){const ft=T[H],vt=ft.object,yt=ft.geometry,Ct=X===null?ft.material:X,Pt=ft.group;vt.layers.test(q.layers)&&$a(vt,B,q,yt,Ct,Pt)}}function $a(T,B,q,X,H,ct){T.onBeforeRender(w,B,q,X,H,ct),T.modelViewMatrix.multiplyMatrices(q.matrixWorldInverse,T.matrixWorld),T.normalMatrix.getNormalMatrix(T.modelViewMatrix),H.transparent===!0&&H.side===ge&&H.forceSinglePass===!1?(H.side=Se,H.needsUpdate=!0,w.renderBufferDirect(q,B,X,H,T,ct),H.side=An,H.needsUpdate=!0,w.renderBufferDirect(q,B,X,H,T,ct),H.side=ge):w.renderBufferDirect(q,B,X,H,T,ct),T.onAfterRender(w,B,q,X,H,ct)}function Zi(T,B,q){B.isScene!==!0&&(B=Dt);const X=Lt.get(T),H=_.state.lights,ct=_.state.shadowsArray,ft=H.state.version,vt=Q.getParameters(T,H.state,ct,B,q),yt=Q.getProgramCacheKey(vt);let Ct=X.programs;X.environment=T.isMeshStandardMaterial?B.environment:null,X.fog=B.fog,X.envMap=(T.isMeshStandardMaterial?S:L).get(T.envMap||X.environment),X.envMapRotation=X.environment!==null&&T.envMap===null?B.environmentRotation:T.envMapRotation,Ct===void 0&&(T.addEventListener("dispose",bt),Ct=new Map,X.programs=Ct);let Pt=Ct.get(yt);if(Pt!==void 0){if(X.currentProgram===Pt&&X.lightsStateVersion===ft)return Ka(T,vt),Pt}else vt.uniforms=Q.getUniforms(T),T.onBeforeCompile(vt,w),Pt=Q.acquireProgram(vt,yt),Ct.set(yt,Pt),X.uniforms=vt.uniforms;const Tt=X.uniforms;return(!T.isShaderMaterial&&!T.isRawShaderMaterial||T.clipping===!0)&&(Tt.clippingPlanes=h.uniform),Ka(T,vt),X.needsLights=Hl(T),X.lightsStateVersion=ft,X.needsLights&&(Tt.ambientLightColor.value=H.state.ambient,Tt.lightProbe.value=H.state.probe,Tt.directionalLights.value=H.state.directional,Tt.directionalLightShadows.value=H.state.directionalShadow,Tt.spotLights.value=H.state.spot,Tt.spotLightShadows.value=H.state.spotShadow,Tt.rectAreaLights.value=H.state.rectArea,Tt.ltc_1.value=H.state.rectAreaLTC1,Tt.ltc_2.value=H.state.rectAreaLTC2,Tt.pointLights.value=H.state.point,Tt.pointLightShadows.value=H.state.pointShadow,Tt.hemisphereLights.value=H.state.hemi,Tt.directionalShadowMap.value=H.state.directionalShadowMap,Tt.directionalShadowMatrix.value=H.state.directionalShadowMatrix,Tt.spotShadowMap.value=H.state.spotShadowMap,Tt.spotLightMatrix.value=H.state.spotLightMatrix,Tt.spotLightMap.value=H.state.spotLightMap,Tt.pointShadowMap.value=H.state.pointShadowMap,Tt.pointShadowMatrix.value=H.state.pointShadowMatrix),X.currentProgram=Pt,X.uniformsList=null,Pt}function Ya(T){if(T.uniformsList===null){const B=T.currentProgram.getUniforms();T.uniformsList=Dr.seqWithValue(B.seq,T.uniforms)}return T.uniformsList}function Ka(T,B){const q=Lt.get(T);q.outputColorSpace=B.outputColorSpace,q.batching=B.batching,q.batchingColor=B.batchingColor,q.instancing=B.instancing,q.instancingColor=B.instancingColor,q.instancingMorph=B.instancingMorph,q.skinning=B.skinning,q.morphTargets=B.morphTargets,q.morphNormals=B.morphNormals,q.morphColors=B.morphColors,q.morphTargetsCount=B.morphTargetsCount,q.numClippingPlanes=B.numClippingPlanes,q.numIntersection=B.numClipIntersection,q.vertexAlphas=B.vertexAlphas,q.vertexTangents=B.vertexTangents,q.toneMapping=B.toneMapping}function Bl(T,B,q,X,H){B.isScene!==!0&&(B=Dt),It.resetTextureUnits();const ct=B.fog,ft=X.isMeshStandardMaterial?B.environment:null,vt=I===null?w.outputColorSpace:I.isXRRenderTarget===!0?I.texture.colorSpace:Rn,yt=(X.isMeshStandardMaterial?S:L).get(X.envMap||ft),Ct=X.vertexColors===!0&&!!q.attributes.color&&q.attributes.color.itemSize===4,Pt=!!q.attributes.tangent&&(!!X.normalMap||X.anisotropy>0),Tt=!!q.morphAttributes.position,Wt=!!q.morphAttributes.normal,te=!!q.morphAttributes.color;let ee=wn;X.toneMapped&&(I===null||I.isXRRenderTarget===!0)&&(ee=w.toneMapping);const Te=q.morphAttributes.position||q.morphAttributes.normal||q.morphAttributes.color,qt=Te!==void 0?Te.length:0,St=Lt.get(X),he=_.state.lights;if(Z===!0&&(at===!0||T!==A)){const Re=T===A&&X.id===F;h.setState(X,T,Re)}let Xt=!1;X.version===St.__version?(St.needsLights&&St.lightsStateVersion!==he.state.version||St.outputColorSpace!==vt||H.isBatchedMesh&&St.batching===!1||!H.isBatchedMesh&&St.batching===!0||H.isBatchedMesh&&St.batchingColor===!0&&H.colorTexture===null||H.isBatchedMesh&&St.batchingColor===!1&&H.colorTexture!==null||H.isInstancedMesh&&St.instancing===!1||!H.isInstancedMesh&&St.instancing===!0||H.isSkinnedMesh&&St.skinning===!1||!H.isSkinnedMesh&&St.skinning===!0||H.isInstancedMesh&&St.instancingColor===!0&&H.instanceColor===null||H.isInstancedMesh&&St.instancingColor===!1&&H.instanceColor!==null||H.isInstancedMesh&&St.instancingMorph===!0&&H.morphTexture===null||H.isInstancedMesh&&St.instancingMorph===!1&&H.morphTexture!==null||St.envMap!==yt||X.fog===!0&&St.fog!==ct||St.numClippingPlanes!==void 0&&(St.numClippingPlanes!==h.numPlanes||St.numIntersection!==h.numIntersection)||St.vertexAlphas!==Ct||St.vertexTangents!==Pt||St.morphTargets!==Tt||St.morphNormals!==Wt||St.morphColors!==te||St.toneMapping!==ee||St.morphTargetsCount!==qt)&&(Xt=!0):(Xt=!0,St.__version=X.version);let Ue=St.currentProgram;Xt===!0&&(Ue=Zi(X,B,H));let Zn=!1,we=!1,ns=!1;const ie=Ue.getUniforms(),pn=St.uniforms;if(Mt.useProgram(Ue.program)&&(Zn=!0,we=!0,ns=!0),X.id!==F&&(F=X.id,we=!0),Zn||A!==T){ie.setValue(D,"projectionMatrix",T.projectionMatrix),ie.setValue(D,"viewMatrix",T.matrixWorldInverse);const Re=ie.map.cameraPosition;Re!==void 0&&Re.setValue(D,pt.setFromMatrixPosition(T.matrixWorld)),Gt.logarithmicDepthBuffer&&ie.setValue(D,"logDepthBufFC",2/(Math.log(T.far+1)/Math.LN2)),(X.isMeshPhongMaterial||X.isMeshToonMaterial||X.isMeshLambertMaterial||X.isMeshBasicMaterial||X.isMeshStandardMaterial||X.isShaderMaterial)&&ie.setValue(D,"isOrthographic",T.isOrthographicCamera===!0),A!==T&&(A=T,we=!0,ns=!0)}if(H.isSkinnedMesh){ie.setOptional(D,H,"bindMatrix"),ie.setOptional(D,H,"bindMatrixInverse");const Re=H.skeleton;Re&&(Re.boneTexture===null&&Re.computeBoneTexture(),ie.setValue(D,"boneTexture",Re.boneTexture,It))}H.isBatchedMesh&&(ie.setOptional(D,H,"batchingTexture"),ie.setValue(D,"batchingTexture",H._matricesTexture,It),ie.setOptional(D,H,"batchingIdTexture"),ie.setValue(D,"batchingIdTexture",H._indirectTexture,It),ie.setOptional(D,H,"batchingColorTexture"),H._colorsTexture!==null&&ie.setValue(D,"batchingColorTexture",H._colorsTexture,It));const is=q.morphAttributes;if((is.position!==void 0||is.normal!==void 0||is.color!==void 0)&&O.update(H,q,Ue),(we||St.receiveShadow!==H.receiveShadow)&&(St.receiveShadow=H.receiveShadow,ie.setValue(D,"receiveShadow",H.receiveShadow)),X.isMeshGouraudMaterial&&X.envMap!==null&&(pn.envMap.value=yt,pn.flipEnvMap.value=yt.isCubeTexture&&yt.isRenderTargetTexture===!1?-1:1),X.isMeshStandardMaterial&&X.envMap===null&&B.environment!==null&&(pn.envMapIntensity.value=B.environmentIntensity),we&&(ie.setValue(D,"toneMappingExposure",w.toneMappingExposure),St.needsLights&&zl(pn,ns),ct&&X.fog===!0&&Et.refreshFogUniforms(pn,ct),Et.refreshMaterialUniforms(pn,X,st,K,_.state.transmissionRenderTarget[T.id]),Dr.upload(D,Ya(St),pn,It)),X.isShaderMaterial&&X.uniformsNeedUpdate===!0&&(Dr.upload(D,Ya(St),pn,It),X.uniformsNeedUpdate=!1),X.isSpriteMaterial&&ie.setValue(D,"center",H.center),ie.setValue(D,"modelViewMatrix",H.modelViewMatrix),ie.setValue(D,"normalMatrix",H.normalMatrix),ie.setValue(D,"modelMatrix",H.matrixWorld),X.isShaderMaterial||X.isRawShaderMaterial){const Re=X.uniformsGroups;for(let rs=0,kl=Re.length;rs<kl;rs++){const ja=Re[rs];Rt.update(ja,Ue),Rt.bind(ja,Ue)}}return Ue}function zl(T,B){T.ambientLightColor.needsUpdate=B,T.lightProbe.needsUpdate=B,T.directionalLights.needsUpdate=B,T.directionalLightShadows.needsUpdate=B,T.pointLights.needsUpdate=B,T.pointLightShadows.needsUpdate=B,T.spotLights.needsUpdate=B,T.spotLightShadows.needsUpdate=B,T.rectAreaLights.needsUpdate=B,T.hemisphereLights.needsUpdate=B}function Hl(T){return T.isMeshLambertMaterial||T.isMeshToonMaterial||T.isMeshPhongMaterial||T.isMeshStandardMaterial||T.isShadowMaterial||T.isShaderMaterial&&T.lights===!0}this.getActiveCubeFace=function(){return z},this.getActiveMipmapLevel=function(){return P},this.getRenderTarget=function(){return I},this.setRenderTargetTextures=function(T,B,q){Lt.get(T.texture).__webglTexture=B,Lt.get(T.depthTexture).__webglTexture=q;const X=Lt.get(T);X.__hasExternalTextures=!0,X.__autoAllocateDepthBuffer=q===void 0,X.__autoAllocateDepthBuffer||Ot.has("WEBGL_multisampled_render_to_texture")===!0&&(console.warn("THREE.WebGLRenderer: Render-to-texture extension was disabled because an external texture was provided"),X.__useRenderToTexture=!1)},this.setRenderTargetFramebuffer=function(T,B){const q=Lt.get(T);q.__webglFramebuffer=B,q.__useDefaultFramebuffer=B===void 0},this.setRenderTarget=function(T,B=0,q=0){I=T,z=B,P=q;let X=!0,H=null,ct=!1,ft=!1;if(T){const yt=Lt.get(T);yt.__useDefaultFramebuffer!==void 0?(Mt.bindFramebuffer(D.FRAMEBUFFER,null),X=!1):yt.__webglFramebuffer===void 0?It.setupRenderTarget(T):yt.__hasExternalTextures&&It.rebindTextures(T,Lt.get(T.texture).__webglTexture,Lt.get(T.depthTexture).__webglTexture);const Ct=T.texture;(Ct.isData3DTexture||Ct.isDataArrayTexture||Ct.isCompressedArrayTexture)&&(ft=!0);const Pt=Lt.get(T).__webglFramebuffer;T.isWebGLCubeRenderTarget?(Array.isArray(Pt[B])?H=Pt[B][q]:H=Pt[B],ct=!0):T.samples>0&&It.useMultisampledRTT(T)===!1?H=Lt.get(T).__webglMultisampledFramebuffer:Array.isArray(Pt)?H=Pt[q]:H=Pt,E.copy(T.viewport),N.copy(T.scissor),$=T.scissorTest}else E.copy(_t).multiplyScalar(st).floor(),N.copy(ut).multiplyScalar(st).floor(),$=wt;if(Mt.bindFramebuffer(D.FRAMEBUFFER,H)&&X&&Mt.drawBuffers(T,H),Mt.viewport(E),Mt.scissor(N),Mt.setScissorTest($),ct){const yt=Lt.get(T.texture);D.framebufferTexture2D(D.FRAMEBUFFER,D.COLOR_ATTACHMENT0,D.TEXTURE_CUBE_MAP_POSITIVE_X+B,yt.__webglTexture,q)}else if(ft){const yt=Lt.get(T.texture),Ct=B||0;D.framebufferTextureLayer(D.FRAMEBUFFER,D.COLOR_ATTACHMENT0,yt.__webglTexture,q||0,Ct)}F=-1},this.readRenderTargetPixels=function(T,B,q,X,H,ct,ft){if(!(T&&T.isWebGLRenderTarget)){console.error("THREE.WebGLRenderer.readRenderTargetPixels: renderTarget is not THREE.WebGLRenderTarget.");return}let vt=Lt.get(T).__webglFramebuffer;if(T.isWebGLCubeRenderTarget&&ft!==void 0&&(vt=vt[ft]),vt){Mt.bindFramebuffer(D.FRAMEBUFFER,vt);try{const yt=T.texture,Ct=yt.format,Pt=yt.type;if(!Gt.textureFormatReadable(Ct)){console.error("THREE.WebGLRenderer.readRenderTargetPixels: renderTarget is not in RGBA or implementation defined format.");return}if(!Gt.textureTypeReadable(Pt)){console.error("THREE.WebGLRenderer.readRenderTargetPixels: renderTarget is not in UnsignedByteType or implementation defined type.");return}B>=0&&B<=T.width-X&&q>=0&&q<=T.height-H&&D.readPixels(B,q,X,H,mt.convert(Ct),mt.convert(Pt),ct)}finally{const yt=I!==null?Lt.get(I).__webglFramebuffer:null;Mt.bindFramebuffer(D.FRAMEBUFFER,yt)}}},this.readRenderTargetPixelsAsync=async function(T,B,q,X,H,ct,ft){if(!(T&&T.isWebGLRenderTarget))throw new Error("THREE.WebGLRenderer.readRenderTargetPixels: renderTarget is not THREE.WebGLRenderTarget.");let vt=Lt.get(T).__webglFramebuffer;if(T.isWebGLCubeRenderTarget&&ft!==void 0&&(vt=vt[ft]),vt){Mt.bindFramebuffer(D.FRAMEBUFFER,vt);try{const yt=T.texture,Ct=yt.format,Pt=yt.type;if(!Gt.textureFormatReadable(Ct))throw new Error("THREE.WebGLRenderer.readRenderTargetPixelsAsync: renderTarget is not in RGBA or implementation defined format.");if(!Gt.textureTypeReadable(Pt))throw new Error("THREE.WebGLRenderer.readRenderTargetPixelsAsync: renderTarget is not in UnsignedByteType or implementation defined type.");if(B>=0&&B<=T.width-X&&q>=0&&q<=T.height-H){const Tt=D.createBuffer();D.bindBuffer(D.PIXEL_PACK_BUFFER,Tt),D.bufferData(D.PIXEL_PACK_BUFFER,ct.byteLength,D.STREAM_READ),D.readPixels(B,q,X,H,mt.convert(Ct),mt.convert(Pt),0),D.flush();const Wt=D.fenceSync(D.SYNC_GPU_COMMANDS_COMPLETE,0);await $u(D,Wt,4);try{D.bindBuffer(D.PIXEL_PACK_BUFFER,Tt),D.getBufferSubData(D.PIXEL_PACK_BUFFER,0,ct)}finally{D.deleteBuffer(Tt),D.deleteSync(Wt)}return ct}}finally{const yt=I!==null?Lt.get(I).__webglFramebuffer:null;Mt.bindFramebuffer(D.FRAMEBUFFER,yt)}}},this.copyFramebufferToTexture=function(T,B=null,q=0){T.isTexture!==!0&&(Hi("WebGLRenderer: copyFramebufferToTexture function signature has changed."),B=arguments[0]||null,T=arguments[1]);const X=Math.pow(2,-q),H=Math.floor(T.image.width*X),ct=Math.floor(T.image.height*X),ft=B!==null?B.x:0,vt=B!==null?B.y:0;It.setTexture2D(T,0),D.copyTexSubImage2D(D.TEXTURE_2D,q,0,0,ft,vt,H,ct),Mt.unbindTexture()},this.copyTextureToTexture=function(T,B,q=null,X=null,H=0){T.isTexture!==!0&&(Hi("WebGLRenderer: copyTextureToTexture function signature has changed."),X=arguments[0]||null,T=arguments[1],B=arguments[2],H=arguments[3]||0,q=null);let ct,ft,vt,yt,Ct,Pt;q!==null?(ct=q.max.x-q.min.x,ft=q.max.y-q.min.y,vt=q.min.x,yt=q.min.y):(ct=T.image.width,ft=T.image.height,vt=0,yt=0),X!==null?(Ct=X.x,Pt=X.y):(Ct=0,Pt=0);const Tt=mt.convert(B.format),Wt=mt.convert(B.type);It.setTexture2D(B,0),D.pixelStorei(D.UNPACK_FLIP_Y_WEBGL,B.flipY),D.pixelStorei(D.UNPACK_PREMULTIPLY_ALPHA_WEBGL,B.premultiplyAlpha),D.pixelStorei(D.UNPACK_ALIGNMENT,B.unpackAlignment);const te=D.getParameter(D.UNPACK_ROW_LENGTH),ee=D.getParameter(D.UNPACK_IMAGE_HEIGHT),Te=D.getParameter(D.UNPACK_SKIP_PIXELS),qt=D.getParameter(D.UNPACK_SKIP_ROWS),St=D.getParameter(D.UNPACK_SKIP_IMAGES),he=T.isCompressedTexture?T.mipmaps[H]:T.image;D.pixelStorei(D.UNPACK_ROW_LENGTH,he.width),D.pixelStorei(D.UNPACK_IMAGE_HEIGHT,he.height),D.pixelStorei(D.UNPACK_SKIP_PIXELS,vt),D.pixelStorei(D.UNPACK_SKIP_ROWS,yt),T.isDataTexture?D.texSubImage2D(D.TEXTURE_2D,H,Ct,Pt,ct,ft,Tt,Wt,he.data):T.isCompressedTexture?D.compressedTexSubImage2D(D.TEXTURE_2D,H,Ct,Pt,he.width,he.height,Tt,he.data):D.texSubImage2D(D.TEXTURE_2D,H,Ct,Pt,ct,ft,Tt,Wt,he),D.pixelStorei(D.UNPACK_ROW_LENGTH,te),D.pixelStorei(D.UNPACK_IMAGE_HEIGHT,ee),D.pixelStorei(D.UNPACK_SKIP_PIXELS,Te),D.pixelStorei(D.UNPACK_SKIP_ROWS,qt),D.pixelStorei(D.UNPACK_SKIP_IMAGES,St),H===0&&B.generateMipmaps&&D.generateMipmap(D.TEXTURE_2D),Mt.unbindTexture()},this.copyTextureToTexture3D=function(T,B,q=null,X=null,H=0){T.isTexture!==!0&&(Hi("WebGLRenderer: copyTextureToTexture3D function signature has changed."),q=arguments[0]||null,X=arguments[1]||null,T=arguments[2],B=arguments[3],H=arguments[4]||0);let ct,ft,vt,yt,Ct,Pt,Tt,Wt,te;const ee=T.isCompressedTexture?T.mipmaps[H]:T.image;q!==null?(ct=q.max.x-q.min.x,ft=q.max.y-q.min.y,vt=q.max.z-q.min.z,yt=q.min.x,Ct=q.min.y,Pt=q.min.z):(ct=ee.width,ft=ee.height,vt=ee.depth,yt=0,Ct=0,Pt=0),X!==null?(Tt=X.x,Wt=X.y,te=X.z):(Tt=0,Wt=0,te=0);const Te=mt.convert(B.format),qt=mt.convert(B.type);let St;if(B.isData3DTexture)It.setTexture3D(B,0),St=D.TEXTURE_3D;else if(B.isDataArrayTexture||B.isCompressedArrayTexture)It.setTexture2DArray(B,0),St=D.TEXTURE_2D_ARRAY;else{console.warn("THREE.WebGLRenderer.copyTextureToTexture3D: only supports THREE.DataTexture3D and THREE.DataTexture2DArray.");return}D.pixelStorei(D.UNPACK_FLIP_Y_WEBGL,B.flipY),D.pixelStorei(D.UNPACK_PREMULTIPLY_ALPHA_WEBGL,B.premultiplyAlpha),D.pixelStorei(D.UNPACK_ALIGNMENT,B.unpackAlignment);const he=D.getParameter(D.UNPACK_ROW_LENGTH),Xt=D.getParameter(D.UNPACK_IMAGE_HEIGHT),Ue=D.getParameter(D.UNPACK_SKIP_PIXELS),Zn=D.getParameter(D.UNPACK_SKIP_ROWS),we=D.getParameter(D.UNPACK_SKIP_IMAGES);D.pixelStorei(D.UNPACK_ROW_LENGTH,ee.width),D.pixelStorei(D.UNPACK_IMAGE_HEIGHT,ee.height),D.pixelStorei(D.UNPACK_SKIP_PIXELS,yt),D.pixelStorei(D.UNPACK_SKIP_ROWS,Ct),D.pixelStorei(D.UNPACK_SKIP_IMAGES,Pt),T.isDataTexture||T.isData3DTexture?D.texSubImage3D(St,H,Tt,Wt,te,ct,ft,vt,Te,qt,ee.data):B.isCompressedArrayTexture?D.compressedTexSubImage3D(St,H,Tt,Wt,te,ct,ft,vt,Te,ee.data):D.texSubImage3D(St,H,Tt,Wt,te,ct,ft,vt,Te,qt,ee),D.pixelStorei(D.UNPACK_ROW_LENGTH,he),D.pixelStorei(D.UNPACK_IMAGE_HEIGHT,Xt),D.pixelStorei(D.UNPACK_SKIP_PIXELS,Ue),D.pixelStorei(D.UNPACK_SKIP_ROWS,Zn),D.pixelStorei(D.UNPACK_SKIP_IMAGES,we),H===0&&B.generateMipmaps&&D.generateMipmap(St),Mt.unbindTexture()},this.initRenderTarget=function(T){Lt.get(T).__webglFramebuffer===void 0&&It.setupRenderTarget(T)},this.initTexture=function(T){T.isCubeTexture?It.setTextureCube(T,0):T.isData3DTexture?It.setTexture3D(T,0):T.isDataArrayTexture||T.isCompressedArrayTexture?It.setTexture2DArray(T,0):It.setTexture2D(T,0),Mt.unbindTexture()},this.resetState=function(){z=0,P=0,I=null,Mt.reset(),xt.reset()},typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("observe",{detail:this}))}get coordinateSystem(){return rn}get outputColorSpace(){return this._outputColorSpace}set outputColorSpace(t){this._outputColorSpace=t;const e=this.getContext();e.drawingBufferColorSpace=t===Na?"display-p3":"srgb",e.unpackColorSpace=$t.workingColorSpace===$r?"display-p3":"srgb"}}class r0 extends fe{constructor(){super(),this.isScene=!0,this.type="Scene",this.background=null,this.environment=null,this.fog=null,this.backgroundBlurriness=0,this.backgroundIntensity=1,this.backgroundRotation=new Ge,this.environmentIntensity=1,this.environmentRotation=new Ge,this.overrideMaterial=null,typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("observe",{detail:this}))}copy(t,e){return super.copy(t,e),t.background!==null&&(this.background=t.background.clone()),t.environment!==null&&(this.environment=t.environment.clone()),t.fog!==null&&(this.fog=t.fog.clone()),this.backgroundBlurriness=t.backgroundBlurriness,this.backgroundIntensity=t.backgroundIntensity,this.backgroundRotation.copy(t.backgroundRotation),this.environmentIntensity=t.environmentIntensity,this.environmentRotation.copy(t.environmentRotation),t.overrideMaterial!==null&&(this.overrideMaterial=t.overrideMaterial.clone()),this.matrixAutoUpdate=t.matrixAutoUpdate,this}toJSON(t){const e=super.toJSON(t);return this.fog!==null&&(e.object.fog=this.fog.toJSON()),this.backgroundBlurriness>0&&(e.object.backgroundBlurriness=this.backgroundBlurriness),this.backgroundIntensity!==1&&(e.object.backgroundIntensity=this.backgroundIntensity),e.object.backgroundRotation=this.backgroundRotation.toArray(),this.environmentIntensity!==1&&(e.object.environmentIntensity=this.environmentIntensity),e.object.environmentRotation=this.environmentRotation.toArray(),e}}class ol extends Kn{constructor(t){super(),this.isLineBasicMaterial=!0,this.type="LineBasicMaterial",this.color=new Ht(16777215),this.map=null,this.linewidth=1,this.linecap="round",this.linejoin="round",this.fog=!0,this.setValues(t)}copy(t){return super.copy(t),this.color.copy(t.color),this.map=t.map,this.linewidth=t.linewidth,this.linecap=t.linecap,this.linejoin=t.linejoin,this.fog=t.fog,this}}const kr=new k,Gr=new k,Yo=new Qt,Fi=new Xc,yr=new Yr,Us=new k,Ko=new k;class s0 extends fe{constructor(t=new Ce,e=new ol){super(),this.isLine=!0,this.type="Line",this.geometry=t,this.material=e,this.updateMorphTargets()}copy(t,e){return super.copy(t,e),this.material=Array.isArray(t.material)?t.material.slice():t.material,this.geometry=t.geometry,this}computeLineDistances(){const t=this.geometry;if(t.index===null){const e=t.attributes.position,i=[0];for(let r=1,s=e.count;r<s;r++)kr.fromBufferAttribute(e,r-1),Gr.fromBufferAttribute(e,r),i[r]=i[r-1],i[r]+=kr.distanceTo(Gr);t.setAttribute("lineDistance",new xe(i,1))}else console.warn("THREE.Line.computeLineDistances(): Computation only possible with non-indexed BufferGeometry.");return this}raycast(t,e){const i=this.geometry,r=this.matrixWorld,s=t.params.Line.threshold,o=i.drawRange;if(i.boundingSphere===null&&i.computeBoundingSphere(),yr.copy(i.boundingSphere),yr.applyMatrix4(r),yr.radius+=s,t.ray.intersectsSphere(yr)===!1)return;Yo.copy(r).invert(),Fi.copy(t.ray).applyMatrix4(Yo);const a=s/((this.scale.x+this.scale.y+this.scale.z)/3),c=a*a,u=this.isLineSegments?2:1,f=i.index,m=i.attributes.position;if(f!==null){const g=Math.max(0,o.start),v=Math.min(f.count,o.start+o.count);for(let M=g,_=v-1;M<_;M+=u){const d=f.getX(M),C=f.getX(M+1),w=Mr(this,t,Fi,c,d,C);w&&e.push(w)}if(this.isLineLoop){const M=f.getX(v-1),_=f.getX(g),d=Mr(this,t,Fi,c,M,_);d&&e.push(d)}}else{const g=Math.max(0,o.start),v=Math.min(m.count,o.start+o.count);for(let M=g,_=v-1;M<_;M+=u){const d=Mr(this,t,Fi,c,M,M+1);d&&e.push(d)}if(this.isLineLoop){const M=Mr(this,t,Fi,c,v-1,g);M&&e.push(M)}}}updateMorphTargets(){const e=this.geometry.morphAttributes,i=Object.keys(e);if(i.length>0){const r=e[i[0]];if(r!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let s=0,o=r.length;s<o;s++){const a=r[s].name||String(s);this.morphTargetInfluences.push(0),this.morphTargetDictionary[a]=s}}}}}function Mr(n,t,e,i,r,s){const o=n.geometry.attributes.position;if(kr.fromBufferAttribute(o,r),Gr.fromBufferAttribute(o,s),e.distanceSqToSegment(kr,Gr,Us,Ko)>i)return;Us.applyMatrix4(n.matrixWorld);const c=t.ray.origin.distanceTo(Us);if(!(c<t.near||c>t.far))return{distance:c,point:Ko.clone().applyMatrix4(n.matrixWorld),index:r,face:null,faceIndex:null,object:n}}class jr extends Ce{constructor(t=1,e=1,i=1,r=32,s=1,o=!1,a=0,c=Math.PI*2){super(),this.type="CylinderGeometry",this.parameters={radiusTop:t,radiusBottom:e,height:i,radialSegments:r,heightSegments:s,openEnded:o,thetaStart:a,thetaLength:c};const u=this;r=Math.floor(r),s=Math.floor(s);const f=[],p=[],m=[],g=[];let v=0;const M=[],_=i/2;let d=0;C(),o===!1&&(t>0&&w(!0),e>0&&w(!1)),this.setIndex(f),this.setAttribute("position",new xe(p,3)),this.setAttribute("normal",new xe(m,3)),this.setAttribute("uv",new xe(g,2));function C(){const b=new k,z=new k;let P=0;const I=(e-t)/i;for(let F=0;F<=s;F++){const A=[],E=F/s,N=E*(e-t)+t;for(let $=0;$<=r;$++){const W=$/r,Y=W*c+a,tt=Math.sin(Y),K=Math.cos(Y);z.x=N*tt,z.y=-E*i+_,z.z=N*K,p.push(z.x,z.y,z.z),b.set(tt,I,K).normalize(),m.push(b.x,b.y,b.z),g.push(W,1-E),A.push(v++)}M.push(A)}for(let F=0;F<r;F++)for(let A=0;A<s;A++){const E=M[A][F],N=M[A+1][F],$=M[A+1][F+1],W=M[A][F+1];f.push(E,N,W),f.push(N,$,W),P+=6}u.addGroup(d,P,0),d+=P}function w(b){const z=v,P=new Bt,I=new k;let F=0;const A=b===!0?t:e,E=b===!0?1:-1;for(let $=1;$<=r;$++)p.push(0,_*E,0),m.push(0,E,0),g.push(.5,.5),v++;const N=v;for(let $=0;$<=r;$++){const Y=$/r*c+a,tt=Math.cos(Y),K=Math.sin(Y);I.x=A*K,I.y=_*E,I.z=A*tt,p.push(I.x,I.y,I.z),m.push(0,E,0),P.x=tt*.5+.5,P.y=K*.5*E+.5,g.push(P.x,P.y),v++}for(let $=0;$<r;$++){const W=z+$,Y=N+$;b===!0?f.push(Y,Y+1,W):f.push(Y+1,Y,W),F+=3}u.addGroup(d,F,b===!0?1:2),d+=F}}copy(t){return super.copy(t),this.parameters=Object.assign({},t.parameters),this}static fromJSON(t){return new jr(t.radiusTop,t.radiusBottom,t.height,t.radialSegments,t.heightSegments,t.openEnded,t.thetaStart,t.thetaLength)}}class Vr extends Ce{constructor(t=1,e=32,i=16,r=0,s=Math.PI*2,o=0,a=Math.PI){super(),this.type="SphereGeometry",this.parameters={radius:t,widthSegments:e,heightSegments:i,phiStart:r,phiLength:s,thetaStart:o,thetaLength:a},e=Math.max(3,Math.floor(e)),i=Math.max(2,Math.floor(i));const c=Math.min(o+a,Math.PI);let u=0;const f=[],p=new k,m=new k,g=[],v=[],M=[],_=[];for(let d=0;d<=i;d++){const C=[],w=d/i;let b=0;d===0&&o===0?b=.5/e:d===i&&c===Math.PI&&(b=-.5/e);for(let z=0;z<=e;z++){const P=z/e;p.x=-t*Math.cos(r+P*s)*Math.sin(o+w*a),p.y=t*Math.cos(o+w*a),p.z=t*Math.sin(r+P*s)*Math.sin(o+w*a),v.push(p.x,p.y,p.z),m.copy(p).normalize(),M.push(m.x,m.y,m.z),_.push(P+b,1-w),C.push(u++)}f.push(C)}for(let d=0;d<i;d++)for(let C=0;C<e;C++){const w=f[d][C+1],b=f[d][C],z=f[d+1][C],P=f[d+1][C+1];(d!==0||o>0)&&g.push(w,b,P),(d!==i-1||c<Math.PI)&&g.push(b,z,P)}this.setIndex(g),this.setAttribute("position",new xe(v,3)),this.setAttribute("normal",new xe(M,3)),this.setAttribute("uv",new xe(_,2))}copy(t){return super.copy(t),this.parameters=Object.assign({},t.parameters),this}static fromJSON(t){return new Vr(t.radius,t.widthSegments,t.heightSegments,t.phiStart,t.phiLength,t.thetaStart,t.thetaLength)}}class jo extends Kn{constructor(t){super(),this.isMeshStandardMaterial=!0,this.defines={STANDARD:""},this.type="MeshStandardMaterial",this.color=new Ht(16777215),this.roughness=1,this.metalness=0,this.map=null,this.lightMap=null,this.lightMapIntensity=1,this.aoMap=null,this.aoMapIntensity=1,this.emissive=new Ht(0),this.emissiveIntensity=1,this.emissiveMap=null,this.bumpMap=null,this.bumpScale=1,this.normalMap=null,this.normalMapType=Ua,this.normalScale=new Bt(1,1),this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.roughnessMap=null,this.metalnessMap=null,this.alphaMap=null,this.envMap=null,this.envMapRotation=new Ge,this.envMapIntensity=1,this.wireframe=!1,this.wireframeLinewidth=1,this.wireframeLinecap="round",this.wireframeLinejoin="round",this.flatShading=!1,this.fog=!0,this.setValues(t)}copy(t){return super.copy(t),this.defines={STANDARD:""},this.color.copy(t.color),this.roughness=t.roughness,this.metalness=t.metalness,this.map=t.map,this.lightMap=t.lightMap,this.lightMapIntensity=t.lightMapIntensity,this.aoMap=t.aoMap,this.aoMapIntensity=t.aoMapIntensity,this.emissive.copy(t.emissive),this.emissiveMap=t.emissiveMap,this.emissiveIntensity=t.emissiveIntensity,this.bumpMap=t.bumpMap,this.bumpScale=t.bumpScale,this.normalMap=t.normalMap,this.normalMapType=t.normalMapType,this.normalScale.copy(t.normalScale),this.displacementMap=t.displacementMap,this.displacementScale=t.displacementScale,this.displacementBias=t.displacementBias,this.roughnessMap=t.roughnessMap,this.metalnessMap=t.metalnessMap,this.alphaMap=t.alphaMap,this.envMap=t.envMap,this.envMapRotation.copy(t.envMapRotation),this.envMapIntensity=t.envMapIntensity,this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this.wireframeLinecap=t.wireframeLinecap,this.wireframeLinejoin=t.wireframeLinejoin,this.flatShading=t.flatShading,this.fog=t.fog,this}}class Zo extends Kn{constructor(t){super(),this.isMeshNormalMaterial=!0,this.type="MeshNormalMaterial",this.bumpMap=null,this.bumpScale=1,this.normalMap=null,this.normalMapType=Ua,this.normalScale=new Bt(1,1),this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.wireframe=!1,this.wireframeLinewidth=1,this.flatShading=!1,this.setValues(t)}copy(t){return super.copy(t),this.bumpMap=t.bumpMap,this.bumpScale=t.bumpScale,this.normalMap=t.normalMap,this.normalMapType=t.normalMapType,this.normalScale.copy(t.normalScale),this.displacementMap=t.displacementMap,this.displacementScale=t.displacementScale,this.displacementBias=t.displacementBias,this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this.flatShading=t.flatShading,this}}class a0 extends fe{constructor(t,e=1){super(),this.isLight=!0,this.type="Light",this.color=new Ht(t),this.intensity=e}dispose(){}copy(t,e){return super.copy(t,e),this.color.copy(t.color),this.intensity=t.intensity,this}toJSON(t){const e=super.toJSON(t);return e.object.color=this.color.getHex(),e.object.intensity=this.intensity,this.groundColor!==void 0&&(e.object.groundColor=this.groundColor.getHex()),this.distance!==void 0&&(e.object.distance=this.distance),this.angle!==void 0&&(e.object.angle=this.angle),this.decay!==void 0&&(e.object.decay=this.decay),this.penumbra!==void 0&&(e.object.penumbra=this.penumbra),this.shadow!==void 0&&(e.object.shadow=this.shadow.toJSON()),this.target!==void 0&&(e.object.target=this.target.uuid),e}}const Ns=new Qt,Jo=new k,Qo=new k;class o0{constructor(t){this.camera=t,this.intensity=1,this.bias=0,this.normalBias=0,this.radius=1,this.blurSamples=8,this.mapSize=new Bt(512,512),this.map=null,this.mapPass=null,this.matrix=new Qt,this.autoUpdate=!0,this.needsUpdate=!1,this._frustum=new Fa,this._frameExtents=new Bt(1,1),this._viewportCount=1,this._viewports=[new se(0,0,1,1)]}getViewportCount(){return this._viewportCount}getFrustum(){return this._frustum}updateMatrices(t){const e=this.camera,i=this.matrix;Jo.setFromMatrixPosition(t.matrixWorld),e.position.copy(Jo),Qo.setFromMatrixPosition(t.target.matrixWorld),e.lookAt(Qo),e.updateMatrixWorld(),Ns.multiplyMatrices(e.projectionMatrix,e.matrixWorldInverse),this._frustum.setFromProjectionMatrix(Ns),i.set(.5,0,0,.5,0,.5,0,.5,0,0,.5,.5,0,0,0,1),i.multiply(Ns)}getViewport(t){return this._viewports[t]}getFrameExtents(){return this._frameExtents}dispose(){this.map&&this.map.dispose(),this.mapPass&&this.mapPass.dispose()}copy(t){return this.camera=t.camera.clone(),this.intensity=t.intensity,this.bias=t.bias,this.radius=t.radius,this.mapSize.copy(t.mapSize),this}clone(){return new this.constructor().copy(this)}toJSON(){const t={};return this.intensity!==1&&(t.intensity=this.intensity),this.bias!==0&&(t.bias=this.bias),this.normalBias!==0&&(t.normalBias=this.normalBias),this.radius!==1&&(t.radius=this.radius),(this.mapSize.x!==512||this.mapSize.y!==512)&&(t.mapSize=this.mapSize.toArray()),t.camera=this.camera.toJSON(!1).object,delete t.camera.matrix,t}}class c0 extends o0{constructor(){super(new el(-5,5,5,-5,.5,500)),this.isDirectionalLightShadow=!0}}class l0 extends a0{constructor(t,e){super(t,e),this.isDirectionalLight=!0,this.type="DirectionalLight",this.position.copy(fe.DEFAULT_UP),this.updateMatrix(),this.target=new fe,this.shadow=new c0}dispose(){this.shadow.dispose()}copy(t){return super.copy(t),this.target=t.target.clone(),this.shadow=t.shadow.clone(),this}}typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("register",{detail:{revision:ba}}));typeof window<"u"&&(window.__THREE__?console.warn("WARNING: Multiple instances of Three.js being imported."):window.__THREE__=ba);function u0(n){const t=n[214]==="3",e=n[214]==="b",i=(parseInt(n[215],16)&1)===1;return{movementState:"??",left:e,right:t,shift:i,forward:!1}}const h0=[{name:"Check_Point_2",uniform_scale:1,position:{x:524.914978,y:-72.053261,z:1609.72998},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:469.625,y:-50.83231,z:1613.47998},quat:{w:1,x:0,y:0,z:0}},{name:"Start_Point",uniform_scale:1,position:{x:645.16803,y:-831.709106,z:110.084},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:486.019012,y:-535.026123,z:676.150024},quat:{w:.931603,x:0,y:-.363478,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.710999,y:-841.440308,z:97.760902},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.682007,y:-839.661926,z:100.323997},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.471008,y:-839.745483,z:100.343002},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.054993,y:-839.763367,z:100.378998},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:640.877991,y:-841.469482,z:97.952599},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.987976,y:-843.000122,z:95.099403},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:643.051025,y:-844.409241,z:92.487297},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.125,y:-844.462646,z:92.621201},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.103027,y:-843.025818,z:95.214104},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:645.038025,y:-844.440796,z:92.780098},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.825989,y:-843.063599,z:95.252403},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.745972,y:-841.444153,z:97.939796},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Finish_Point",uniform_scale:1.396,position:{x:620.690979,y:372.30188,z:2564.110107},quat:{w:1,x:0,y:0,z:0}}],f0=[{name:"Start_Point",uniform_scale:1,position:{x:645.16803,y:-831.709106,z:110.084},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.710999,y:-841.440308,z:97.760902},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.682007,y:-839.661926,z:100.323997},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.471008,y:-839.745483,z:100.343002},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.054993,y:-839.763367,z:100.378998},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:640.877991,y:-841.469482,z:97.952599},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.987976,y:-843.000122,z:95.099403},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:643.051025,y:-844.409241,z:92.487297},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.125,y:-844.462646,z:92.621201},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.103027,y:-843.025818,z:95.214104},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:645.038025,y:-844.440796,z:92.780098},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.825989,y:-843.063599,z:95.252403},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.745972,y:-841.444153,z:97.939796},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:1051.060059,y:68.596832,z:1590.109985},quat:{w:.9987,x:0,y:.0509779,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:637.55603,y:-585.382996,z:606.434998},quat:{w:1,x:0,y:0,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:1119.880005,y:543.206604,z:2291.780029},quat:{w:1,x:0,y:0,z:0}}],d0=[{name:"Start_Point",uniform_scale:1,position:{x:645.16803,y:-831.709106,z:110.084},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.710999,y:-841.440308,z:97.760902},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.682007,y:-839.661926,z:100.323997},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.471008,y:-839.745483,z:100.343002},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.054993,y:-839.763367,z:100.378998},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:640.877991,y:-841.469482,z:97.952599},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:642.987976,y:-843.000122,z:95.099403},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:643.051025,y:-844.409241,z:92.487297},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.125,y:-844.462646,z:92.621201},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:641.103027,y:-843.025818,z:95.214104},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:645.038025,y:-844.440796,z:92.780098},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.825989,y:-843.063599,z:95.252403},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:644.745972,y:-841.444153,z:97.939796},quat:{w:.999982,x:0,y:-.00600007,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:608.411011,y:-551.169128,z:654.901001},quat:{w:.996218,x:0,y:.0868903,z:0}},{name:"Finish_Point",uniform_scale:1.396,position:{x:620.711975,y:372.307526,z:2564.129883},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:718.445007,y:-147.717392,z:1429.160034},quat:{w:.993846,x:0,y:.110772,z:0}}],p0=[{name:"Finish_Point",uniform_scale:1,position:{x:534.888,y:-284.277588,z:2313.639893},quat:{w:1,x:0,y:0,z:0}},{name:"Start_Point",uniform_scale:1,position:{x:519.367981,y:-1384.164185,z:99.7817},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:671.789978,y:-631.825745,z:1691.939941},quat:{w:.996721,x:0,y:-.0809115,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:334.958008,y:-701.713623,z:1544.140015},quat:{w:.98965,x:0,y:-.143503,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:259.109985,y:-1044.600708,z:872.73999},quat:{w:.997823,x:0,y:.0659521,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:507.769989,y:-691.222168,z:1560.670044},quat:{w:.990901,x:0,y:.13459,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:512.228027,y:-1066.99585,z:843.586975},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:526.361023,y:-1401.963379,z:52.828701},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:524.234009,y:-1401.888062,z:52.872601},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:521.651001,y:-1401.730713,z:53.207901},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:528.848999,y:-1401.966797,z:52.491501},quat:{w:.999456,x:0,y:-.0329958,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:509.837006,y:-1401.014038,z:55.337502},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:513.203979,y:-1401.264648,z:54.7066},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:516.369995,y:-1401.418091,z:54.133801},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:519.234009,y:-1401.579346,z:53.577999},quat:{w:1,x:0,y:0,z:0}},{name:"Start_Point",uniform_scale:1.612,position:{x:412.575012,y:-232.187561,z:-64.914001},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:723.994995,y:-1124.040405,z:702.659973},quat:{w:.964567,x:0,y:.263839,z:0}}],m0=[{name:"Check_Point_1",uniform_scale:1,position:{x:723.994995,y:-1247.439087,z:702.659973},quat:{w:.964567,x:0,y:.263839,z:0}},{name:"Start_Point",uniform_scale:1,position:{x:518.552979,y:-1535.383911,z:99.917},quat:{w:1,x:0,y:0,z:0}},{name:"Start_Point",uniform_scale:1.612,position:{x:412.575012,y:-257.285614,z:-64.914001},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:519.234009,y:-1554.089722,z:53.577999},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:516.369995,y:-1553.914795,z:54.133801},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:513.203979,y:-1553.747314,z:54.7066},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:509.837006,y:-1553.481201,z:55.337502},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:528.848999,y:-1554.503784,z:52.491501},quat:{w:.999456,x:0,y:-.0329958,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:521.651001,y:-1554.250122,z:53.207901},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:524.234009,y:-1554.415649,z:52.872601},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:526.361023,y:-1554.492065,z:52.828701},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:512.228027,y:-1185.055298,z:843.586975},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:507.769989,y:-767.855957,z:1560.670044},quat:{w:.990901,x:0,y:.13459,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:259.109985,y:-1160.998779,z:872.73999},quat:{w:.997823,x:0,y:.0659521,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:334.958008,y:-779.157715,z:1544.140015},quat:{w:.98965,x:0,y:-.143503,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:543.77002,y:-304.486206,z:2340.919922},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:671.789978,y:-702.809082,z:1691.939941},quat:{w:.996721,x:0,y:-.0809115,z:0}}],_0=[{name:"Check_Point_1",uniform_scale:1,position:{x:723.994995,y:-1247.439087,z:702.659973},quat:{w:.964567,x:0,y:.263839,z:0}},{name:"Start_Point",uniform_scale:1,position:{x:518.552979,y:-1535.383911,z:99.917},quat:{w:1,x:0,y:0,z:0}},{name:"Start_Point",uniform_scale:1.612,position:{x:412.575012,y:-257.285614,z:-64.914001},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:519.234009,y:-1554.089722,z:53.577999},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:516.369995,y:-1553.914795,z:54.133801},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:513.203979,y:-1553.747314,z:54.7066},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:509.837006,y:-1553.481201,z:55.337502},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:528.848999,y:-1554.503784,z:52.491501},quat:{w:.999456,x:0,y:-.0329958,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:521.651001,y:-1554.250122,z:53.207901},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:524.234009,y:-1554.415649,z:52.872601},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:526.361023,y:-1554.492065,z:52.828701},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:512.228027,y:-1185.055298,z:843.586975},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:507.769989,y:-767.855957,z:1560.670044},quat:{w:.990901,x:0,y:.13459,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:259.109985,y:-1160.998779,z:872.73999},quat:{w:.997823,x:0,y:.0659521,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:334.958008,y:-779.157715,z:1544.140015},quat:{w:.98965,x:0,y:-.143503,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:543.77002,y:-304.486206,z:2340.919922},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:671.789978,y:-702.809082,z:1691.939941},quat:{w:.996721,x:0,y:-.0809115,z:0}}],g0=[{name:"Player_Start_Location",uniform_scale:1,position:{x:734.325012,y:-461.033691,z:225.824997},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:732.656006,y:-461.06723,z:225.498001},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:731.10199,y:-461.079224,z:225.306},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:729.349976,y:-461.088135,z:225.104996},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:727.888,y:-461.091522,z:225},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:726.346985,y:-461.093994,z:224.899994},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:724.721008,y:-461.095154,z:224.837006},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:722.926025,y:-461.089417,z:224.729996},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:490.509003,y:-283.806305,z:607.773987},quat:{w:1,x:0,y:0,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:430.776001,y:738.914917,z:2657.77002},quat:{w:1,x:0,y:0,z:0}},{name:"Start_Point",uniform_scale:1,position:{x:729.526978,y:-457.720856,z:231.772003},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:432.812012,y:268.643463,z:1832},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:547.317017,y:265.436798,z:1833.48999},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:250,y:271.255768,z:1832},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:130,y:266.659546,z:1832},quat:{w:1,x:0,y:0,z:0}}],x0=[{name:"Player_Start_Location",uniform_scale:1,position:{x:433.334015,y:-513.037354,z:96.240799},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:435.20401,y:-513.162476,z:95.375198},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:437.582001,y:-513.566772,z:94.366997},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:440.243988,y:-513.761963,z:93.888298},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:442.778015,y:-513.983032,z:93.335999},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:248.843994,y:-45.584229,z:1096.150024},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:276.050995,y:-274.875824,z:464.118011},quat:{w:.962966,x:0,y:-.269621,z:0}},{name:"Start_Point",uniform_scale:1,position:{x:439.385986,y:-501.645935,z:114.804001},quat:{w:1,x:0,y:0,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:211.158005,y:292.066498,z:1897.359985},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:449.298004,y:-513.57428,z:94.264702},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:447.554993,y:-513.726257,z:93.949699},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:444.868011,y:-513.97467,z:93.357101},quat:{w:1,x:0,y:0,z:0}}],v0=[{name:"Check_Point_1",uniform_scale:1,position:{x:595.789978,y:-145.276123,z:827.879028},quat:{w:1,x:0,y:0,z:0}},{name:"Start_Point",uniform_scale:1,position:{x:801,y:-481.165039,z:299.283997},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:852.960022,y:-109.809311,z:847.317993},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:954.27002,y:387.389771,z:1827.089966},quat:{w:.966132,x:0,y:.258047,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:334.089996,y:404.545593,z:1607.920044},quat:{w:1,x:0,y:0,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:605.559998,y:1014.658264,z:2583.449951},quat:{w:.999118,x:0,y:-.0419877,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:562.330017,y:1013.05835,z:2580.01001},quat:{w:1,x:0,y:0,z:0}},{name:"Finish_Point",uniform_scale:1,position:{x:661.130005,y:1013.058411,z:2589.110107},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:805.97998,y:-484.148041,z:291.459991},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:808.049988,y:-484.111481,z:291.584015},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:799.973999,y:-484.205536,z:290.96701},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:798.021973,y:-484.156891,z:291.47699},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:795.872986,y:-484.126862,z:291.640015},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:793.724976,y:-484.082977,z:291.743011},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:801.661987,y:-484.198792,z:291.009003},quat:{w:1,x:0,y:0,z:0}},{name:"Player_Start_Location",uniform_scale:1,position:{x:803.802979,y:-484.175079,z:291.208008},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:15,y:314.300995,z:1607.920044},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:1032.959961,y:-148.53656,z:847.317993},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_1",uniform_scale:1,position:{x:1200,y:-192.438019,z:847.317993},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:1400,y:347.761963,z:1827.089966},quat:{w:1,x:0,y:0,z:0}},{name:"Check_Point_2",uniform_scale:1,position:{x:1576,y:349.224152,z:1827.089966},quat:{w:1,x:0,y:0,z:0}}],y0={AlpineEasy:h0,AlpineHard:f0,AlpineMedium:d0,ForestEasy:p0,ForestHard:m0,ForestMedium:_0,VillageEasy:g0,VillageHard:x0,VillageMedium:v0};let Sr;function cl(){if(!Sr){Sr=[];for(const[n,t]of Object.entries(y0))Sr.push({name:n,entries:t})}return Sr}function Ur(n,t){const e=t.x-n.x,i=t.y-n.y,r=t.z-n.z;return Math.sqrt(e*e+i*i+r*r)}function M0(n,t){const e={x:2*(n.quat.x*n.quat.z+n.quat.w*n.quat.y),y:2*(n.quat.y*n.quat.z-n.quat.w*n.quat.x),z:1-2*(n.quat.x*n.quat.x+n.quat.y*n.quat.y)},i=Math.sqrt(e.x*e.x+e.y*e.y+e.z*e.z);if(i<1e-6)return;const r={x:e.x/i,y:e.y/i,z:e.z/i},s={x:t.p2.x-t.p1.x,y:t.p2.y-t.p1.y,z:t.p2.z-t.p1.z},o=r.x*s.x+r.y*s.y+r.z*s.z;if(Math.abs(o)<1e-6)return;const a={x:t.p1.x-n.position.x,y:t.p1.y-n.position.y,z:t.p1.z-n.position.z},u=-(r.x*a.x+r.y*a.y+r.z*a.z)/o;return u<0||u>1?void 0:{x:t.p1.x+u*s.x,y:t.p1.y+u*s.y,z:t.p1.z+u*s.z}}function S0(n){const t=[],e=n.map(r=>({x:r.x,y:r.y,z:r.z})),i=["Check_Point_1","Check_Point_2","Start_Point","Finish_Point"];for(const r of cl()){const o=r.entries.filter(c=>i.includes(c.name)).map(c=>({planeName:c.name,plane:{position:c.position,quat:c.quat}})),a={name:r.name,collisions:[]};for(let c=0;c<n.length-1;c++){const u=e[c],f=e[c+1];for(const{planeName:p,plane:m}of o){const g=M0(m,{p1:u,p2:f});g!==void 0&&a.collisions.push({intersection:g,distance:Ur(g,m.position),p1:u,p2:f,frame1:c,frame2:c+1,objectName:p,plane:m})}}t.push(a)}return t}const Ba=90,tc=5;function E0(n){const t=S0(n.coords.rows),e=T0(n,t);return{allCollisions:t,levelScores:e}}function T0(n,t){var i,r,s;let e=[];for(const o of t){const a=w0(o.name,n.coords.rows),c=(i=Fs(o,"Start_Point"))==null?void 0:i.at(0),u=(r=Fs(o,"Check_Point_1"))==null?void 0:r.at(0),f=(s=Fs(o,"Finish_Point"))==null?void 0:s.at(0),p=o.collisions.filter(_=>_.objectName==="Check_Point_2"),m=p.filter(_=>_.distance<=Ba).filter(_=>!c||c.frame2<=_.frame1).filter(_=>!u||u.frame2<=_.frame1).filter(_=>!f||_.frame2<=f.frame2),g={nearestStartDist:a.dist,startPlaneDiffMs:Os(n,c,"Start_Point"),checkpoint1DiffMs:Os(n,u,"Check_Point_1"),finishPointDiffMs:Os(n,f,"Finish_Point"),checkpoint2Exists:m.length>0,firstValidStartPointCollision:c,firstValidCheckPoint1Collision:u,firstValidFinishPointCollision:f,allCheckPoint2Collisions:p,validCheckPoint2Collisions:m},v=ll(g),M=Object.values(v).filter(_=>_).length;e.push({name:o.name,scoreData:g,score:M})}return e=e.sort((o,a)=>a.score-o.score),e}function Fs(n,t){return n.collisions.filter(e=>e.objectName===t&&e.distance<=Ba)}function Os(n,t,e){if(!t)return;const{crossStartPlusStartDelayMs:i,totalTimeToFinishMs:r,checkpoint1TotalMs:s}=n.timingDataFromHeader;switch(e){case"Start_Point":return t.frame2*10-i;case"Check_Point_1":return t.frame2*10-s;case"Finish_Point":return t.frame2*10-r;default:throw new Error(`Unhandled ${e}`)}}function w0(n,t){const e=cl().filter(u=>u.name===n);if(e.length!==1)throw new Error(`Expected to find only one level with levelName: ${n}, found ${e.length}`);const r=e[0].entries.filter(u=>u.name==="Player_Start_Location");if(r.length===0)throw new Error("No start locations");let s=r[0];const o={x:t[0].x,y:t[0].y,z:t[0].z};for(const u of r){const f=s.position,p=u.position,m=Ur(o,f);Ur(o,p)<m&&(s=u)}const a=s.position,c=Ur(o,a);return{gameObject:s,pos:a,dist:c}}function ll(n){return{hasPlayerStartLocation:n.nearestStartDist<=2,hasStartPlane:n.startPlaneDiffMs!==void 0&&Math.abs(n.startPlaneDiffMs)<=10,hasCheckPoint1:n.checkpoint1DiffMs!==void 0&&Math.abs(n.checkpoint1DiffMs)<=10,hasFinishPoint:n.finishPointDiffMs!==void 0&&Math.abs(n.finishPointDiffMs)<=10,hasCheckPoint2:n.checkpoint2Exists}}function A0(n){const t=ll(n),e=[];return e.push(["Has Player Start Location ≤2m",{valid:t.hasPlayerStartLocation}]),e.push(["Has Start_Point ≤10ms, ≤90m",{valid:t.hasStartPlane}]),e.push(["Has Check_Point_1 ≤10ms, ≤90m",{valid:t.hasCheckPoint1}]),e.push(["Has Finish_Point ≤10ms, ≤90m",{valid:t.hasFinishPoint}]),e.push(["Has Check_Point_2 ≤90m",{valid:t.hasCheckPoint2}]),e}const $n="Unknown Track";function ul(n,t){const{playerName:e,endNameAddr:i}=hl(n),r=b0(n,i),s=r.totalTimeToFinishMs-r.crossStartPlusStartDelayMs,o=R0(n)*10,a=r.crossStartPlusStartDelayMs-o,c=r.totalRecordingTimeMs-r.totalTimeToFinishMs,u={playerName:e,trackName:$n,displayedMs:s,startMs:a,totalMs:s+a,lagBeforeStartMs:o,lagAfterFinishMs:c,recordingMs:r.totalRecordingTimeMs,checkpoint1Ms:r.checkpoint1TotalMs-r.crossStartPlusStartDelayMs,timingDataFromHeader:r};if(!t||!t.skipCoords){const f=P0(n,r);u.coords=f;const p=E0(u);u.trackScoreData=p;const m=p.levelScores[0].score;if(m!==0){const g=p.levelScores.filter(v=>v.score===m).map(v=>v.name).sort();g.length===2&&g[0]==="ForestHard"&&g[1]==="ForestMedium"?u.trackName="Forest MediumOrHard":g.length===1?u.trackName=g[0]:u.trackName="UnknownTrack"}}return u}function hl(n){let t="",e="",i=48;for(;i<n.length;i+=2){if(n.slice(i,i+2)==="00"){e=Buffer.from(t,"hex").toString("utf8");break}t+=n.slice(i,i+2)}return{playerName:e,endNameAddr:i}}function b0(n,t){return{totalRecordingTimeMs:10*Er(n.slice(8,12)),crossStartPlusStartDelayMs:10*Er(n.slice(t+2,t+6)),checkpoint1TotalMs:10*Er(n.slice(t+10,t+14)),totalTimeToFinishMs:10*Er(n.slice(t+18,t+22))}}function Er(n){const t=parseInt(n.slice(0,2),16);return parseInt(n.slice(2,4),16)<<8|t}const fl=new Uint32Array(1),C0=new Float32Array(fl.buffer);function zi(n){let t="";for(let e=n.length-2;e>=0;e-=2)t+=n.substring(e,e+2);return fl[0]=parseInt(t,16),C0[0]}function R0(n){const t=dl(n);for(let e=1;e<t.length-1;++e)if(t[e]!==t[e+1])return e;throw new Error("unreachable")}function dl(n){const{playerName:t,endNameAddr:e}=hl(n),i=26,r=[n.slice(0,e+i)];for(let s=e+i;s<n.length;s+=218)r.push(n.slice(s,s+218));return r}function P0(n,t){var a;const e=dl(n),i=(a=e.at(-1))==null?void 0:a.match(/^[0]+$/),r={rows:[]},s=e.length-1-(i?1:0),o=t.totalRecordingTimeMs/10;s!==o&&console.warn(`Warning: Expected ${o} coordinate blocks, but got ${s}`);for(let c=1;c<e.length-(i?1:0);++c){const u=e[c],f=88,p=f+3*8,m=u0(u),g=[];for(let M=0;M<3;M++){const _=[];for(let d=0;d<3;d++){const C=p+(M*3+d)*8;_.push(zi(u.slice(C,C+8)))}g.push(_)}const v={x:zi(u.slice(f,f+8)),y:zi(u.slice(f+8,f+2*8)),z:zi(u.slice(f+2*8,f+3*8)),rotation3x3:g,...m,ex:u.slice(202,208),raw:u};r.rows.push(v)}return r}async function L0(){return await pl("replays/Village/Medium/VM 1.09.08 Dom.dat")}async function pl(n){const e=await(await fetch(n)).arrayBuffer(),i=Buffer.from(e).toString("hex");return ul(i,!0)}const I0={ah:"HOSM3c2Zdf0",vm:"_DC0g90k6eE",vm109:"Ja3OmVZS2jQ"};function D0(n,t){console.log("Setup Video");const e=document.createElement("script"),i={videoTarget:void 0};e.src="https://www.youtube.com/iframe_api";let r=document.getElementsByTagName("script")[0];r.parentNode.insertBefore(e,r);let s;return window.onYouTubeIframeAPIReady=()=>{s=new YT.Player("player",{...t,videoId:n,playerVars:{playsinline:1,autoplay:1,rel:0,modestbranding:1,controls:1,showinfo:0,fs:1,cc_load_policy:0,iv_load_policy:3,autohide:1,enablejsapi:1,"endscreen-client_20":0},events:{onReady:o=>{o.target.playVideo(),i.videoTarget=o.target},onStateChange:o=>{o.data===YT.PlayerState.ENDED&&(s.seekTo(0),s.playVideo())}}})},i}function U0(n,t){if(t<1||t>=n.length-1)return 0;const e=1/100,i=n[t-1].y,r=n[t].y;return(n[t+1].y-2*r+i)/(e*e)}function ec(n,t,e=100){if(t===0)return 0;const i=n[t-1],r=n[t],s=r.x-i.x,o=r.y-i.y,a=r.z-i.z,c=Math.sqrt(s*s+o*o+a*a),u=1/(e*3600);return c/1e3/u}class N0{constructor(t,e={}){this.coordinates=t,this.jumpDetectionConfig={accelerationThreshold:e.accelerationThreshold??4.5,requiredDuration:e.requiredDuration??50}}calculateAcceleration(t){if(t<1||t>=this.coordinates.length-1)return 0;const e=1/100,i=this.coordinates[t-1].y,r=this.coordinates[t].y;return-(this.coordinates[t+1].y-2*r+i)/(e*e)}detectJumps(){const t=[],{accelerationThreshold:e,requiredDuration:i}=this.jumpDetectionConfig;let r=null,s=0;for(let o=1;o<this.coordinates.length-1;o++){const a=this.calculateAcceleration(o);a>e?(s++,r===null&&(r=o),s===i&&r!==null&&t.push({index:r,position:this.coordinates[r],acceleration:a,type:"takeoff"})):(r!==null&&s>=i&&t.push({index:o,position:this.coordinates[o],acceleration:a,type:"landing"}),r=null,s=0)}return t}createSlopeMesh(t={}){const{width:e=30,roughness:i=.8,metalness:r=.2,playerYOffset:s=1,debug:o=!1}=t,a=new En,c=this.detectJumps(),u=v=>new k(-v.z,0,v.x).normalize().multiplyScalar(e/2),f=(v,M)=>{const _=[],d=[];for(let w=v;w<=M;w++){const b=this.coordinates[w],z=this.coordinates[Math.min(w+1,M)],P=new k(z.x-b.x,z.y-b.y,z.z-b.z).normalize(),I=u(P);if(_.push(b.x+I.x,b.y-s,b.z+I.z,b.x-I.x,b.y-s,b.z-I.z),w<M){const F=(w-v)*2;d.push(F,F+1,F+2,F+1,F+3,F+2)}}const C=new Ce;return C.setAttribute("position",new xe(_,3)),C.setIndex(d),C.computeVertexNormals(),C},p=o?new sn({color:16777215,wireframe:!0,side:ge}):new jo({color:15790320,roughness:.9,metalness:.1,side:ge}),m=o?new sn({color:4210752,wireframe:!0,side:ge}):new jo({color:4210752,roughness:i,metalness:r,side:ge});let g=0;if(c.forEach(v=>{if(v.type==="takeoff"){if(g<v.index){const _=f(g,v.index),d=new ae(_,p);d.receiveShadow=!0,a.add(d)}const M=c.find(_=>_.type==="landing"&&_.index>v.index);if(M){const _=[],d=[],C=this.coordinates[v.index],w=this.coordinates[M.index],b=new k(w.x-C.x,w.y-C.y,w.z-C.z).normalize(),z=u(b),P=u(b);_.push(C.x+z.x,C.y-s,C.z+z.z,C.x-z.x,C.y-s,C.z-z.z,w.x+P.x,w.y-s,w.z+P.z,w.x-P.x,w.y-s,w.z-P.z),d.push(0,1,2,1,3,2);const I=new Ce;I.setAttribute("position",new xe(_,3)),I.setIndex(d),I.computeVertexNormals(),a.add(new ae(I,m)),g=M.index}}}),g<this.coordinates.length-1){const v=f(g,this.coordinates.length-1);a.add(new ae(v,p))}return a}createJumpMarkers(t={}){const{width:e=30,takeoffColor:i=65280,landingColor:r=16711680}=t;return this.detectJumps().map(o=>{const a=new En,c=new Ki(4,3),u=new sn({color:o.type==="takeoff"?i:r,side:ge,transparent:!1}),f=new ae(c,u);f.position.y=5,f.position.x=2;const p=12,m=new jr(.1,.1,p),g=new sn({color:13421772}),v=new ae(m,g);v.position.y=-p/2+5,a.add(v),a.add(f);const M=Math.max(0,o.index-1),_=new k(this.coordinates[o.index].x-this.coordinates[M].x,this.coordinates[o.index].y-this.coordinates[M].y,this.coordinates[o.index].z-this.coordinates[M].z).normalize(),d=new k(-_.z,0,_.x).normalize().multiplyScalar(e/2),C=o.type==="takeoff"?-1:1;return a.position.set(o.position.x+d.x*C,o.position.y,o.position.z+d.z*C),a})}}const F0={distance:1.5,height:1.5,smoothing:.995};function O0(n,t,e={}){const i={...F0,...e},r=new k,s=new k,o=new k(0,0,1),a=new Ce,c=new ol({color:65280,transparent:!0,opacity:.5}),u=new Float32Array(300);a.setAttribute("position",new ke(u,3));const f=new s0(a,c),p=[];function m(v,M,_,d){const C=new k;M&&(C.set(-v.x- -M.x,-v.y- -M.y,v.z-M.z),C.lengthSq()>.01&&(C.normalize(),o.copy(C))),p.unshift(new k(-v.x,-v.y,v.z)),p.length>100&&p.pop();const w=f.geometry.attributes.position.array;for(let I=0;I<p.length;I++)w[I*3]=p[I].x,w[I*3+1]=p[I].y,w[I*3+2]=p[I].z;f.geometry.attributes.position.needsUpdate=!0;const b=new k(-o.x*i.distance,i.height,-o.z*i.distance),z=new k(-v.x+b.x,-v.y+b.y,v.z+b.z);s.lerp(z,i.smoothing),n.position.copy(s);const P=new k(-v.x,-v.y,v.z);r.lerp(P,i.smoothing),n.lookAt(r)}function g(v){Object.assign(i,v)}return{trail:f,trailPoints:p,updateCamera:m,updateConfig:g,getConfig:()=>({...i})}}function B0(n=.04){const t=new En,e={headRadius:4,neckRadius:2,neckHeight:6,bodyWidth:5,bodyHeight:12.5,bodyDepth:7.5,boardWidth:6.25,boardHeight:.5,boardLength:20,headY:15,neckY:9,boardY:-7},i=e.headRadius*n,r=32,s=new Vr(i,r,r,0,Math.PI,0,Math.PI),o=new sn({color:65280,side:ge}),a=new ae(s,o),c=new Vr(i,r,r,Math.PI,Math.PI,0,Math.PI),u=new sn({color:16711680,side:ge}),f=new ae(c,u),p=new En;p.add(a),p.add(f),p.position.y=e.headY*n,a.castShadow=!0,f.castShadow=!0;const m=new jr(e.neckRadius*n,e.neckRadius*n,e.neckHeight*n,32),g=new Zo,v=new ae(m,g);v.position.y=e.neckY*n;const M=new Xn(e.bodyWidth*n,e.bodyHeight*n,e.bodyDepth*n),_=new Zo,d=new ae(M,_),C=new Xn(e.boardWidth*n,e.boardHeight*n,e.boardLength*n),w=new sn({color:16776960}),b=new ae(C,w);return b.position.y=e.boardY*n,t.add(d),t.add(v),t.add(p),t.add(b),d.castShadow=!0,v.castShadow=!0,b.castShadow=!0,t}function z0({processCallback:n}){const t=Tr("dropzone"),e=Tr("replayFile");Tr("dropzoneLabel");const i=Tr("dropZoneSubLabel");function r(f,p=!1){i.textContent=f,p&&(t.classList.add("error"),setTimeout(()=>{i.textContent="Drag and drop replay file here",t.classList.remove("error")},3e3))}async function s(f){r("Processing...");try{const p=await f.arrayBuffer(),m=Buffer.from(new Uint8Array(p)).toString("hex");await n(m),r(`Processed ${f.name}`)}catch(p){console.error("Error processing file:",p),r("Error processing file",!0)}}function o(f){f.preventDefault(),f.stopPropagation(),t.classList.add("dragover")}function a(f){f.preventDefault(),f.stopPropagation(),t.classList.remove("dragover")}async function c(f){var m;f.preventDefault(),f.stopPropagation(),t.classList.remove("dragover");const p=(m=f.dataTransfer)==null?void 0:m.files[0];p&&await s(p)}async function u(f){var g;const m=(g=f.target.files)==null?void 0:g[0];m&&await s(m)}t.addEventListener("dragover",o),t.addEventListener("dragleave",a),t.addEventListener("drop",c),e.addEventListener("change",u),document.addEventListener("dragover",f=>f.preventDefault()),document.addEventListener("drop",f=>f.preventDefault())}function Tr(n){const t=document.getElementById(n);if(!t)throw new Error(`Element with id '${n}' not found`);return t}function H0(n,t,e){const{offsetSeconds:i,videoId:r,replayFile:s,syncWithVideo:o}=ml();_l(n),r.addEventListener("change",()=>{var a;n.videoId=r.value,(a=t.videoTarget)==null||a.loadVideoById(r.value)}),i.addEventListener("change",()=>{n.offsetSeconds=parseFloat(i.value)}),o.addEventListener("change",()=>{n.syncWithVideo=o.checked}),z0({processCallback:async a=>{const c=await ul(a,!0);e(c)}})}function ml(){const n=document.getElementById("offsetSeconds"),t=document.getElementById("videoId"),e=document.getElementById("replayFile"),i=document.getElementById("syncWithVideo");return{offsetSeconds:n,videoId:t,replayFile:e,syncWithVideo:i}}function _l(n){const{offsetSeconds:t,videoId:e,syncWithVideo:i,replayFile:r}=ml();t.value=n.offsetSeconds.toString(),e.value=n.videoId,i.checked=n.syncWithVideo}function k0(n,t,e){const i=document.getElementById("presetSelector"),r=[{name:"Dom VM 1.09.08 with speed & inputs",videoId:"Ja3OmVZS2jQ",datFile:"replays/Village/Medium/VM 1.09.08 Dom.dat",offsetSeconds:-1.05},{name:"Dom AE 1.19.93",videoId:"VQJ3hyQctoY",datFile:"replays/Alpine/Easy/1.19.93 Dom.dat",offsetSeconds:-.21},{name:"Dom AM 1.22.90",videoId:"drLyc1Ty9sw",datFile:"replays/Alpine/Medium/1.22.91 Dom.dat",offsetSeconds:-.21},{name:"Dom FH 0.57.06",videoId:"DIPZX8KbBfQ",datFile:"replays/Forest/Hard/0.57.06 Dom.dat",offsetSeconds:-.21},{name:"Dom VE 1.17.06",videoId:"FtyCM-26eGg",datFile:"replays/Village/Easy/1.17.06 Dom.dat",offsetSeconds:-.22},{name:"Dom VM 1.02.44",videoId:"qblHObF7np8",datFile:"replays/Village/Medium/1.02.44 Dom.dat",offsetSeconds:-.21}];for(const s of r){const o=document.createElement("option");o.value=s.videoId,o.text=s.name,i.add(o)}i.addEventListener("change",async()=>{var a;const s=r.find(c=>c.videoId===i.value);if(!s)return;const o=await pl(s.datFile);n.videoId=s.videoId,n.offsetSeconds=s.offsetSeconds,(a=t.videoTarget)==null||a.loadVideoById(s.videoId),e(o)})}var G0=typeof global=="object"&&global&&global.Object===Object&&global;const gl=G0;var V0=typeof self=="object"&&self&&self.Object===Object&&self,W0=gl||V0||Function("return this")();const hn=W0;var q0=hn.Symbol;const Cn=q0;var xl=Object.prototype,X0=xl.hasOwnProperty,$0=xl.toString,Oi=Cn?Cn.toStringTag:void 0;function Y0(n){var t=X0.call(n,Oi),e=n[Oi];try{n[Oi]=void 0;var i=!0}catch{}var r=$0.call(n);return i&&(t?n[Oi]=e:delete n[Oi]),r}var K0=Object.prototype,j0=K0.toString;function Z0(n){return j0.call(n)}var J0="[object Null]",Q0="[object Undefined]",nc=Cn?Cn.toStringTag:void 0;function Ci(n){return n==null?n===void 0?Q0:J0:nc&&nc in Object(n)?Y0(n):Z0(n)}function wi(n){return n!=null&&typeof n=="object"}var t_="[object Symbol]";function Zr(n){return typeof n=="symbol"||wi(n)&&Ci(n)==t_}function e_(n,t){for(var e=-1,i=n==null?0:n.length,r=Array(i);++e<i;)r[e]=t(n[e],e,n);return r}var n_=Array.isArray;const un=n_;var i_=1/0,ic=Cn?Cn.prototype:void 0,rc=ic?ic.toString:void 0;function vl(n){if(typeof n=="string")return n;if(un(n))return e_(n,vl)+"";if(Zr(n))return rc?rc.call(n):"";var t=n+"";return t=="0"&&1/n==-i_?"-0":t}function za(n){var t=typeof n;return n!=null&&(t=="object"||t=="function")}function r_(n){return n}var s_="[object AsyncFunction]",a_="[object Function]",o_="[object GeneratorFunction]",c_="[object Proxy]";function yl(n){if(!za(n))return!1;var t=Ci(n);return t==a_||t==o_||t==s_||t==c_}var l_=hn["__core-js_shared__"];const Bs=l_;var sc=function(){var n=/[^.]+$/.exec(Bs&&Bs.keys&&Bs.keys.IE_PROTO||"");return n?"Symbol(src)_1."+n:""}();function u_(n){return!!sc&&sc in n}var h_=Function.prototype,f_=h_.toString;function jn(n){if(n!=null){try{return f_.call(n)}catch{}try{return n+""}catch{}}return""}var d_=/[\\^$.*+?()[\]{}|]/g,p_=/^\[object .+?Constructor\]$/,m_=Function.prototype,__=Object.prototype,g_=m_.toString,x_=__.hasOwnProperty,v_=RegExp("^"+g_.call(x_).replace(d_,"\\$&").replace(/hasOwnProperty|(function).*?(?=\\\()| for .+?(?=\\\])/g,"$1.*?")+"$");function y_(n){if(!za(n)||u_(n))return!1;var t=yl(n)?v_:p_;return t.test(jn(n))}function M_(n,t){return n==null?void 0:n[t]}function Ri(n,t){var e=M_(n,t);return y_(e)?e:void 0}var S_=Ri(hn,"WeakMap");const Ma=S_;var E_=9007199254740991,T_=/^(?:0|[1-9]\d*)$/;function Ml(n,t){var e=typeof n;return t=t??E_,!!t&&(e=="number"||e!="symbol"&&T_.test(n))&&n>-1&&n%1==0&&n<t}function Sl(n,t){return n===t||n!==n&&t!==t}var w_=9007199254740991;function Ha(n){return typeof n=="number"&&n>-1&&n%1==0&&n<=w_}function A_(n){return n!=null&&Ha(n.length)&&!yl(n)}var b_=Object.prototype;function C_(n){var t=n&&n.constructor,e=typeof t=="function"&&t.prototype||b_;return n===e}function R_(n,t){for(var e=-1,i=Array(n);++e<n;)i[e]=t(e);return i}var P_="[object Arguments]";function ac(n){return wi(n)&&Ci(n)==P_}var El=Object.prototype,L_=El.hasOwnProperty,I_=El.propertyIsEnumerable,D_=ac(function(){return arguments}())?ac:function(n){return wi(n)&&L_.call(n,"callee")&&!I_.call(n,"callee")};const Tl=D_;function U_(){return!1}var wl=typeof an=="object"&&an&&!an.nodeType&&an,oc=wl&&typeof on=="object"&&on&&!on.nodeType&&on,N_=oc&&oc.exports===wl,cc=N_?hn.Buffer:void 0,F_=cc?cc.isBuffer:void 0,O_=F_||U_;const Sa=O_;var B_="[object Arguments]",z_="[object Array]",H_="[object Boolean]",k_="[object Date]",G_="[object Error]",V_="[object Function]",W_="[object Map]",q_="[object Number]",X_="[object Object]",$_="[object RegExp]",Y_="[object Set]",K_="[object String]",j_="[object WeakMap]",Z_="[object ArrayBuffer]",J_="[object DataView]",Q_="[object Float32Array]",tg="[object Float64Array]",eg="[object Int8Array]",ng="[object Int16Array]",ig="[object Int32Array]",rg="[object Uint8Array]",sg="[object Uint8ClampedArray]",ag="[object Uint16Array]",og="[object Uint32Array]",jt={};jt[Q_]=jt[tg]=jt[eg]=jt[ng]=jt[ig]=jt[rg]=jt[sg]=jt[ag]=jt[og]=!0;jt[B_]=jt[z_]=jt[Z_]=jt[H_]=jt[J_]=jt[k_]=jt[G_]=jt[V_]=jt[W_]=jt[q_]=jt[X_]=jt[$_]=jt[Y_]=jt[K_]=jt[j_]=!1;function cg(n){return wi(n)&&Ha(n.length)&&!!jt[Ci(n)]}function lg(n){return function(t){return n(t)}}var Al=typeof an=="object"&&an&&!an.nodeType&&an,ki=Al&&typeof on=="object"&&on&&!on.nodeType&&on,ug=ki&&ki.exports===Al,zs=ug&&gl.process,hg=function(){try{var n=ki&&ki.require&&ki.require("util").types;return n||zs&&zs.binding&&zs.binding("util")}catch{}}();const lc=hg;var uc=lc&&lc.isTypedArray,fg=uc?lg(uc):cg;const bl=fg;var dg=Object.prototype,pg=dg.hasOwnProperty;function mg(n,t){var e=un(n),i=!e&&Tl(n),r=!e&&!i&&Sa(n),s=!e&&!i&&!r&&bl(n),o=e||i||r||s,a=o?R_(n.length,String):[],c=a.length;for(var u in n)(t||pg.call(n,u))&&!(o&&(u=="length"||r&&(u=="offset"||u=="parent")||s&&(u=="buffer"||u=="byteLength"||u=="byteOffset")||Ml(u,c)))&&a.push(u);return a}function _g(n,t){return function(e){return n(t(e))}}var gg=_g(Object.keys,Object);const xg=gg;var vg=Object.prototype,yg=vg.hasOwnProperty;function Mg(n){if(!C_(n))return xg(n);var t=[];for(var e in Object(n))yg.call(n,e)&&e!="constructor"&&t.push(e);return t}function Cl(n){return A_(n)?mg(n):Mg(n)}var Sg=/\.|\[(?:[^[\]]*|(["'])(?:(?!\1)[^\\]|\\.)*?\1)\]/,Eg=/^\w*$/;function ka(n,t){if(un(n))return!1;var e=typeof n;return e=="number"||e=="symbol"||e=="boolean"||n==null||Zr(n)?!0:Eg.test(n)||!Sg.test(n)||t!=null&&n in Object(t)}var Tg=Ri(Object,"create");const Vi=Tg;function wg(){this.__data__=Vi?Vi(null):{},this.size=0}function Ag(n){var t=this.has(n)&&delete this.__data__[n];return this.size-=t?1:0,t}var bg="__lodash_hash_undefined__",Cg=Object.prototype,Rg=Cg.hasOwnProperty;function Pg(n){var t=this.__data__;if(Vi){var e=t[n];return e===bg?void 0:e}return Rg.call(t,n)?t[n]:void 0}var Lg=Object.prototype,Ig=Lg.hasOwnProperty;function Dg(n){var t=this.__data__;return Vi?t[n]!==void 0:Ig.call(t,n)}var Ug="__lodash_hash_undefined__";function Ng(n,t){var e=this.__data__;return this.size+=this.has(n)?0:1,e[n]=Vi&&t===void 0?Ug:t,this}function Yn(n){var t=-1,e=n==null?0:n.length;for(this.clear();++t<e;){var i=n[t];this.set(i[0],i[1])}}Yn.prototype.clear=wg;Yn.prototype.delete=Ag;Yn.prototype.get=Pg;Yn.prototype.has=Dg;Yn.prototype.set=Ng;function Fg(){this.__data__=[],this.size=0}function Jr(n,t){for(var e=n.length;e--;)if(Sl(n[e][0],t))return e;return-1}var Og=Array.prototype,Bg=Og.splice;function zg(n){var t=this.__data__,e=Jr(t,n);if(e<0)return!1;var i=t.length-1;return e==i?t.pop():Bg.call(t,e,1),--this.size,!0}function Hg(n){var t=this.__data__,e=Jr(t,n);return e<0?void 0:t[e][1]}function kg(n){return Jr(this.__data__,n)>-1}function Gg(n,t){var e=this.__data__,i=Jr(e,n);return i<0?(++this.size,e.push([n,t])):e[i][1]=t,this}function fn(n){var t=-1,e=n==null?0:n.length;for(this.clear();++t<e;){var i=n[t];this.set(i[0],i[1])}}fn.prototype.clear=Fg;fn.prototype.delete=zg;fn.prototype.get=Hg;fn.prototype.has=kg;fn.prototype.set=Gg;var Vg=Ri(hn,"Map");const Wi=Vg;function Wg(){this.size=0,this.__data__={hash:new Yn,map:new(Wi||fn),string:new Yn}}function qg(n){var t=typeof n;return t=="string"||t=="number"||t=="symbol"||t=="boolean"?n!=="__proto__":n===null}function Qr(n,t){var e=n.__data__;return qg(t)?e[typeof t=="string"?"string":"hash"]:e.map}function Xg(n){var t=Qr(this,n).delete(n);return this.size-=t?1:0,t}function $g(n){return Qr(this,n).get(n)}function Yg(n){return Qr(this,n).has(n)}function Kg(n,t){var e=Qr(this,n),i=e.size;return e.set(n,t),this.size+=e.size==i?0:1,this}function dn(n){var t=-1,e=n==null?0:n.length;for(this.clear();++t<e;){var i=n[t];this.set(i[0],i[1])}}dn.prototype.clear=Wg;dn.prototype.delete=Xg;dn.prototype.get=$g;dn.prototype.has=Yg;dn.prototype.set=Kg;var jg="Expected a function";function Ga(n,t){if(typeof n!="function"||t!=null&&typeof t!="function")throw new TypeError(jg);var e=function(){var i=arguments,r=t?t.apply(this,i):i[0],s=e.cache;if(s.has(r))return s.get(r);var o=n.apply(this,i);return e.cache=s.set(r,o)||s,o};return e.cache=new(Ga.Cache||dn),e}Ga.Cache=dn;var Zg=500;function Jg(n){var t=Ga(n,function(i){return e.size===Zg&&e.clear(),i}),e=t.cache;return t}var Qg=/[^.[\]]+|\[(?:(-?\d+(?:\.\d+)?)|(["'])((?:(?!\2)[^\\]|\\.)*?)\2)\]|(?=(?:\.|\[\])(?:\.|\[\]|$))/g,tx=/\\(\\)?/g,ex=Jg(function(n){var t=[];return n.charCodeAt(0)===46&&t.push(""),n.replace(Qg,function(e,i,r,s){t.push(r?s.replace(tx,"$1"):i||e)}),t});const nx=ex;function ix(n){return n==null?"":vl(n)}function Rl(n,t){return un(n)?n:ka(n,t)?[n]:nx(ix(n))}var rx=1/0;function ts(n){if(typeof n=="string"||Zr(n))return n;var t=n+"";return t=="0"&&1/n==-rx?"-0":t}function Pl(n,t){t=Rl(t,n);for(var e=0,i=t.length;n!=null&&e<i;)n=n[ts(t[e++])];return e&&e==i?n:void 0}function sx(n,t,e){var i=n==null?void 0:Pl(n,t);return i===void 0?e:i}function ax(n,t){for(var e=-1,i=t.length,r=n.length;++e<i;)n[r+e]=t[e];return n}function ox(){this.__data__=new fn,this.size=0}function cx(n){var t=this.__data__,e=t.delete(n);return this.size=t.size,e}function lx(n){return this.__data__.get(n)}function ux(n){return this.__data__.has(n)}var hx=200;function fx(n,t){var e=this.__data__;if(e instanceof fn){var i=e.__data__;if(!Wi||i.length<hx-1)return i.push([n,t]),this.size=++e.size,this;e=this.__data__=new dn(i)}return e.set(n,t),this.size=e.size,this}function cn(n){var t=this.__data__=new fn(n);this.size=t.size}cn.prototype.clear=ox;cn.prototype.delete=cx;cn.prototype.get=lx;cn.prototype.has=ux;cn.prototype.set=fx;function dx(n,t){for(var e=-1,i=n==null?0:n.length,r=0,s=[];++e<i;){var o=n[e];t(o,e,n)&&(s[r++]=o)}return s}function px(){return[]}var mx=Object.prototype,_x=mx.propertyIsEnumerable,hc=Object.getOwnPropertySymbols,gx=hc?function(n){return n==null?[]:(n=Object(n),dx(hc(n),function(t){return _x.call(n,t)}))}:px;const xx=gx;function vx(n,t,e){var i=t(n);return un(n)?i:ax(i,e(n))}function fc(n){return vx(n,Cl,xx)}var yx=Ri(hn,"DataView");const Ea=yx;var Mx=Ri(hn,"Promise");const Ta=Mx;var Sx=Ri(hn,"Set");const wa=Sx;var dc="[object Map]",Ex="[object Object]",pc="[object Promise]",mc="[object Set]",_c="[object WeakMap]",gc="[object DataView]",Tx=jn(Ea),wx=jn(Wi),Ax=jn(Ta),bx=jn(wa),Cx=jn(Ma),zn=Ci;(Ea&&zn(new Ea(new ArrayBuffer(1)))!=gc||Wi&&zn(new Wi)!=dc||Ta&&zn(Ta.resolve())!=pc||wa&&zn(new wa)!=mc||Ma&&zn(new Ma)!=_c)&&(zn=function(n){var t=Ci(n),e=t==Ex?n.constructor:void 0,i=e?jn(e):"";if(i)switch(i){case Tx:return gc;case wx:return dc;case Ax:return pc;case bx:return mc;case Cx:return _c}return t});const xc=zn;var Rx=hn.Uint8Array;const vc=Rx;var Px="__lodash_hash_undefined__";function Lx(n){return this.__data__.set(n,Px),this}function Ix(n){return this.__data__.has(n)}function Wr(n){var t=-1,e=n==null?0:n.length;for(this.__data__=new dn;++t<e;)this.add(n[t])}Wr.prototype.add=Wr.prototype.push=Lx;Wr.prototype.has=Ix;function Dx(n,t){for(var e=-1,i=n==null?0:n.length;++e<i;)if(t(n[e],e,n))return!0;return!1}function Ux(n,t){return n.has(t)}var Nx=1,Fx=2;function Ll(n,t,e,i,r,s){var o=e&Nx,a=n.length,c=t.length;if(a!=c&&!(o&&c>a))return!1;var u=s.get(n),f=s.get(t);if(u&&f)return u==t&&f==n;var p=-1,m=!0,g=e&Fx?new Wr:void 0;for(s.set(n,t),s.set(t,n);++p<a;){var v=n[p],M=t[p];if(i)var _=o?i(M,v,p,t,n,s):i(v,M,p,n,t,s);if(_!==void 0){if(_)continue;m=!1;break}if(g){if(!Dx(t,function(d,C){if(!Ux(g,C)&&(v===d||r(v,d,e,i,s)))return g.push(C)})){m=!1;break}}else if(!(v===M||r(v,M,e,i,s))){m=!1;break}}return s.delete(n),s.delete(t),m}function Ox(n){var t=-1,e=Array(n.size);return n.forEach(function(i,r){e[++t]=[r,i]}),e}function Bx(n){var t=-1,e=Array(n.size);return n.forEach(function(i){e[++t]=i}),e}var zx=1,Hx=2,kx="[object Boolean]",Gx="[object Date]",Vx="[object Error]",Wx="[object Map]",qx="[object Number]",Xx="[object RegExp]",$x="[object Set]",Yx="[object String]",Kx="[object Symbol]",jx="[object ArrayBuffer]",Zx="[object DataView]",yc=Cn?Cn.prototype:void 0,Hs=yc?yc.valueOf:void 0;function Jx(n,t,e,i,r,s,o){switch(e){case Zx:if(n.byteLength!=t.byteLength||n.byteOffset!=t.byteOffset)return!1;n=n.buffer,t=t.buffer;case jx:return!(n.byteLength!=t.byteLength||!s(new vc(n),new vc(t)));case kx:case Gx:case qx:return Sl(+n,+t);case Vx:return n.name==t.name&&n.message==t.message;case Xx:case Yx:return n==t+"";case Wx:var a=Ox;case $x:var c=i&zx;if(a||(a=Bx),n.size!=t.size&&!c)return!1;var u=o.get(n);if(u)return u==t;i|=Hx,o.set(n,t);var f=Ll(a(n),a(t),i,r,s,o);return o.delete(n),f;case Kx:if(Hs)return Hs.call(n)==Hs.call(t)}return!1}var Qx=1,tv=Object.prototype,ev=tv.hasOwnProperty;function nv(n,t,e,i,r,s){var o=e&Qx,a=fc(n),c=a.length,u=fc(t),f=u.length;if(c!=f&&!o)return!1;for(var p=c;p--;){var m=a[p];if(!(o?m in t:ev.call(t,m)))return!1}var g=s.get(n),v=s.get(t);if(g&&v)return g==t&&v==n;var M=!0;s.set(n,t),s.set(t,n);for(var _=o;++p<c;){m=a[p];var d=n[m],C=t[m];if(i)var w=o?i(C,d,m,t,n,s):i(d,C,m,n,t,s);if(!(w===void 0?d===C||r(d,C,e,i,s):w)){M=!1;break}_||(_=m=="constructor")}if(M&&!_){var b=n.constructor,z=t.constructor;b!=z&&"constructor"in n&&"constructor"in t&&!(typeof b=="function"&&b instanceof b&&typeof z=="function"&&z instanceof z)&&(M=!1)}return s.delete(n),s.delete(t),M}var iv=1,Mc="[object Arguments]",Sc="[object Array]",wr="[object Object]",rv=Object.prototype,Ec=rv.hasOwnProperty;function sv(n,t,e,i,r,s){var o=un(n),a=un(t),c=o?Sc:xc(n),u=a?Sc:xc(t);c=c==Mc?wr:c,u=u==Mc?wr:u;var f=c==wr,p=u==wr,m=c==u;if(m&&Sa(n)){if(!Sa(t))return!1;o=!0,f=!1}if(m&&!f)return s||(s=new cn),o||bl(n)?Ll(n,t,e,i,r,s):Jx(n,t,c,e,i,r,s);if(!(e&iv)){var g=f&&Ec.call(n,"__wrapped__"),v=p&&Ec.call(t,"__wrapped__");if(g||v){var M=g?n.value():n,_=v?t.value():t;return s||(s=new cn),r(M,_,e,i,s)}}return m?(s||(s=new cn),nv(n,t,e,i,r,s)):!1}function Va(n,t,e,i,r){return n===t?!0:n==null||t==null||!wi(n)&&!wi(t)?n!==n&&t!==t:sv(n,t,e,i,Va,r)}var av=1,ov=2;function cv(n,t,e,i){var r=e.length,s=r,o=!i;if(n==null)return!s;for(n=Object(n);r--;){var a=e[r];if(o&&a[2]?a[1]!==n[a[0]]:!(a[0]in n))return!1}for(;++r<s;){a=e[r];var c=a[0],u=n[c],f=a[1];if(o&&a[2]){if(u===void 0&&!(c in n))return!1}else{var p=new cn;if(i)var m=i(u,f,c,n,t,p);if(!(m===void 0?Va(f,u,av|ov,i,p):m))return!1}}return!0}function Il(n){return n===n&&!za(n)}function lv(n){for(var t=Cl(n),e=t.length;e--;){var i=t[e],r=n[i];t[e]=[i,r,Il(r)]}return t}function Dl(n,t){return function(e){return e==null?!1:e[n]===t&&(t!==void 0||n in Object(e))}}function uv(n){var t=lv(n);return t.length==1&&t[0][2]?Dl(t[0][0],t[0][1]):function(e){return e===n||cv(e,n,t)}}function hv(n,t){return n!=null&&t in Object(n)}function fv(n,t,e){t=Rl(t,n);for(var i=-1,r=t.length,s=!1;++i<r;){var o=ts(t[i]);if(!(s=n!=null&&e(n,o)))break;n=n[o]}return s||++i!=r?s:(r=n==null?0:n.length,!!r&&Ha(r)&&Ml(o,r)&&(un(n)||Tl(n)))}function dv(n,t){return n!=null&&fv(n,t,hv)}var pv=1,mv=2;function _v(n,t){return ka(n)&&Il(t)?Dl(ts(n),t):function(e){var i=sx(e,n);return i===void 0&&i===t?dv(e,n):Va(t,i,pv|mv)}}function gv(n){return function(t){return t==null?void 0:t[n]}}function xv(n){return function(t){return Pl(t,n)}}function vv(n){return ka(n)?gv(ts(n)):xv(n)}function yv(n){return typeof n=="function"?n:n==null?r_:typeof n=="object"?un(n)?_v(n[0],n[1]):uv(n):vv(n)}function Mv(n,t){return n<t}function Sv(n,t,e){for(var i=-1,r=n.length;++i<r;){var s=n[i],o=t(s);if(o!=null&&(a===void 0?o===o&&!Zr(o):e(o,a)))var a=o,c=s}return c}function Ev(n,t){return n&&n.length?Sv(n,yv(t),Mv):void 0}const _e={width:480,height:360},ze={offsetSeconds:-1.05,videoId:I0.vm109,syncWithVideo:!0},Tc=.02,Ye=D0(ze.videoId,_e),me={isPlaying:!0,resumedTime:0,lastFrameRendered:-5e3,lastFrameChecked:-5e3};async function Tv(){document.getElementById("mainContainer").style.width=`${_e.width*2}px`;const n=Av(),t=await L0(),e={textFields:n,analyzeResult:t};let i;const r=s=>{i==null||i.dispose(),e.analyzeResult=s,i=wv(e)};k0(ze,Ye,r),H0(ze,Ye,r),r(t)}function wv(n){const{textFields:t,analyzeResult:e}=n;_l(ze),t.nameText.textContent=e.playerName,t.timeText.textContent=wc(e,0);const i=document.getElementById("playPauseButton"),r=document.getElementById("backButton"),s=document.getElementById("forwardButton"),o=document.getElementById("playerRange"),a=document.getElementById("frameInfo");document.getElementsByClassName("keyLeft")[0],document.getElementsByClassName("keyRight")[0],document.getElementsByClassName("keyShift")[0],i.onclick=()=>{var P,I;if(me.isPlaying){if(me.isPlaying=!1,ze.syncWithVideo){(P=Ye.videoTarget)==null||P.pauseVideo();const{expectedFrame:F}=Ul();z(0,Math.floor(F))}}else me.resumedTime=performance.now()-10*me.lastFrameRendered,me.isPlaying=!0,ze.syncWithVideo&&((I=Ye.videoTarget)==null||I.playVideo());i.innerHTML=`<i data-lucide="${me.isPlaying?"pause":"play"}"></i>`,lucide.createIcons()},s.onclick=()=>{z(0,(me.lastFrameRendered+1)%u.length)},r.onclick=()=>{z(0,(me.lastFrameRendered-1+u.length)%u.length)};const c=document.getElementById("preText"),u=e.coords.rows.slice(0,-1);document.getElementById("headerInfo").innerHTML=Lv(e),document.getElementById("scoreInfo").innerHTML=Uv(e),lucide.createIcons(),o.min="0",o.max=u.length.toString();const f=new Ie(75,window.innerWidth/window.innerHeight,.01,1e3),p=new r0,m=document.getElementById("keyControls");m.style.bottom="0",m.style.right=`${_e.width}px`;const g=O0(f),v=new N0(u.map(P=>Fl(P)),{accelerationThreshold:.05}),M=v.createSlopeMesh({playerYOffset:.4}),_=v.createJumpMarkers({});p.add(M),_.forEach(P=>p.add(P));const d=new l0(16777215,1);d.castShadow=!0,p.add(d);const C=u[0],w=B0();Ac(w,C),p.add(w);const b=new i0({antialias:!0});b.setSize(_e.width,_e.height),b.setAnimationLoop(P=>z(P)),b.shadowMap.enabled=!0,document.getElementById("container").prepend(b.domElement),o.addEventListener("click",P=>{var A;const I=P.offsetX/o.offsetWidth,F=Math.floor(I*u.length);me.resumedTime=performance.now()-10*F,ze.syncWithVideo&&((A=Ye.videoTarget)==null||A.seekTo(F/100+ze.offsetSeconds,!0)),me.lastFrameChecked=F,z(0,F)});function z(P,I=void 0){if(I===void 0&&!me.isPlaying)return;const F=I??Math.floor((P-me.resumedTime)/10)%u.length;if(F<0||F>u.length-1)return;const A=u[F];Cv(w,A),Ac(w,A);const E=ec(u,F);F>0&&g.updateCamera(u[F],u[F-1],w,E);const N=Nl(F),$=`x: ${w.position.x}
y: ${w.position.y}
z: ${w.position.z}
accel(y): ${U0(u,F).toFixed(1)}
drift(s): ${N?(N.actualSeconds-N.expectedSeconds).toFixed(3):"N/A"}
rotation3x3:
[
  ${A.rotation3x3[0].map(W=>W.toFixed(3)).join(", ")}
  ${A.rotation3x3[1].map(W=>W.toFixed(3)).join(", ")}
  ${A.rotation3x3[2].map(W=>W.toFixed(3)).join(", ")}
]
${bv(A.raw)}`;o.value=F.toString(),a.innerHTML=`Frame: ${F} / ${u.length}`,t.timeText.textContent=wc(e,F),t.speedText1.textContent=`${Math.floor(ec(u,F)).toFixed(0).padStart(3,"0")} km/h`,t.speedText2.textContent=t.speedText1.textContent.replace(" ",""),$!==c.innerHTML&&(c.innerHTML=$),Ye.videoTarget!==void 0&&Math.abs(me.lastFrameChecked-F)>100&&(Pv(F),me.lastFrameChecked=F),me.lastFrameRendered=F,b.render(p,f)}return{dispose:()=>{console.log("DISPOSE!!!!"),b.domElement.remove(),b.dispose()}}}function Av(){const n=.07777777777777778*_e.height,t=124/540*_e.height,e=152/540*_e.height,i=22/720*_e.width,r=20/720*_e.width,s=22/540*_e.height,o=10/540*_e.height,a=Ar("",{top:`${n}px`,right:`${_e.width+i}px`,fontSize:`${s}px`}),c=Ar("",{top:`${n+i}px`,right:`${_e.width+i}px`,fontSize:`${s}px`}),u=Ar("000km/h",{top:`${e}px`,right:`${_e.width+i}px`,fontSize:`${s}px`}),f=Ar("000km/h",{top:`${t}px`,left:`${_e.width+r}px`,backgroundColor:"black",letterSpacing:"1px",padding:`0 ${o}px`,fontSize:`${s}px`});return{nameText:a,timeText:c,speedText1:u,speedText2:f}}function Ar(n,t){const e=document.createElement("div");return e.style.position="absolute",e.style.color="white",e.style.fontSize="22px",Object.assign(e.style,t),e.textContent=n,document.getElementById("container").appendChild(e),e}function wc(n,t){let e=Math.max(0,10*t-(n.lagBeforeStartMs+n.startMs));return e=Math.min(e,n.displayedMs),Mn(e)}function Mn(n){return new Date(n).toISOString().slice(14,22).replace(".",":")}function bv(n){const t=new Array;for(let a=0;a<n.length;a+=8)t.push(zi(n.slice(a,a+8)));const e=7,i=Math.ceil(t.length/e);let r=t.map(a=>a.toFixed(3));const s=Math.max(10,...r.map(a=>a.length));r=r.map(a=>a.padStart(s," ")).map((a,c)=>c>=11&&c<=22?`<mark>${a}</mark>`:a);const o=new Array;for(let a=0;a<t.length;a+=i)o.push(r.slice(a,a+i).join(", "));return`frame: ${n.length/2} bytes, ${t.length} 4-byte floats:
`+o.join(`
`)}function Cv(n,t){const e=t.rotation3x3,i=new Qt().set(e[0][0],e[0][1],e[0][2],0,e[1][0],e[1][1],e[1][2],0,e[2][0],e[2][1],e[2][2],0,0,0,0,1),r=new Ge().setFromRotationMatrix(i,"XYZ");r.x=-r.x,r.y=-r.y;const s=new Qt().makeRotationFromEuler(r);n.setRotationFromMatrix(s)}let Rv=0;function Pv(n){if(Ye.videoTarget===void 0||!ze.syncWithVideo)return;const t=Nl(n);t&&Math.abs(t.actualSeconds-t.expectedSeconds)>Tc&&(console.log(`Synced video to replay, precision: ${Tc}, times: ${++Rv}.`),Ul(t.actualSeconds))}function Ul(n){n=n??Ye.videoTarget.getCurrentTime();const t=(n-ze.offsetSeconds)*100;return me.resumedTime=performance.now()-10*t,{expectedFrame:t}}function Nl(n){const t=ze.offsetSeconds+n/100;if(!Ye.videoTarget)return;const e=Ye.videoTarget.getCurrentTime();return{expectedSeconds:t,actualSeconds:e}}function Ac(n,t){const e=Fl(t);n.position.x=e.x,n.position.y=e.y,n.position.z=e.z}function Fl(n){return{x:-n.x,y:-n.y,z:n.z}}function Lv(n){var p;const{timingDataFromHeader:t}=n,e=t.totalTimeToFinishMs!==0,i=t.checkpoint1TotalMs!==0?Mn(n.checkpoint1Ms-10):"N/A (No CP1 data recorded)",r=Ol(n),s=r?Mn(r.frame2*10-n.timingDataFromHeader.crossStartPlusStartDelayMs):n.trackName===$n?"N/A (Unknown Track)":"N/A (Could not find CP2)",o=n.trackName!==$n?(p=n.trackScoreData)==null?void 0:p.levelScores[0]:void 0,a=e?Mn(n.displayedMs):"N/A (Did not finish)",c=e?Mn(n.totalMs):"N/A (Did not finish)";console.log(n);const u=[o==null?void 0:o.scoreData.firstValidCheckPoint1Collision,r,o==null?void 0:o.scoreData.firstValidFinishPointCollision],f=Iv(n);return`Player: ${n.playerName}
Track : ${n.trackName}
CP1   : ${i}${br(u,o==null?void 0:o.scoreData.firstValidCheckPoint1Collision,"Distance to Check_Point_1, must be ≤90m")}
CP2   : ${s}${br(u,r,"Distance to Check_Point_2, must be ≤90m")}
Time  : ${a}${br(u,o==null?void 0:o.scoreData.firstValidFinishPointCollision,"Distance to Finish_Point, must be ≤90m")}

Legitimate Run? : ${f?`<span style='color:red'>No (${f})</span>`:"Yes"}
Total Time      : ${c}
Lag before start: ${Mn(n.lagBeforeStartMs)}${br([],o==null?void 0:o.scoreData.firstValidStartPointCollision,"Distance to Start_Point, must be ≤90m")}
Leg after finish: ${Mn(n.lagAfterFinishMs)}
Recording time  : ${Mn(n.recordingMs)}
`}function Iv(n){var e;if(n.trackName===$n)return"UnknownTrack";const t=(e=n.trackScoreData)==null?void 0:e.levelScores[0].score;if(t!==tc)return`Score ${t} is not max score of ${tc}`}function Ol(n){var c,u;if(n.trackName===$n)return;const t=(c=n.trackScoreData)==null?void 0:c.levelScores[0];if(!t)return;const{scoreData:e}=t,i=(u=t.scoreData.validCheckPoint2Collisions)==null?void 0:u.at(0);if(i)return i;const{firstValidStartPointCollision:r,firstValidCheckPoint1Collision:s,firstValidFinishPointCollision:o}=e,a=t.scoreData.allCheckPoint2Collisions.filter(f=>!r||r.frame2<=f.frame1).filter(f=>!s||s.frame2<=f.frame1).filter(f=>!o||f.frame2<=o.frame2);return Ev(a,f=>f.distance)}function br(n,t,e){if(!t)return"";const i=Math.max(...n.map(s=>(s==null?void 0:s.distance.toFixed(2).length)??0));return` <span style='${t.distance<=Ba?"color:green":"color:darkorange"}'>(${t.distance.toFixed(2).padStart(i," ")}m)</span>${Dv(e)}`}function Dv(n){return`<span data-tooltip='${n}'> <i data-lucide="info" style='width:12px; height:12px; vertical-align:middle'></i></span>`}function Uv(n){var o,a;const t=(o=n.trackScoreData)==null?void 0:o.levelScores,i=`
  <table>
    <thead>
      <th>Score</th>
      <th style='text-align: left'>Track</th>
    </thead>
    <tbody>
      ${[...new Set((a=n.trackScoreData)==null?void 0:a.levelScores.map(c=>c.score))].sort((c,u)=>u-c).map(c=>`<tr>
            <td>${c}</td>
            <td>${t==null?void 0:t.filter(u=>u.score===c).map(u=>u.name).sort().join(", ")}</td>
          </tr>`).join("")}
    </tbody>
  </table>`,r=Fv(n),s=Nv(n);return i+r+s}function Nv(n){var s;if(n.trackName===$n)return"";const t=(s=n.trackScoreData)==null?void 0:s.levelScores[0];if((t==null?void 0:t.score)!==5)return"";const e=Ol(n),i=o=>(o/1e3).toFixed(2);return`
    <div style='margin-top:15px'>High Times</div>
    <pre>${[`"${n.playerName}"`,i(n.checkpoint1Ms),i(e.frame2*10-n.timingDataFromHeader.crossStartPlusStartDelayMs),i(n.displayedMs),"2025/1/1",0].join(",")},</pre>
`}function Fv(n){var s;if(n.trackName===$n)return"";const t=(s=n.trackScoreData)==null?void 0:s.levelScores[0];if(!t)return"";const e=t.scoreData,i=A0(e);return`

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
  <div style='font-size:10px'>Explanation of the track logic is in <a target='_blank' href='https://github.com/domsleee/SS-Dat-Info/wiki/Dat%E2%80%90Info#determining-trackname-of-a-dat-file'>the wiki</a>.</div>`}Tv()});export default Ov();
