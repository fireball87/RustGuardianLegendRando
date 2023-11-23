use crate::patcher::Patcher;
use rand::seq::SliceRandom;
struct CorridorSkyData {
    address: &'static str,
    data: &'static str,
}

fn get_sky_data() -> Vec<CorridorSkyData> {
    //if in a 3 byte section, if the 3rd byte first bit is 1 then we do another 3 byte section
    //
    //
    //first bit is what gets spawned, second bit includes length?
    //
    //on 2 byte ones, first says what spawns, second is length before next command happens
    //on 4 byte loops
    //    first bit says what spawns, then number of spawns, time between spawns, then time to next command
    //
    //    on + byte ones
    //    in every extra iteration, thing that would be time says what spawns in next iteration, then number of spawns, time between spawns and then time to next command or next iteration

    let c0 = CorridorSkyData{
        address: "101c4",
        data: "005400641C641C649C0A78649C0496649C045A649C0A78649C0A78641B641B649A0A5A641C641B64A005F0649C0A7864E20478649C0A5064020000",
    };
    let c1 = CorridorSkyData {
        address: "11d6a",
        data: "000AC20F7864C20F3C28430AC40C7828C20F7878C40A3C46C2143C64C304966EC20F2864020000",
    };
    let c2 = CorridorSkyData {
        address: "11f14",
        data: "C2061E0EC405283CE403F032E106F0C20A6446E3141E64C40A3C14E3141EE3142828C20A3232020000",
    };

    let c3 = CorridorSkyData {
        address: "123f5",
        data: "0012E6081E34E632641EE702641EA108321EE002961E1B0A1B149C0F1414E20A641E9C143264020000",
    };

    let c4 = CorridorSkyData{
        address:"125cd",
        data:"00149C321E50E21432509C0A5064D9043C14E0039614E8049632E60A463CE704823C9B033C9C1E7864E61446649C0A3264A0030A5AF6032814020000"
    };

    let c5 = CorridorSkyData {
        address: "12afa",
        data: "7B149C3C0A289B04643CEE0A3C3C9A0F2832D9083C28E30F1428EE083C289A1E28329A141432020000",
    };

    //also has a loop point, was removed see c7 for details, i have split this into 2 sections to make the loop point safe
    let c6_1 = CorridorSkyData {
        address: "12c80",
        data: "00129A0A289C141E169B033C28",
    };

    let c6_2 = CorridorSkyData {
        address: "12c8D",
        data: "9A065A1EA0061E28EE06321EED081428",
    };

    //c7 is special, not only does it have something that i don't know how to parse, it has enemies through the boss that i don't want to shuffle, therefore i have manually cut the input off early
    //there is a pointer in a 01 command to point to b18b, so i cut JUST before that point
    let c7 = CorridorSkyData {
        address: "13175",
        data: "0011E91446E6085A176728E9143C640A641EEC086428E1089628E6147850E904C850E4087864",
    };

    //c8 also hits a loop before the boss that is removed, also remove the last phase because enemies end up spawning into the boss
    let c8 = CorridorSkyData {
        address: "13337",
        data: "0014EC285A9B14F01EE21432A10A6432E7063C32C3065032",
    };

    //c9 uses a strange firt byte 02 that copies a clusterfuck of memory as it's last command, so we'll skip that
    let c9 = CorridorSkyData {
        address: "1389f",
        data: "9C280F289A141E14BE025A28E904C850A00C3C1EA10A3C50EE055A28EA047864F60F5A78BE047864",
    };

    let c10 = CorridorSkyData{address:"13ac9",
        data:"9C1E1E329B063C509A0A789C0A1464E9043CEE086478EA0828789B047878A00846329C0F3C9B0478649C051E78F60364649C0A3250BE0AC8509C1428649A066464E9043264020000"};

    let c11 = CorridorSkyData {
        address: "12077",
        data: "000AC20FB432E3083C6EE30A463CC2059664E3050A32C206B41EC306B464E102B428C2061464020000",
    };

    let c12 = CorridorSkyData{address:"12243",
        data:"420A4246434664286464C20F643CE30A7832C2023264E402783CC20A3C64C30A782800640064C20A1E64020000"};

    //shortened by one to fix enemies spawning into the boss
    let c13 = CorridorSkyData {
        address: "12797",
        data: "00149C14289C644628E2067828A006501ED902641E9C3C5A1E9B03F064D904641E9C32283C",
    };

    let c14 =  CorridorSkyData{address:"12971",
        data:"00149C0F280AE802F0329C0A5050E20AC81EE60A5A32A0051432E804F0A00A3C1EE60A3C289C1E50A10A7864020000"};

    //this does that crazy data pass of c9, but i leave it in the stream because i consider command 2 to be an end string now
    let c15 = CorridorSkyData{address:"12e06",
        data:"7B14ED101E329A0A323CE903501EEE03324B6B19ED1414A004283CED1414A1063C3C9C143CE2142D78020C35C40D2AD68E51F40000"};

    //c16 includes my fix
    let c16 = CorridorSkyData{address:"12fec",
        data:"7B146A50E0035A649B0350649C0A1E50F602F078A10A28509C0A1E32ED0A1EEE0A32786B78F6043C50A0045AE30F287FEA056464020000"};

    //this has the looping point and i will remove it
    let c17 = CorridorSkyData {
        address: "134c1",
        data: "0014EC051414E106281EC3063C28E60F3C32E705286EF6026428EC08503CE4045A50E70A3264",
    };

    let c18 = CorridorSkyData{address:"136a5",
        data:"0011A10A3C3FE904503C9C14281E9B0232647614C3043CE70378E60A649C143C64F60350149C141464020000"};

    let c19 = CorridorSkyData{address:"13d0f",
        data:"960A461E96141E1E9A04501EA0083C960F3C50EA0450E90650509C141E9B02F014EE0464646B32A10A46329C1932EE066464020000"};

    let c20 = CorridorSkyData{address:"13e32",
        data:"E90A3C329C0F5AEA085A46A10A509A0F3C50961478A00882E9043232E914C89A0FC8A004C8EE08C8649C1E5AA008C8649B04C8961E1E64020000"};

    vec![
        c0, c1, c2, c3, c4, c5, c6_1, c6_2, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17,
        c18, c19, c20,
    ]
}

fn parse_c2_length(length: &mut usize, input: &str) {
    let first = &input[*length..*length + 2];
    if 128 & u8::from_str_radix(first, 16).unwrap() != 0 {
        *length += 6;
        parse_c2_length(length, input);
    } else {
        *length += 2;
    }
}

pub fn shuffle_skies(patcher: &mut Patcher) {
    let input_data = get_sky_data();

    for corridor in input_data {
        let mut input = corridor.data;
        let mut split = Vec::new();
        while input.len() > 0 {
            let mut length = 0;
            let first = &input[0..2];
            if &input[2..6] == "00" {
                break;
            }
            if first == "02" {
                break;
            }
            if 128 & u8::from_str_radix(first, 16).unwrap() != 0 {
                length += 6;
            } else {
                length += 2;
            }
            parse_c2_length(&mut length, input);
            split.push(input[0..length].to_string());
            if first == "01" {
                panic!("Hit a loop point");
            }
            input = &input[length..];
        }
        split.shuffle(&mut rand::thread_rng());
        let patchstring = split.join("");
        patcher.add_change(&patchstring, corridor.address);
    }
}
