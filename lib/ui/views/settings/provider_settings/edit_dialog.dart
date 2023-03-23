import 'package:flutter/material.dart';
import 'package:review_tool/api.dart';
import 'package:review_tool/ui/views/settings/provider_settings/provider_gitlab.dart';

import 'provider_github.dart';
import 'provider_upsource.dart';

class EditProviderDialog extends StatefulWidget {
  final ProviderSettings settings;

  const EditProviderDialog(this.settings, {Key? key}) : super(key: key);

  @override
  State<EditProviderDialog> createState() => _EditProviderDialogState();
}

class _EditProviderDialogState extends State<EditProviderDialog> {
  final TextEditingController _nameController = TextEditingController();
  late ProviderSettings _settings;

  @override
  void initState() {
    super.initState();
    _settings = widget.settings;
    _nameController.text = _settings.name;
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text("Edit Provider"),
      content: _providerSettings(),
      actions: [
        TextButton(child: const Text("Cancel"), onPressed: () => Navigator.of(context).pop()),
        TextButton(
            child: const Text("Save"), onPressed: () => Navigator.of(context).pop(_settings)),
      ],
    );
  }

  Widget _providerSettings() {
    var providerSettings = _settings.module.when(
        upsource: (settings) => EditUpsourceSettings(
            onUpdate: (s) => _updateModule(ProviderModule.upsource(s)), settings: settings),
        github: (settings) => EditGithubSettings(
            onUpdate: (s) => _updateModule(ProviderModule.github(s)), settings: settings),
        gitlab: (settings) => EditGitlabSettings(
            settings: settings, onUpdate: (s) => _updateModule(ProviderModule.gitlab(s))));

    return Column(mainAxisSize: MainAxisSize.min, children: [
      TextFormField(
        decoration: const InputDecoration(labelText: "Name"),
        controller: _nameController,
        onChanged: _updateName,
      ),
      providerSettings
    ]);
  }

  _updateModule(ProviderModule module) {
    setState(() {
      _settings = ProviderSettings(id: _settings.id, name: _settings.name, module: module);
    });
  }

  _updateName(String name) {
    setState(() {
      _settings = ProviderSettings(id: _settings.id, name: name, module: _settings.module);
    });
  }
}
