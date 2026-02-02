import { ExecTask, TaskOfTasks } from '@flybywiresim/igniter';

export default new TaskOfTasks(
    'horizon',
    [
        new TaskOfTasks(
            'test',
            [new ExecTask('lint', 'npm run lint'), new ExecTask('prettier', 'npm run prettier')],
            true
        ),
        new TaskOfTasks(
            'build',
            [
                new ExecTask('compile', 'tsc -p ./tsconfig.json'),
                new ExecTask('bundle', 'node ./scripts/bundle.js'),
                new ExecTask('build', './scripts/build.sh'),
            ],
            false
        ),
    ],
    false
);
