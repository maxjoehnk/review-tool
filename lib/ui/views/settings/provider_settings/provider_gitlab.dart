import 'package:flutter/material.dart';
import 'package:review_tool/api.dart';

class EditGitlabSettings extends StatefulWidget {
  final Function(GitlabProviderSettings) onUpdate;
  final GitlabProviderSettings settings;

  const EditGitlabSettings({required this.settings, required this.onUpdate, Key? key})
      : super(key: key);

  @override
  State<EditGitlabSettings> createState() => _EditGitlabSettingsState();
}

class _EditGitlabSettingsState extends State<EditGitlabSettings> {
  final TextEditingController _urlController = TextEditingController();
  final TextEditingController _tokenController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _urlController.text = widget.settings.url;
    _tokenController.text = widget.settings.token;
  }

  @override
  Widget build(BuildContext context) {
    return Column(mainAxisSize: MainAxisSize.min, children: [
      TextFormField(
        decoration: const InputDecoration(labelText: "URL"),
        controller: _urlController,
        onChanged: (url) {
          var settings = GitlabProviderSettings(url: url, token: widget.settings.token);
          widget.onUpdate(settings);
        },
      ),
      TextFormField(
        decoration: const InputDecoration(labelText: "Token"),
        obscureText: true,
        controller: _tokenController,
        onChanged: (token) {
          var settings = GitlabProviderSettings(token: token, url: widget.settings.url);
          widget.onUpdate(settings);
        },
      ),
    ]);
  }
}
