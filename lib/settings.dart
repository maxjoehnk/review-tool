import 'package:review_tool/api.dart';
import 'package:review_tool/ffi.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:uuid/uuid.dart';

const String defaultTheme = "Darcula";

class Settings {
  final SharedPreferences _prefs;

  Settings(this._prefs);

  static Future<Settings> load() async {
    var prefs = await SharedPreferences.getInstance();
    var settings = Settings(prefs);

    return settings;
  }

  String get theme {
    return _prefs.getString("theme") ?? defaultTheme;
  }

  Future<void> addProvider(ProviderSettings settings) async {
    var id = const Uuid().v4();
    var type = settings.module
        .map(upsource: (_) => ProviderType.upsource, github: (_) => ProviderType.github);
    var providers = _prefs.getStringList("providers") ?? [];
    providers.add(id);
    await _prefs.setInt("$id.type", type.index);
    await _setProvider(id, settings);
    await _prefs.setStringList("providers", providers);
    await _reconfigureProviders();
  }

  Future<void> editProvider(String id, ProviderSettings settings) async {
    await _setProvider(id, settings);
    await _reconfigureProviders();
  }

  Future<void> _setProvider(String id, ProviderSettings settings) async {
    await _prefs.setString("$id.name", settings.name);
    await settings.module.when(upsource: (upsource) async {
      await _prefs.setString("$id.url", upsource.url);
      await _prefs.setString("$id.token", upsource.token);
    }, github: (github) async {
      await _prefs.setString("$id.token", github.token);
      await _prefs.setString("$id.query", github.query);
    });
  }

  List<ProviderSettings> get providers {
    var providers = _prefs.getStringList("providers") ?? [];

    return providers.map((e) => _getProvider(e)).toList();
  }

  ProviderSettings _getProvider(String id) {
    int typeIndex = _prefs.getInt("$id.type")!;
    String name = _prefs.getString("$id.name")!;
    ProviderType type = ProviderType.values[typeIndex];
    ProviderModule? module;

    if (type == ProviderType.upsource) {
      module = ProviderModule.upsource(_getUpsourceProvider(id));
    } else if (type == ProviderType.github) {
      module = ProviderModule.github(_getGithubProvider(id));
    }

    return ProviderSettings(id: id, name: name, module: module!);
  }

  UpsourceProviderSettings _getUpsourceProvider(String id) {
    String url = _prefs.getString("$id.url")!;
    String token = _prefs.getString("$id.token")!;

    return UpsourceProviderSettings(
      url: url,
      token: token,
    );
  }

  GithubProviderSettings _getGithubProvider(String key) {
    String token = _prefs.getString("$key.token")!;
    String query = _prefs.getString("$key.query")!;

    return GithubProviderSettings(token: token, query: query);
  }

  Future<void> _reconfigureProviders() async {
    await api.configureModules(modules: providers);
  }
}

enum ProviderType {
  upsource,
  github,
}
