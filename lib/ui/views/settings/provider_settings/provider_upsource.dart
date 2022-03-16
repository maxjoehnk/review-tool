import 'package:flutter/material.dart';
import 'package:review_tool/api.dart';

class EditUpsourceSettings extends StatefulWidget {
  final Function(UpsourceProviderSettings) onUpdate;
  final UpsourceProviderSettings settings;

  const EditUpsourceSettings({required this.onUpdate, required this.settings, Key? key})
      : super(key: key);

  @override
  State<EditUpsourceSettings> createState() => _EditUpsourceSettingsState();
}

class _EditUpsourceSettingsState extends State<EditUpsourceSettings> {
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
          var settings = UpsourceProviderSettings(url: url, token: widget.settings.token);
          widget.onUpdate(settings);
        },
      ),
      TextFormField(
        decoration: const InputDecoration(labelText: "Token"),
        controller: _tokenController,
        obscureText: true,
        onChanged: (token) {
          var settings = UpsourceProviderSettings(url: widget.settings.url, token: token);
          widget.onUpdate(settings);
        },
      ),
    ]);
  }
}
