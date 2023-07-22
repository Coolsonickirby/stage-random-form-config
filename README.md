# Usage instructions
1. Place the plugin in either
- `sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins`
- `sd:/ultimate/mods/[Mod Folder]/plugin.nro` (rename it to that)
2. Create/Place the config file in `sd:/ultimate/stage-config.toml`

## Config making/editing
The basic structure of the config is like this:
```toml
[stage_name]
valid_types = ["Normal", "Battle", "End"] # Types go here and get randomly picked
hazard = "Default" # Whether hazard is on, off, default, or random for normal/bf/fd
```
As for the values themselves:
```
valid_types => Array of stage kind enums
Enum Values:
Normal: the normal version of the stage
Battle: the battlefield version of the stage
End: the fd version of the stage
```
```
hazard => enum for the state of the hazard for the stage
Enum Values:
On: hazards are on
Off: hazards are off
Random: 50% chance whether hazards are on or off
Default: whatever the ruleset is set to
```


List of valid stage names (use either the hash or the name itself, not both):
```
0xB51B7F6D50 => battlefield
0xF5BB910AA0 => end_battlefield
0xD906F49D30 => battlefield_l
0x300FC33B10 => end
0xA0F8A80C50 => battle_end
0xE32131E3B0 => mario_castle64
0x1282224831 => end_mario_castle64
0x1543B0A783 => battle_mario_castle64
0x945364EDF0 => dk_jungle
0xDD6E6F2D70 => end_dk_jungle
0x10C7EACBB1 => battle_dk_jungle
0xCE05AE4F40 => zelda_hyrule
0x1020EA86BF => end_zelda_hyrule
0x136B82F0AD => battle_zelda_hyrule
0xBCAA686480 => yoshi_story
0xFC0A860370 => end_yoshi_story
0x12AC0A3512 => battle_yoshi_story
0xE45FB6E570 => kirby_pupupu64
0x12F5CA385D => end_kirby_pupupu64
0x153458D7EF => battle_kirby_pupupu64
0xDF902E8A00 => poke_yamabuki
0x1118CCC0DA => end_poke_yamabuki
0x14EB3ED9E4 => battle_poke_yamabuki
0xCAFF252600 => mario_past64
0x106F42302B => end_mario_past64
0x13242A4639 => battle_mario_past64
0xEC5F42C560 => mario_castledx
0x1275C57A5C => end_mario_castledx
0x15B45795EE => battle_mario_castledx
0xDE1EFB31A0 => mario_rainbow
0x1100219B60 => end_mario_rainbow
0x14F3D3825E => battle_mario_rainbow
0xCFAF08AF60 => dk_waterfall
0x103A40E8BD => end_dk_waterfall
0x1371289EAF => battle_dk_waterfall
0x822D7A5470 => dk_lodge
0xCFB9430CA0 => end_dk_lodge
0xFF6A60A110 => battle_dk_lodge
0xE03274FC10 => zelda_greatbay
0x12B31619CB => end_zelda_greatbay
0x157284F679 => battle_zelda_greatbay
0xC3FEFD9B50 => zelda_temple
0x10FF5FBBFE => end_zelda_temple
0x13B437CDEC => battle_zelda_temple
0xF3BCDC5F40 => yoshi_cartboard
0x13DBA81DBC => end_yoshi_cartboard
0x16FE065DF3 => battle_yoshi_cartboard
0xC5A5ADC840 => yoshi_yoster
0x109AEABECF => end_yoshi_yoster
0x13D182C8DD => battle_yoshi_yoster
0xE59ACC7D10 => kirby_fountain
0x12E99D91DB => end_kirby_fountain
0x15280F7E69 => battle_kirby_fountain
0xC0952ADAE0 => kirby_greens
0x10C9E2CFE5 => end_kirby_greens
0x13828AB9F7 => battle_kirby_greens
0xCA196E22F0 => fox_corneria
0x1061268064 => end_fox_corneria
0x132A4EF676 => battle_fox_corneria
0x90BA1E4000 => fox_venom
0xD987158080 => end_fox_venom
0x10897D616E => battle_fox_venom
0xF462513870 => metroid_zebesdx
0x13A640CBCF => end_metroid_zebesdx
0x1683EE8B80 => battle_metroid_zebesdx
0xC9A857A660 => mother_onett
0x105A35182D => end_mother_onett
0x13115D6E3F => battle_mother_onett
0xC542E24520 => poke_stadium
0x10949E4619 => end_poke_stadium
0x13DFF6300B => battle_poke_stadium
0xD1C634EAC0 => metroid_kraid
0x11FDAD66D6 => end_metroid_kraid
0x140E5F7FE8 => battle_metroid_kraid
0xF282F9C0F0 => mother_fourside
0x13C84A4447 => end_mother_fourside
0x16EDE40408 => battle_mother_fourside
0xD144E57FF0 => fzero_bigblue
0x11F5807F85 => end_fzero_bigblue
0x14067266BB => battle_fzero_bigblue
0xD684EC7120 => mario_pastusa
0x118980EF68 => end_mario_pastusa
0x147A72F656 => battle_mario_pastusa
0xC4FDAC6550 => mario_dolpic
0x108F6AA41E => end_mario_dolpic
0x13C402D20C => battle_mario_dolpic
0xCF92BEE540 => yoshi_island
0x10399B8C1F => end_yoshi_island
0x1372F3FA0D => battle_yoshi_island
0xF02BA9E050 => fox_lylatcruise
0x13E2DF464D => end_fox_lylatcruise
0x16C7710602 => battle_fox_lylatcruise
0xB455202470 => zelda_oldin
0xF4F5CE4380 => end_zelda_oldin
0x1223FEB11D => battle_zelda_oldin
0xEC1B310900 => animal_village
0x127182469A => end_animal_village
0x15B010A928 => battle_animal_village
0xF4CC19D9F0 => icarus_skyworld
0x13ACA445D7 => end_icarus_skyworld
0x16890A0598 => battle_icarus_skyworld
0x8F6D4ECDF0 => fe_siege
0xC2F9779520 => end_fe_siege
0xF22A543890 => battle_fe_siege
0xC902219A80 => wario_madein
0x1050927BE3 => end_wario_madein
0x131BFA0DF1 => battle_wario_madein
0xD9FE4A0F10 => poke_stadium2
0x117E2A888B => end_poke_stadium2
0x148DD891B5 => battle_poke_stadium2
0xD5F39123A0 => kirby_halberd
0x11BEF73A40 => end_kirby_halberd
0x144D05237E => battle_kirby_halberd
0xE83B5E8AF0 => mg_shadowmoses
0x123384BEA5 => end_mg_shadowmoses
0x15F2165117 => battle_mg_shadowmoses
0xE4C469B8E0 => mother_newpork
0x12FC77CD84 => end_mother_newpork
0x153DE52236 => battle_mother_newpork
0x72E068AAD0 => ice_top
0xBBBC9DB440 => end_ice_top
0xEE4993ECE0 => battle_ice_top
0xF4CA3501D0 => metroid_norfair
0x13ACC68855 => end_metroid_norfair
0x168968C81A => battle_metroid_norfair
0xD77E6A9AB0 => kart_circuitx
0x11962881D1 => end_kart_circuitx
0x1465DA98EF => battle_kart_circuitx
0xF8AD13DAA0 => metroid_orpheon
0x136AB4E5E2 => end_metroid_orpheon
0x164F1AA5AD => battle_metroid_orpheon
0xD6E7A250D0 => pikmin_planet
0x118FB40D77 => end_pikmin_planet
0x147C461449 => battle_pikmin_planet
0xB6647302C0 => mario_pastx
0xF6C49D6530 => end_mario_pastx
0x1200EB8376 => battle_mario_pastx
0xE4803D96B0 => fzero_porttown
0x12F8328F61 => end_fzero_porttown
0x1539A060D3 => battle_fzero_porttown
0xC7A849D0C0 => luigimansion
0x10BA34FF47 => end_luigimansion
0x13F15C8955 => battle_luigimansion
0xD3B9741D80 => zelda_pirates
0x11DA5969A2 => end_zelda_pirates
0x1429AB709C => battle_zelda_pirates
0xB3E0C39660 => poke_tengam
0xF3402DF190 => end_poke_tengam
0x1258A08A3C => battle_poke_tengam
0x4023CECAE0 => _75m
0x739D630290 => end_75m
0xA402146200 => battle_75m
0x9F539792C0 => mariobros
0xD66E9C5240 => end_mariobros
0x1077E5FC42 => battle_mariobros
0x8977EF6020 => plankton
0xC4E3D638F0 => end_plankton
0xF430F59540 => battle_plankton
0xF9F664DFA0 => sonic_greenhill
0x137F0395B2 => end_sonic_greenhill
0x165AADD5FD => battle_sonic_greenhill
0xCE54005280 => mario_3dland
0x1025F06763 => end_mario_3dland
0x136E981171 => battle_mario_3dland
0xEF054B4C20 => mario_newbros2
0x124065E2C8 => end_mario_newbros2
0x1581F70D7A => battle_mario_newbros2
0xBD70D447B0 => mario_paper
0xFDD03A2040 => end_mario_paper
0x12B1A1F721 => battle_mario_paper
0xC9C104FB40 => zelda_gerudo
0x105CA02DFF => end_zelda_gerudo
0x1317C85BED => battle_zelda_gerudo
0xBE46F46930 => zelda_train
0xFEE61A0EC0 => end_zelda_train
0x1282C3F5C9 => battle_zelda_train
0xAD55E2A3A0 => poke_unova
0xEDB1672B40 => end_poke_unova
0x117FD6BAE0 => battle_poke_unova
0xAA7E6966B0 => poke_tower
0xEA9AECEE50 => end_poke_tower
0x110D6E06B1 => battle_poke_tower
0x8BD4387F30 => fe_arena
0xC6400127E0 => end_fe_arena
0xF693228A50 => battle_fe_arena
0xFFBE312820 => icarus_uprising
0x131B86CACA => end_icarus_uprising
0x163E288A85 => battle_icarus_uprising
0xD13FEAA360 => animal_island
0x11F230824C => end_animal_island
0x1401C29B72 => battle_animal_island
0xA87BA584D0 => punchoutsb
0xE89F200C30 => end_punchoutsb
0x112D32C897 => battle_punchoutsb
0x9BAFC2BCC0 => punchoutw
0xD292C97C40 => end_punchoutw
0x103820AEA2 => battle_punchoutw
0x9AEB6FA9F0 => xeno_gaur
0xD3D6646970 => end_xeno_gaur
0x102C6A7FF1 => battle_xeno_gaur
0xA6B664B910 => nintendogs
0xE652E131F0 => end_nintendogs
0x11C1EEDB4B => battle_nintendogs
0xADF1FE2450 => streetpass
0xED157BACB0 => end_streetpass
0x117597729F => battle_streetpass
0x96DBDA80B0 => tomodachi
0xDFE6D14030 => end_tomodachi
0x10EF612D65 => battle_tomodachi
0xAB073AAEA0 => pictochat2
0xEBE3BF2640 => end_pictochat2
0x111AFB3A30 => battle_pictochat2
0x902C639A80 => rock_wily
0xD911685A00 => end_rock_wily
0x10801ABCC6 => battle_rock_wily
0xFB20130580 => mother_magicant
0x135264E810 => end_mother_magicant
0x1677CAA85F => battle_mother_magicant
0xD90E3659A0 => kirby_gameboy
0x11712D4DE0 => end_kirby_gameboy
0x1482DF54DE => battle_kirby_gameboy
0xCFE29B8340 => balloonfight
0x103E99DA7F => end_balloonfight
0x1375F1AC6D => battle_balloonfight
0x11F42B83E2 => fzero_mutecity3ds
0x154483FC6C => end_fzero_mutecity3ds
0x1845BECB98 => battle_fzero_mutecity3ds
0xC25E2D14E0 => mario_uworld
0x10E552B305 => end_mario_uworld
0x13AE3AC517 => battle_mario_uworld
0xCC52119B70 => mario_galaxy
0x1005917BFC => end_mario_galaxy
0x134EF90DEE => battle_mario_galaxy
0xFE9F6F2750 => kart_circuitfor
0x1309932A3D => end_kart_circuitfor
0x162C3D6A72 => battle_kart_circuitfor
0xDD9A8A3D60 => zelda_skyward
0x1138668BAC => end_zelda_skyward
0x14CB949292 => battle_zelda_skyward
0xA7A44DD610 => kirby_cave
0xE740C85EF0 => end_kirby_cave
0x11D0CC4DBB => battle_kirby_cave
0xA381275860 => poke_kalos
0xE365A2D080 => end_poke_kalos
0x11929AE55C => battle_poke_kalos
0xCC79C788D0 => fe_colloseum
0x10072C1AC6 => end_fe_colloseum
0x134C446CD4 => battle_fe_colloseum
0xF3D78F3220 => icarus_angeland
0x13DD1D2B6A => end_icarus_angeland
0x16F8B36B25 => battle_icarus_angeland
0xB5D6AA3850 => wario_gamer
0xF576445FA0 => end_wario_gamer
0x123BC610DF => battle_wario_gamer
0xD3A6057420 => pikmin_garden
0x11DBAE7F38 => end_pikmin_garden
0x14285C6606 => battle_pikmin_garden
0xB6A7643000 => animal_city
0xF6078A57F0 => end_animal_city
0x120CDAF05A => battle_animal_city
0x6DAE4E3360 => wiifit
0xAAAFF88FE0 => end_wiifit
0xDFF920AC70 => battle_wiifit
0xCFD27D8710 => wreckingcrew
0x103D97BA3A => end_wreckingcrew
0x1376FFCC28 => battle_wreckingcrew
0xA232FA3A40 => pilotwings
0xE2D67FB2A0 => end_pilotwings
0x1189A7337E => battle_pilotwings
0xAFD32E8F40 => wufuisland
0xEF37AB07A0 => end_wufuisland
0x1157BA782E => battle_wufuisland
0xF09ECEE3B0 => sonic_windyhill
0x13E9893673 => end_sonic_windyhill
0x16CC27763C => battle_sonic_windyhill
0x859D426820 => pac_land
0xC8097B30F0 => end_pac_land
0xF8DA589D40 => battle_pac_land
0x9277CCE0F0 => flatzonex
0xDB4AC72070 => end_flatzonex
0x10A5A04B61 => battle_flatzonex
0x8EFEF33570 => duckhunt
0xC36ACA6DA0 => end_duckhunt
0xF3B9E9C010 => battle_duckhunt
0x9BB4B1D3C0 => sf_suzaku
0xD289BA1340 => end_sf_suzaku
0x1039979852 => battle_sf_suzaku
0xB5F0E5BD90 => mario_maker
0xF5500BDA60 => end_mario_maker
0x1239A2E883 => battle_mario_maker
0x91110225D0 => ff_midgar
0xD82C09E550 => end_ff_midgar
0x1093CCA733 => battle_ff_midgar
0xA36E06CC60 => bayo_clock
0xE38A834480 => end_bayo_clock
0x119C68FC1C => battle_bayo_clock
0xCF5BDDAEF0 => spla_parking
0x10350DB8A4 => end_spla_parking
0x137E65CEB6 => battle_spla_parking
0xE739838F80 => dracula_castle
0x12C3A96EF2 => end_dracula_castle
0x15023B8140 => battle_dracula_castle
0xBE60305500 => zelda_tower
0xFEC0DE32F0 => end_zelda_tower
0x1280AFB60A => battle_zelda_tower
0xD750BCBAD0 => mario_odyssey
0x1194C5E3D7 => end_mario_odyssey
0x146737FAE9 => battle_mario_odyssey
0x8D5128A8F0 => training
0xC5EEDCF940 => settingstage
0xB0F62F83C0 => resultstage
0x9520AB82E0 => shamfight
0xFCBB2FFA20 => spiritsroulette
0xB03D598FF0 => campaignmap
0x9A12CD0DE0 => bonusgame
0xEC1EE52520 => homeruncontest
0x9EDD2B0FD0 => staffroll
0x132805F09B => bossstage_ganonboss
0x127EE98A93 => bossstage_rathalos
0xE620A8E920 => bossstage_marx
0x1155463EC1 => bossstage_dracula
0x117FA23C16 => bossstage_galleom
0x10C3B7D901 => bossstage_final1
0x105ABE88BB => bossstage_final2
0x102DB9B82D => bossstage_final3
0x751FC41150 => sp_edit
0xA1E6851860 => photostage
0x99A20C9BD0 => teststage
0xEEF3FEB910 => jack_mementoes
0x125F0EBD9B => end_jack_mementoes
0x159E9C5229 => battle_jack_mementoes
0x101E97F18D => resultstage_jack
0xBD045831C0 => brave_altar
0xFDA4B65630 => end_brave_altar
0x12B6E93046 => battle_brave_altar
0xCD215418C0 => buddy_spiral
0x1012A523C7 => end_buddy_spiral
0x1359CD55D5 => battle_buddy_spiral
0xDA183AAE70 => dolly_stadium
0x11404D829D => end_dolly_stadium
0x14B3BF9BA3 => battle_dolly_stadium
0x9E084D3A40 => fe_shrine
0xD73546FAC0 => end_fe_shrine
0x10625856CA => battle_fe_shrine
0xDBAD6E3D10 => tantan_spring
0x115B18CBAB => end_tantan_spring
0x14A8EAD295 => battle_tantan_spring
0xD1D6744260 => battlefield_s
0xC45709B1E0 => pickel_world
0x1085C0F955 => end_pickel_world
0x13CEA88F47 => battle_pickel_world
0x735DFA0A10 => ff_cave
0xBA010F1480 => end_ff_cave
0xEFF4014C20 => battle_ff_cave
0x10C3EBCB33 => resultstage_edge
0x93C3C34A30 => xeno_alst
0xDAFEC88AB0 => end_xeno_alst
0x10BEE0B1CD => battle_xeno_alst
0xA260FA46C0 => demon_dojo
0xE2847FCE20 => end_demon_dojo
0x118C8734B6 => battle_demon_dojo
0xC923BBE460 => trail_castle
0x10528BDC0D => end_trail_castle
```

## Example Config
```toml
[bayo_clock]
valid_types = ["Normal", "End"]
hazard = "Off"

[dolly_stadium]
valid_types = ["Normal", "Battle"]
hazard = "Default"

[dracula_castle]
valid_types = ["Normal"]
hazard = "Random"

[animal_city]
valid_types = ["Normal", "Battle", "End"]
hazard = "On"
```
will make it so that the following stages will have the settings applied to them if they were picked via Random
- Umbra Clock Tower will pick only either the Normal form with Hazard being set to Off the Final Destination form 
- King of Fighters Stadium will pick only either the Normal form with the Hazard being set to whatever the ruleset has it set to or the Battlefield form
- Dracula's Castle will only pick the Normal form with a 50% chance of Hazards being on or off
- Town and City will only pick the Normal form with Hazards on, the Battlefield form, or the Final Destination form