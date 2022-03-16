import 'package:flutter/material.dart';
import 'package:review_tool/api.dart';

class EditGithubSettings extends StatefulWidget {
  final Function(GithubProviderSettings) onUpdate;
  final GithubProviderSettings settings;

  const EditGithubSettings({required this.settings, required this.onUpdate, Key? key})
      : super(key: key);

  @override
  State<EditGithubSettings> createState() => _EditGithubSettingsState();
}

class _EditGithubSettingsState extends State<EditGithubSettings> {
  final TextEditingController _tokenController = TextEditingController();
  final TextEditingController _queryController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _tokenController.text = widget.settings.token;
    _queryController.text = widget.settings.query;
  }

  @override
  Widget build(BuildContext context) {
    return Column(mainAxisSize: MainAxisSize.min, children: [
      TextFormField(
        decoration: const InputDecoration(labelText: "Token"),
        obscureText: true,
        controller: _tokenController,
        onChanged: (token) {
          var settings = GithubProviderSettings(token: token, query: widget.settings.query);
          widget.onUpdate(settings);
        },
      ),
      TextFormField(
        decoration: const InputDecoration(labelText: "Query"),
        controller: _queryController,
        onChanged: (query) {
          var settings = GithubProviderSettings(query: query, token: widget.settings.token);
          widget.onUpdate(settings);
        },
      ),
    ]);
  }
}
