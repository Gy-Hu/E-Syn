class CircuitParser:
    def __init__(self, input_file_path, output_file_path):
        self.input_file_path = input_file_path
        self.output_file_path = output_file_path
        self.new_n_dict = {}

    def parse_circuit(self):
        with open(self.input_file_path, 'r') as f:
            lines = f.readlines()
        
        comments = lines[0].rstrip()
        lines = lines[1:]

        output = []
        in_order = ""
        out_order = ""
        current_line = ""
        for line in lines:
            line = line.strip()
            current_line += " " + line
            if line.endswith(";"):
                if current_line.startswith(" INORDER"):
                    in_order += current_line
                elif current_line.startswith(" OUTORDER"):
                    out_order += current_line
                elif current_line.startswith(" new_n"):
                    new_n_name, new_n_expr = current_line.split(" = ")
                    self.new_n_dict[new_n_name.strip()] = new_n_expr.strip(";")
                else:
                    output.append(self.replace_new_n(current_line))
                current_line = ""

        # Add INORDER and OUTORDER lines
        output.insert(0, in_order.strip())
        output.insert(1, out_order.strip())

        for key in self.new_n_dict:
            self.new_n_dict[key] = self.replace_new_n(self.new_n_dict[key])

        output = [self.replace_new_n(line) for line in output]
        
        # insert comments in the beginning
        output.insert(0, comments)

        return "\n".join(output)

    def replace_new_n(self, expr):
        for key in self.new_n_dict:
            expr = expr.replace(key, "("+self.new_n_dict[key]+")")
        return expr

    def write_to_file(self, content):
        with open(self.output_file_path, 'w') as f:
            f.write(content)

    def process(self):
        parsed_content = self.parse_circuit()
        self.write_to_file(parsed_content)


# input_file_path = "/data/guangyuh/coding_env/E-Brush/test_data/raw_circuit.txt"
# output_file_path = "/data/guangyuh/coding_env/E-Brush/test_data/original_circuit.txt"

# parser = CircuitParser(input_file_path, output_file_path)
# parser.process()