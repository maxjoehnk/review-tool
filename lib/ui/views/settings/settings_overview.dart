import 'package:flutter/material.dart';
import 'package:review_tool/api.dart';
import 'package:review_tool/settings.dart';
import 'package:settings_ui/settings_ui.dart';

import 'provider_settings/edit_dialog.dart';
import 'provider_settings/new_dialog.dart';

class SettingsOverviewView extends StatelessWidget {
  final Settings settings;
  final Function() onUpdate;

  const SettingsOverviewView(this.settings, {required this.onUpdate, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SettingsList(
      sections: [
        SettingsSection(
          title: const Text("Providers"),
          tiles: [
            ...settings.providers.map((provider) => SettingsTile.navigation(
                title: Text(provider.name),
                description: Text(provider.title),
                trailing: const Icon(Icons.edit),
                onPressed: (context) => onEditProvider(context, provider),
                value: Text(provider.module.when(
                    upsource: (upsource) => upsource.url,
                    github: (github) => github.query,
                    gitlab: (gitlab) => gitlab.url)))),
            SettingsTile(
              title: const Text("Add Provider", style: TextStyle(color: Colors.white54)),
              onPressed: (context) => onAddProvider(context),
              trailing: const Icon(Icons.add, color: Colors.white54),
            )
          ],
        ),
        SettingsSection(title: const Text("Code Viewer"), tiles: [
          SettingsTile(
            title: const Text("Theme"),
            value: Text(settings.theme),
          )
        ])
      ],
    );
  }

  onAddProvider(BuildContext context) async {
    ProviderSettings? provider =
        await showDialog(context: context, builder: (context) => const NewProviderDialog());
    if (provider == null) {
      return;
    }
    await settings.addProvider(provider);
    onUpdate();
  }

  onEditProvider(BuildContext context, ProviderSettings settings) async {
    ProviderSettings? provider =
        await showDialog(context: context, builder: (context) => EditProviderDialog(settings));
    if (provider == null) {
      return;
    }
    await this.settings.editProvider(settings.id, provider);
    onUpdate();
  }
}
