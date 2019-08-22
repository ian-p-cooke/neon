{
    "targets": [{
        "target_name": "neon",
        "win_delay_load_hook": "true",
        "sources": [ "src/neon.cc" ],
        "include_dirs": [ "<!(node -e \"require('nan')\")" ],
        'configurations': {
            'Release': {
                'msvs_settings': {
                    'VCCLCompilerTool': {
                        'WholeProgramOptimization': 'false'
                    },
                    'VCLinkerTool': {
                        'LinkTimeCodeGeneration': 0
                    }
                }
            }
        }
    }]
}
