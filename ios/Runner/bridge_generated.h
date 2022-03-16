#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_UpsourceProviderSettings {
  struct wire_uint_8_list *url;
  struct wire_uint_8_list *token;
} wire_UpsourceProviderSettings;

typedef struct ProviderModule_Upsource {
  struct wire_UpsourceProviderSettings *field0;
} ProviderModule_Upsource;

typedef struct wire_GithubProviderSettings {
  struct wire_uint_8_list *token;
  struct wire_uint_8_list *query;
} wire_GithubProviderSettings;

typedef struct ProviderModule_Github {
  struct wire_GithubProviderSettings *field0;
} ProviderModule_Github;

typedef union ProviderModuleKind {
  struct ProviderModule_Upsource *Upsource;
  struct ProviderModule_Github *Github;
} ProviderModuleKind;

typedef struct wire_ProviderModule {
  int32_t tag;
  union ProviderModuleKind *kind;
} wire_ProviderModule;

typedef struct wire_ProviderSettings {
  struct wire_uint_8_list *id;
  struct wire_uint_8_list *name;
  struct wire_ProviderModule *module;
} wire_ProviderSettings;

typedef struct wire_list_provider_settings {
  struct wire_ProviderSettings *ptr;
  int32_t len;
} wire_list_provider_settings;

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

void wire_get_reviews(int64_t port_, struct wire_uint_8_list *provider_id);

void wire_get_review_discussions(int64_t port_,
                                 struct wire_uint_8_list *provider_id,
                                 struct wire_uint_8_list *review_id);

void wire_get_review_file_summaries(int64_t port_,
                                    struct wire_uint_8_list *provider_id,
                                    struct wire_uint_8_list *review_id);

void wire_get_review_file(int64_t port_,
                          struct wire_uint_8_list *provider_id,
                          struct wire_uint_8_list *review_id,
                          struct wire_uint_8_list *file_path,
                          struct wire_uint_8_list *revision);

void wire_mark_file_read(int64_t port_,
                         struct wire_uint_8_list *provider_id,
                         struct wire_uint_8_list *review_id,
                         struct wire_uint_8_list *file_path,
                         struct wire_uint_8_list *revision,
                         bool read);

void wire_configure_modules(int64_t port_, struct wire_list_provider_settings *modules);

struct wire_GithubProviderSettings *new_box_autoadd_github_provider_settings(void);

struct wire_UpsourceProviderSettings *new_box_autoadd_upsource_provider_settings(void);

struct wire_ProviderModule *new_box_provider_module(void);

struct wire_list_provider_settings *new_list_provider_settings(int32_t len);

struct wire_uint_8_list *new_uint_8_list(int32_t len);

union ProviderModuleKind *inflate_ProviderModule_Upsource(void);

union ProviderModuleKind *inflate_ProviderModule_Github(void);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_get_reviews);
    dummy_var ^= ((int64_t) (void*) wire_get_review_discussions);
    dummy_var ^= ((int64_t) (void*) wire_get_review_file_summaries);
    dummy_var ^= ((int64_t) (void*) wire_get_review_file);
    dummy_var ^= ((int64_t) (void*) wire_mark_file_read);
    dummy_var ^= ((int64_t) (void*) wire_configure_modules);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_github_provider_settings);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_upsource_provider_settings);
    dummy_var ^= ((int64_t) (void*) new_box_provider_module);
    dummy_var ^= ((int64_t) (void*) new_list_provider_settings);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list);
    dummy_var ^= ((int64_t) (void*) inflate_ProviderModule_Upsource);
    dummy_var ^= ((int64_t) (void*) inflate_ProviderModule_Github);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}