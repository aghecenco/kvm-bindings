from glob import glob
import fileinput


BLACKLIST = [
    'kvm_enable_cap ',
    'kvm_hyperv_exit__bindgen_ty_1__bindgen_ty_2 ',
    'kvm_ioapic_state ',
    'kvm_ioeventfd ',
    'kvm_lapic_state ',
    'kvm_ppc_pvinfo ',
    'kvm_xsave ',
    'union '
]


def println(line):
    print(line, end='')


def main():
    with fileinput.input(files=glob('src/x86/bindings_*.rs'),
                         inplace=True, backup='.bak',) as src:
        for line in src:
            # Add Serialize & Deserialize where necessary.
            if line.startswith('#[derive('):
                # Skip if Serialize & Deserialize were manually added.
                if 'Serialize' in line:
                    println(line)
                    continue

                # Don't derive [De]Serialize on structs that don't support it.
                next_line = src.readline()
                ok_to_derive = True
                for not_allowed in BLACKLIST:
                    if not_allowed in next_line:
                        ok_to_derive = False
                        break
                if ok_to_derive:
                    println(line.replace(')]', ', Serialize, Deserialize)]'))
                else:
                    println(line)
                println(next_line)

            else:
                println(line)


if __name__ == '__main__':
    main()
