import 'package:flutter/material.dart';
import 'package:review_tool/api.dart';

import 'provider_github.dart';
import 'provider_gitlab.dart';
import 'provider_upsource.dart';

class NewProviderDialog extends StatefulWidget {
  const NewProviderDialog({Key? key}) : super(key: key);

  @override
  State<NewProviderDialog> createState() => _NewProviderDialogState();
}

class _NewProviderDialogState extends State<NewProviderDialog> {
  final TextEditingController _nameController = TextEditingController();
  ProviderSettings? _settings;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text("Add Provider"),
      content: _settings == null ? _providerSelector() : _providerSettings(),
      actions: [
        TextButton(child: const Text("Cancel"), onPressed: () => Navigator.of(context).pop()),
        TextButton(
            child: const Text("Add"),
            onPressed: _settings == null ? null : () => Navigator.of(context).pop(_settings)),
      ],
    );
  }

  Widget _providerSelector() {
    return Column(mainAxisSize: MainAxisSize.min, children: [
      ListTile(
        title: const Text("Upsource"),
        onTap: () =>
            _initSettings(ProviderModule.upsource(UpsourceProviderSettings(url: "", token: ""))),
      ),
      ListTile(
        title: const Text("Github"),
        onTap: () =>
            _initSettings(ProviderModule.github(GithubProviderSettings(token: "", query: ""))),
      ),
      ListTile(
        title: const Text("Gitlab"),
        onTap: () => _initSettings(
            ProviderModule.gitlab(GitlabProviderSettings(url: "https://gitlab.com", token: ""))),
      ),
    ]);
  }

  Widget _providerSettings() {
    var providerSettings = _settings!.module.when(
      upsource: (settings) => EditUpsourceSettings(
          onUpdate: (s) => _updateModule(ProviderModule.upsource(s)), settings: settings),
      github: (settings) => EditGithubSettings(
          onUpdate: (s) => _updateModule(ProviderModule.github(s)), settings: settings),
      gitlab: (settings) => EditGitlabSettings(
          onUpdate: (s) => _updateModule(ProviderModule.gitlab(s)), settings: settings),
    );

    return Column(mainAxisSize: MainAxisSize.min, children: [
      TextFormField(
        decoration: const InputDecoration(labelText: "Name"),
        controller: _nameController,
        onChanged: _updateName,
      ),
      providerSettings
    ]);
  }

  _initSettings(ProviderModule module) {
    setState(() {
      _settings = ProviderSettings(id: "", name: "", module: module);
    });
  }

  _updateModule(ProviderModule module) {
    setState(() {
      _settings = ProviderSettings(id: _settings!.id, name: _settings!.name, module: module);
    });
  }

  _updateName(String name) {
    setState(() {
      _settings = ProviderSettings(id: _settings!.id, name: name, module: _settings!.module);
    });
  }
}
