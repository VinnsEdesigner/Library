import { Command } from 'commander';
import pc from 'picocolors';

import { initCommand } from './commands/init.js';
import { themeCommand } from './commands/theme.js';
import { doctorCommand } from './commands/doctor.js';

const program = new Command();

program
  .name('vyzorix')
  .description(pc.magenta('Vyzorix Workspace Management CLI'))
  .version('1.0.0');

// Mount individual commands
program.addCommand(initCommand);
program.addCommand(themeCommand);
program.addCommand(doctorCommand);

program.parse(process.argv);
