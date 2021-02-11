// Imports
const fs = require("fs");

// Constants
const CONFIG_NAME = "trmv.config.json";
const CWD = process.cwd();
const TEST_ENDING = ".spec.lua" || config.test_ending;

// Variables
const config_path = `${CWD}\\${CONFIG_NAME}`;
const config = fs.existsSync(config_path) && require(config_path);

const source_path = `${CWD}\\${config.source || "src"}`;
const out_path = `${CWD}\\${config.out || "out"}`;

if (!fs.existsSync(source_path)) 
{
    console.error(`\n'${source_path}' does not exist`);
    process.exit(0);
} 

if (fs.existsSync(out_path))
{
    fs.rmdirSync(out_path, { recursive: true }); 
}

fs.mkdirSync(out_path);

function copy_dir_to(dir, out)
{
    const files = fs.readdirSync(dir);

    for (let file of files)
    {
        const file_path = `${dir}\\${file}`;
        const out_file_path = `${out}\\${file}`
        const stats = fs.statSync(file_path);

        if (!file.endsWith(TEST_ENDING))
        {
            if (stats.isFile())
            {
                fs.copyFileSync(file_path, out_file_path);
            }
            else
            {
                fs.mkdirSync(out_file_path);
                copy_dir_to(file_path, out_file_path);
            }
        }
    }
}

copy_dir_to(source_path, out_path);