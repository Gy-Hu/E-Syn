# convert the circuit file to a .eqn file
def convert_to_eqn(file_name):  # sourcery skip: raise-specific-error
    if file_name.endswith('.blif'):
        return 'abc -c \"read_blif ' + file_name + '; write_blif ' + file_name[:-5] + '.blif\"'
    elif file_name.endswith('.aig'):
        return 'abc -c \"read_aiger ' + file_name + '; write_blif ' + file_name[:-4] + '.blif\"'
    elif file_name.endswith('.v'):
        return 'abc -c \"read_verilog ' + file_name + '; write_blif ' + file_name[:-2] + '.blif\"'
    else:
        raise Exception('File suffix not supported!')
    
# read the circuit file and return the circuit information as a dictionary
# name: circuit name
# inputs: input count
# ANDs: AND gate count
# NOTs: NOT gate count
# ORs: OR gate count

def circuit_info(file_name):
    info = {'name': file_name, 'inputs': 0, 'ANDs': 0, 'NOTs': 0, 'ORs': 0}
    with open(file_name, 'r') as f:
        for line in f:
            if line.startswith('.inputs'):
                info['inputs'] = len(line.split()[1:])
            elif line.startswith('.names'):
                info['ANDs'] += 1
            elif line.startswith('.subckt'):
                info['NOTs'] += 1
            elif line.startswith('.end'):
                break
    info['ORs'] = info['ANDs'] + info['NOTs']
    return info