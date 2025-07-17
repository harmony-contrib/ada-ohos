# @ohos-rs/ada

![Platform](https://img.shields.io/badge/platform-arm64/arm/x86\_64-blue) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

The binding of [ada-url](https://github.com/ada-url/ada) for OpenHarmony/HarmonyNext.

If you want to use `ndk` to build this project, you need to pin `ada` with `3.1.0` or lower. Otherwise you need to use `cargo-zigbuild` build it. See [example](https://github.com/ohos-rs/zig-setup).

## Install

You can use ohpm to install it directly.

```shell
ohpm install @ohos-rs/ada
```

## Usage

```ts
import { Ada } from "@ohos-rs/ada";

const url = Ada.parse("https://www.qq.com");

const host = url.host;
```

## Api

```ts
export declare const enum HostType {
  Domain = 0,
  IPV4 = 1,
  IPV6 = 2
}

/** SchemeType Enum which if copied from ada-url */
export declare const enum SchemeType {
  Http = 0,
  NotSpecial = 1,
  Https = 2,
  Ws = 3,
  Ftp = 4,
  Wss = 5,
  File = 6
}

/** ada-url wrapper support parse and canParse method. */
export declare class Ada {
  /** parse url */
  static parse(url: string, base?: string | undefined | null): Url
  /** check url can be parsed */
  static canParse(url: string, base?: string | undefined | null): boolean
  /** create url search params */
  static urlSearchParams(params: string): UrlSearchParams
}

/** url instance wrapper, can directly get host,protocol,etc. */
export declare class Url {
  get host(): string
  set host(host: string)
  get hash(): string
  set hash(hash: string)
  get hostname(): string
  set hostname(hosttname: string)
  get pathname(): string
  set pathname(pathtname: string)
  get search(): string
  set search(search: string)
  get protocol(): string
  set protocol(protocol: string)
  get href(): string
  set href(href: string)
  get username(): string
  set username(username: string)
  get password(): string
  set password(password: string)
  get port(): string
  set port(port: string)
  get origin(): string
  get schemaType(): SchemeType
  get hostType(): HostType
}

/** url search params instance wrapper, can directly get/set search params. */
export declare class UrlSearchParams {
  /** get search params by key */
  get(key: string): string | null
  /** set search params by key */
  set(key: string, value: string): void
  /** append search params by key */
  append(key: string, value: string): void
  /** search params has key and value */
  contains(key: string, value: string): boolean
  /** delete search params by key and value */
  remove(key: string, value: string): void
  /** delete all search params by key */
  removeKey(key: string): void
  /** search params has key */
  containsKey(key: string): boolean
  /** search params is empty or not */
  isEmpty(): boolean
  /** get all search params by key */
  getAll(key: string): Array<string>
  /** get all keys length */
  len(): number
  /** sort search params */
  sort(): void
  /** get all keys */
  keys(): Array<string>
  /** get all values */
  values(): Array<string>
}
```

## Performance

Based on some informal tests, it's approximately **4x** times faster than the built-in `@ohos.url`.

| package      | Avg | Max  | Min | unit |
|--------------|-----|------|-----|------|
| @ohos-rs/ada | 57  | 352  | 31  | us   |
| @ohos.url    | 196 | 1405 | 99  | us   |
