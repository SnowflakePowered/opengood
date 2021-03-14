# OpenGood

OpenGood is a freely-available distribution of XML DAT files for ROMs that are listed in the April 3rd, 2016 distribution of the GoodTools suite of ROM auditing applications.

It is made available here for historical preservation purposes. These DATs are sourced from highly outdated collections of ROMs.

To remain openly distributable, OpenGood was compiled using publicly available data, and was not reverse engineered from the GoodTools suite. As a result, some DAT files may be incomplete. The corresponding have and miss lists are available under the [havelists](/havelists) directory. This also means that OpenGood DATs include MD5 hash information, in addition to the CRC32 and SHA1 hashes that GoodTools provides.

DAT files are available under the [dats](/dats) directory and are in Standard Logqix DAT format. They are versioned with the version number of GoodTools used to verify the completeness of the ROM set. Because OpenGood is a historical preservation project, updates will be very rare; in the case new information is available, the corresponding DAT file will be updated accordingly without a version update.


## List of DATs available

OpenGood covers all 35 platforms supported by the April 3rd, 2016 distribution of the GoodTools suite.

### Complete DATs

These DATs are 'complete' in that 0 ROMs are reported missing by their corresponding GoodTool. 

* Open5200 (Atari 5200) &mdash; 284 of 284
* OpenChaF (Fairchild Channel F) &mdash; 88 of 88
* OpenCol (ColecoVision) &mdash; 459 of 459
* OpenCPC (Amstrad CPC) &mdash; 27103 of 27103
* OpenGBA (Nintendo Game Boy Advance) &mdash; 37145 of 37145
* OpenGBx (Nintendo Game Boy/Game Boy Color) &mdash; 9333 of 9333
* OpenGCOM (Tiger Game.com) &mdash; 40 of 40
* OpenGen (Sega Mega Drive/Genesis/32X) &mdash; 8563 of 8563
* OpenGG (Sega Game Gear) &mdash; 1020 of 1020
* OpenINTV (Mattel Intellivision) &mdash; 238 of 238
* OpenJag (Atari Jaguar) &mdash; 139 of 139
* OpenLynx (Atari Lynx) &mdash; 300 of 300
* OpenMO5 (Thomson MO5) &mdash; 391 of 391
* OpenMSX1 (Microsoft MSX) &mdash; 589 of 589
* OpenMSX2 (Microsoft MSX2) &mdash; 166 of 166
* OpenMTX (Memotech MTX) &mdash; 93 of 93
* OpenNGPx (SNK Neo Geo PocketNeo Geo Pocket Color) &mdash; 452 of 452
* OpenOric (Tangerine Oric) &mdash; 1236 of 1236
* OpenPCE (NEC PC Engine/TurboGrafx-16) &mdash; 1565 of 1565
* OpenSPC (Super Nintendo Entertainment System (SPC Music)) &mdash; 57154 of 57154
* OpenSV (Watara Supervision) &mdash; 69 of 69
* OpenVBoy (Nintendo Virtual Boy) &mdash; 179 of 179
* OpenVECT (GCE Vectrex) &mdash; 287 of 287

### Incomplete DATs

These DATs are reported by their corresponding GoodTool to have some missing ROMs when the hash information was compiled.

* Open2600 (Atari 2600) &mdash; 7,204 of 7216
* Open7800 (Atari 7800) &mdash; 925 of 932
* OpenCoCo (Tandy Color Computer) &mdash; 168 of 184
* OpenN64 (Nintendo 64/Nintendo 64DD) &mdash; 5265 of 5282
* OpenNES (Nintendo Entertainment System) &mdash; 22094 of 22096
* OpenPico (Sega Pico) &mdash;- 173 of 178
* OpenSNES (Super Nintendo Entertainment System) &mdash; 24485 of 24508
* OpenWSx (Bandai WonderSwan/Bandai WonderSwan Color) &mdash; 375 of 377

## Split DATs

GoodTools sometimes grouped together different but similar systems into one DAT file. OpenGood also provides platform-separated DAT files where applicable. For historical purposes, the DAT file containing the original set of ROMs remains available.

* OpenGBA
  * OpenGBA.GBA (Nintendo Game Boy Advance)
  * OpenGBA.MB (Nintendo Game Boy Advance (Multiboot))
  * OpenGBA.E+ (Nintendo e-Reader)
* OpenGBx
  * OpenGBx.GB (Nintendo Game Boy)
  * OpenGBx.GBC (Nintendo Game Boy Color)
* OpenGen
  * OpenGen.Gen (Sega Mega Drive/Genesis)
  * OpenGen.32X (Sega 32X)
* OpenWSx
  * OpenWSx.WS (Bandai WonderSwan)
  * OpenWSx.WSC (Bandai WonderSwan Color)
* OpenN64
  * OpenN64.N64 (Nintendo 64)
  * OpenN64.64DD (Nintendo 64 DD)
* OpenNGPx
  * OpenNGPx.NGP (SNK Neo Geo Pocket)
  * OpenNGPx.NGC (SNK Neo Geo Pocket Color)

## Legal

OpenGood is made freely available under the [Creative Commons CC0 License](LICENSE.md). 

The data available as part of OpenGood **was not extracted from the GoodTools suite executables**. The April 3rd, 2016 distribution of the GoodTools suite was simply used to verify the completeness of the source data, from which a standard Logiqx XML format DAT was created using ClrMamePro.

Hence, OpenGood DATs do not run afoul of the reverse-engineering prohibitions (clause 3) of the GoodTools license and may be freely distributed.

## Freqently Asked Questions

### Will there be Parent-Clone DATs?

OpenGood is made available only in Standard Logiqx DAT format. 

OpenGood is primarily available for historical preservation purposes because of its outdated romset information. As well, Parent-Clone DATs are not widely available (out of the three most prolific calatoguing organizations, only No-Intro provides P/C XMLs) and their utility is of limited use. For 1G1R, a tool such as [Retool](https://github.com/unexpectedpanda/retool) can be used. In addition, for most cases, [shiratsu-lib](https://github.com/SnowflakePowered/shiratsu/tree/master/src/shiratsu-lib) provides enough utility in its API to emit Parent-Clone relationships for Standard DATs via name grouping and regular expressions, especially in conjunction with [GoodMerge XMDBs](https://sourceforge.net/projects/goodmerge/files/GoodMerge%20XMDBs/).

### Can I add entries to the DATs?

OpenGood's specific goal is to archive as close as possible without running afoul of the GoodTools usage license, the filenames and hashes of files that verify with the April 3rd, 2016 release of the GoodTools suite. Therefore, we will not be accepting manual contributions. If somehow you are able to improve on the missing list for an incomplete system, please get in touch by [filing an issue](https://github.com/SnowflakePowered/opengood/issues).


### Why should anyone still use GoodTools?

OpenGood does not encourage the continued use of GoodTools. No-Intro, Redump, and TOSEC have done a much better job in terms of preservation and have essentially obsoleted GoodTools. However, GoodTools left a lasting legacy on the ROM preservation scene, and even 5 years since the last available release, the database information within GoodTools remains opaque and not easily distributable. OpenGood aims to preserve this database in a freely distributable and standard format, regardless of the practicalities of its use.