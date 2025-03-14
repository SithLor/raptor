//ts decorators for checking runtime types

/**
 * Formats a string with replacements similar to Rust's format! macro
 * Supports simple replacements using {} and named/positional arguments
 */
function format(template: string, ...args: any[]): string {
    let result = template;

    // Handle simple replacements like "{}"
    if (template.includes('{}')) {
        let argIndex = 0;
        result = template.replace(/{}/g, () => {
            const replacement = args[argIndex];
            argIndex++;
            return String(replacement);
        });
    }
    // Handle numbered replacements like "{0}", "{1}", etc.
    else if (template.match(/{[0-9]+}/g)) {
        result = template.replace(/{([0-9]+)}/g, (_, index) => {
            const idx = parseInt(index, 10);
            if (idx >= args.length) {
                throw new Error(`Index ${idx} out of bounds for format arguments`);
            }
            return String(args[idx]);
        });
    }
    // Handle named replacements from an object
    else if (template.match(/{[a-zA-Z_][a-zA-Z0-9_]*}/g) && args.length === 1 && typeof args[0] === 'object') {
        const params = args[0];
        result = template.replace(/{([a-zA-Z_][a-zA-Z0-9_]*)}/g, (_, key) => {
            if (!(key in params)) {
                throw new Error(`Key "${key}" not found in format parameters`);
            }
            return String(params[key]);
        });
    }

    return result;
}

// Type definition for the format function
type FormatFn = {
    (template: string, ...args: any[]): string;
    (template: string, params: Record<string, any>): string;
}

// ...existing code...
function main() {
    // Examples of using format
    console.log(format("Hello, {}!", "world"));
    console.log(format("The answer is {0} and {1}", 42, "life"));
    console.log(format("Hello, {name}!", { name: "TypeScript" }));

    // test match function
    const match: MatchFn<"Ok", number, string, string> = (result) => {
        switch (result.tag) {
            case "Ok":
                return `Success: ${result.value}`;
            case "Err":
                return `Error: ${result.error}`;
        }
    };
    console.log("Hello World");
}