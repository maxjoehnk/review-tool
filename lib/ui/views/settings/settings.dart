import 'package:flutter/material.dart';
import 'package:review_tool/settings.dart';

import 'settings_overview.dart';

class SettingsScreen extends StatefulWidget {
  const SettingsScreen({Key? key}) : super(key: key);

  @override
  State<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen> {
  @override
  Widget build(BuildContext context) {
    return FutureBuilder<Settings>(
        future: Settings.load(),
        builder: (context, state) {
          if (!state.hasData) {
            return Container();
          }

          return SettingsOverviewView(state.data!, onUpdate: () => setState(() {}));
        });
  }
}
